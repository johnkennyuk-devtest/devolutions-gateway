use windows::core::{Owned, PCSTR};
use windows::Win32::Foundation::{
    DuplicateHandle, GetLastError, DUPLICATE_SAME_ACCESS, ERROR_IO_PENDING, HANDLE, WAIT_EVENT, WAIT_OBJECT_0,
};
use windows::Win32::Storage::FileSystem::{ReadFile, WriteFile};
use windows::Win32::System::RemoteDesktop::{
    WTSFreeMemory, WTSVirtualChannelClose, WTSVirtualChannelOpenEx, WTSVirtualChannelQuery, WTSVirtualFileHandle,
    CHANNEL_CHUNK_LENGTH, CHANNEL_PDU_HEADER, WTS_CHANNEL_OPTION_DYNAMIC, WTS_CURRENT_SESSION,
};
use windows::Win32::System::Threading::{CreateEventW, GetCurrentProcess, SetEvent, WaitForMultipleObjects, INFINITE};
use windows::Win32::System::IO::{GetOverlappedResult, OVERLAPPED};

use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::dvc::now_message_dissector::NowMessageDissector;
use now_proto_pdu::NowMessage;
use tokio::sync::mpsc::error::TrySendError;

const WAIT_OBJECT_1: WAIT_EVENT = WAIT_EVENT(WAIT_OBJECT_0.0 + 1);
const WAIT_OBJECT_2: WAIT_EVENT = WAIT_EVENT(WAIT_OBJECT_0.0 + 2);

pub type DvcIoReadTx = Sender<NowMessage>;
pub type DvcIoReadRx = Receiver<NowMessage>;

const DVC_CHANNEL_NAME: &str = "Devolutions::Now::Agent";
const DVC_IO_CHANNEL_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct DvcIoWriteTx {
    tx: Sender<NowMessage>,
    event: WinApiEvent,
}

impl DvcIoWriteTx {
    pub async fn send(&self, message: NowMessage) -> anyhow::Result<()> {
        self.tx.send(message).await?;

        // DVC IO loop is controlled by WinAPI events signaling, therefore we need to fire event to
        // notify DVC IO loop about new incomming message.

        // SAFETY: No preconditions.
        unsafe {
            SetEvent(self.event.raw_handle())?;
        }
        Ok(())
    }

    pub fn blocking_send(&self, message: NowMessage) -> anyhow::Result<()> {
        self.tx.blocking_send(message)?;

        // SAFETY: No preconditions.
        unsafe {
            SetEvent(self.event.raw_handle())?;
        }
        Ok(())
    }
}

pub struct DvcIoWriteRx {
    rx: Receiver<NowMessage>,
    event: WinApiEvent,
}

impl DvcIoWriteRx {
    pub fn try_recv(&mut self) -> anyhow::Result<NowMessage> {
        let value = self.rx.try_recv()?;
        Ok(value)
    }

    pub fn raw_event(&self) -> HANDLE {
        self.event.raw_handle()
    }
}

pub fn dvc_io_write_channel() -> anyhow::Result<(DvcIoWriteTx, DvcIoWriteRx)> {
    // Create WinAPI event.

    let event = WinApiEvent::new_unnamed()?;

    let (tx, rx) = mpsc::channel(DVC_IO_CHANNEL_SIZE);

    Ok((
        DvcIoWriteTx {
            tx,
            event: event.clone(),
        },
        DvcIoWriteRx { rx, event },
    ))
}

pub fn dvc_io_read_channel() -> anyhow::Result<(DvcIoReadTx, DvcIoReadRx)> {
    let (tx, rx) = mpsc::channel(DVC_IO_CHANNEL_SIZE);

    Ok((tx, rx))
}

#[derive(Debug, Clone)]
pub struct WinApiEvent {
    handle: Arc<Owned<HANDLE>>,
}

// SAFETY: WinAPI's event handles are thread-safe and can be sent between threads.
unsafe impl Send for WinApiEvent {}

// SAFETY: WinAPI's event handles are thread-safe and can be sent between threads.
unsafe impl Sync for WinApiEvent {}

/// RAII wrapper for WinAPI event handle.
impl WinApiEvent {
    pub fn new_unnamed() -> anyhow::Result<Self> {
        // SAFETY: No preconditions.
        let raw_handle = unsafe { CreateEventW(None, false, false, None) }?;

        // SAFETY: No preconditions.
        let owned_event = unsafe { Owned::new(raw_handle) };

        // NOTE: Although HANDLE is not `Send` nor `Sync`, it is safe to wrap it into `Arc` because
        // WinAPI events are thread-safe and can be sent between threads.
        #[allow(clippy::arc_with_non_send_sync)]
        Ok(Self {
            handle: Arc::new(owned_event),
        })
    }

    pub fn raw_handle(&self) -> HANDLE {
        **self.handle
    }

    pub fn set(&self) -> anyhow::Result<()> {
        // SAFETY: No preconditions.
        unsafe {
            SetEvent(**self.handle)?;
        }
        Ok(())
    }
}

/// RAII wrapper for WTS virtual channel handle.
struct WTSVirtualChannel(HANDLE);

impl Drop for WTSVirtualChannel {
    fn drop(&mut self) {
        // SAFETY: `Ok` value returned from `WTSVirtualChannelOpenEx` is always a valid handle.
        if let Err(error) = unsafe { WTSVirtualChannelClose(self.0) } {
            error!(%error, "Failed to close WTS virtual channel handle");
        }
    }
}

/// RAII wrapper for WTS memory handle.
struct WTSMemory(*mut core::ffi::c_void);

impl WTSMemory {
    pub(crate) fn as_handle(&self) -> HANDLE {
        if self.0.is_null() {
            return HANDLE::default();
        }

        // SAFETY: `self.0` is always a valid pointer to a handle if constructed properly,
        // therefore it is safe to dereference it.
        HANDLE(unsafe { *(self.0 as *mut *mut std::ffi::c_void) })
    }
}

impl Drop for WTSMemory {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }

        // SAFETY: No preconditions.
        unsafe { WTSFreeMemory(self.0) }
    }
}

impl Default for WTSMemory {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

pub fn run_dvc_io(
    mut write_rx: DvcIoWriteRx,
    read_tx: Sender<NowMessage>,
    stop_event: WinApiEvent,
) -> Result<(), anyhow::Error> {
    let channel_file = open_agent_dvc_channel_impl()?;

    let mut pdu_chunk_buffer = [0u8; CHANNEL_CHUNK_LENGTH as usize];
    let mut overlapped = OVERLAPPED::default();
    let mut bytes_read: u32 = 0;

    let mut message_dissector = NowMessageDissector::default();

    let read_event = WinApiEvent::new_unnamed()?;
    overlapped.hEvent = read_event.raw_handle();

    info!("DVC IO thread is running");

    // Prepare async read operation.
    // SAFETY: Both `channel_file` and event passed to `overlapped` are valid during this call,
    // therefore it is safe to call.
    let read_result: Result<(), windows::core::Error> =
        unsafe { ReadFile(*channel_file, Some(&mut pdu_chunk_buffer), None, Some(&mut overlapped)) };

    ensure_overlapped_io_result(read_result)?;

    loop {
        let events = [read_event.raw_handle(), write_rx.raw_event(), stop_event.raw_handle()];

        // SAFETY: No preconditions.
        let wait_status = unsafe { WaitForMultipleObjects(&events, false, INFINITE) };

        match wait_status {
            // Read event is signaled (incomming data from DVC channel).
            WAIT_OBJECT_0 => {
                trace!("DVC channel read event is signaled");

                // SAFETY: No preconditions.
                unsafe { GetOverlappedResult(*channel_file, &overlapped, &mut bytes_read, false) }?;

                if bytes_read
                    < u32::try_from(std::mem::size_of::<CHANNEL_PDU_HEADER>())
                        .expect("BUG CHANNEL_PDU_HEADER size always fits into u32")
                {
                    // Channel is closed abruptly; abort loop.
                    return Ok(());
                }

                let chunk_data_size = usize::try_from(bytes_read)
                    .expect(
                        "BUG: Read size can't be breater than CHANNEL_CHUNK_LENGTH, therefore it should fit into usize",
                    )
                    .checked_sub(std::mem::size_of::<CHANNEL_PDU_HEADER>())
                    .expect("BUG: Read size is less than header size; Corectness of this should be ensured by the OS");

                const HEADER_SIZE: usize = std::mem::size_of::<CHANNEL_PDU_HEADER>();

                let messages = message_dissector
                    .dissect(&pdu_chunk_buffer[HEADER_SIZE..HEADER_SIZE + chunk_data_size])
                    .expect("BUG: Failed to dissect messages");

                // Send all messages over the channel.
                for message in messages {
                    debug!(?message, "DVC message received");
                    // We do non-blocking send to avoid blocking the IO thread. Processing
                    // task is expected to be fast enough to keep up with the incoming messages.
                    match read_tx.try_send(message) {
                        Ok(_) => {
                            trace!("Received DVC message is sent to the processing channel");
                        }
                        Err(TrySendError::Full(_)) => {
                            trace!("DVC message is dropped due to busy channel");
                        }
                        Err(e) => {
                            trace!("DVC message is dropped due to closed channel");
                            return Err(e.into());
                        }
                    }
                }

                // Prepare async read file operation one more time.
                // SAFETY: No preconditions.
                let result =
                    unsafe { ReadFile(*channel_file, Some(&mut pdu_chunk_buffer), None, Some(&mut overlapped)) };

                ensure_overlapped_io_result(result)?;
            }
            // Write event is signaled (outgoing data to DVC channel).
            WAIT_OBJECT_1 => {
                trace!("DVC channel write event is signaled");

                let message_to_write = write_rx.try_recv()?;
                let message_bytes = ironrdp::core::encode_vec(&message_to_write)?;

                let mut dw_written: u32 = 0;

                // SAFETY: No preconditions.
                unsafe { WriteFile(*channel_file, Some(&message_bytes), Some(&mut dw_written), None)? }
            }
            WAIT_OBJECT_2 => {
                info!("DVC IO thread is stopped");
                // Stop event is signaled; abort loop.
                return Ok(());
            }
            _ => {
                // Spurious wakeup
            }
        };
    }
}

fn open_agent_dvc_channel_impl() -> anyhow::Result<Owned<HANDLE>> {
    let channel_name_wide = PCSTR::from_raw(DVC_CHANNEL_NAME.as_ptr());

    trace!("Opening DVC channel");

    // SAFETY: No preconditions.
    let wts_handle = WTSVirtualChannel(unsafe {
        WTSVirtualChannelOpenEx(WTS_CURRENT_SESSION, channel_name_wide, WTS_CHANNEL_OPTION_DYNAMIC)
    }?);

    let mut channel_file_handle_ptr = WTSMemory::default();
    let mut len: u32 = 0;

    trace!("Querying DVC channel");

    // SAFETY: It is safe to call `WTSVirtualChannelQuery` with valid channel and
    // destination pointers.
    unsafe {
        WTSVirtualChannelQuery(
            wts_handle.0,
            WTSVirtualFileHandle,
            &mut channel_file_handle_ptr.0 as *mut _,
            &mut len,
        )
    }?;

    if len != u32::try_from(std::mem::size_of::<HANDLE>()).expect("HANDLE always fits into u32") {
        return Err(anyhow::anyhow!("Failed to query DVC channel file handle"));
    }

    let mut raw_handle = HANDLE::default();

    // SAFETY: `GetCurrentProcess` is always safe to call.
    let current_process = unsafe { GetCurrentProcess() };

    // SAFETY: `lptargetprocesshandle` is valid and points to `raw_handle` declared above,
    // therefore it is safe to call.
    unsafe {
        DuplicateHandle(
            current_process,
            channel_file_handle_ptr.as_handle(),
            current_process,
            &mut raw_handle,
            0,
            false,
            DUPLICATE_SAME_ACCESS,
        )?;
    };

    info!("DVC channel opened");

    // SAFETY: `DuplicateHandle` is always safe to call.
    let new_handle = unsafe { Owned::new(raw_handle) };

    Ok(new_handle)
}

fn ensure_overlapped_io_result(result: windows::core::Result<()>) -> Result<(), anyhow::Error> {
    if let Err(e) = result {
        // SAFETY: GetLastError is alwasy safe to call
        if unsafe { GetLastError() } != ERROR_IO_PENDING {
            return Err(e.into());
        }
    }

    Ok(())
}
