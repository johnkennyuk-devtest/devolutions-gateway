use super::io::{dvc_io_read_channel, run_dvc_io, WinApiEvent};
use super::io::{dvc_io_write_channel, DvcIoReadRx, DvcIoWriteTx};
use async_trait::async_trait;
use devolutions_gateway_task::Task;
use now_proto_pdu::{
    NowExecCapsetFlags, NowExecCapsetMsg, NowExecMessage, NowExecResultMsg, NowExecRunMsg, NowMessage,
    NowMsgBoxResponse, NowSessionMessage, NowSessionMsgBoxReqMsg, NowSessionMsgBoxRspMsg, NowSeverity, NowStatus,
    NowStatusCode, NowSystemMessage,
};
use tokio::select;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::System::Shutdown::{ExitWindowsEx, LockWorkStation, EWX_LOGOFF, SHUTDOWN_REASON};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MESSAGEBOX_STYLE, SW_RESTORE};

#[derive(Default)]
pub struct DvcIoTask {}

#[async_trait]
impl Task for DvcIoTask {
    type Output = anyhow::Result<()>;

    const NAME: &'static str = "DVC processing";

    async fn run(self, shutdown_signal: devolutions_gateway_task::ShutdownSignal) -> Self::Output {
        let (write_tx, write_rx) = dvc_io_write_channel()?;
        let (read_tx, read_rx) = dvc_io_read_channel()?;

        // WinAPI event to terminate DVC IO thread
        let io_thread_shutdown_event = WinApiEvent::new_unnamed()?;

        let cloned_shutdown_event = io_thread_shutdown_event.clone();

        // Spawning thread is relatively short operation, so it could be executed synchronously
        let io_thread = std::thread::spawn(move || {
            let io_thread_result = run_dvc_io(write_rx, read_tx, cloned_shutdown_event);

            if let Err(error) = io_thread_result {
                error!(%error, "DVC IO thread failed");
            }
        });

        // Join thread some time in future
        tokio::task::spawn_blocking(move || {
            if io_thread.join().is_err() {
                error!("DVC IO thread join failed");
            };
        });

        info!("Processing DVC messages...");

        let process_result = process_messages(read_rx, write_tx, shutdown_signal).await;

        // Send shutdown signal to IO thread to release WTS channel resources
        info!("Shutting down DVC IO thread");
        let _ = io_thread_shutdown_event.set();

        process_result?;

        Ok(())
    }
}

async fn process_messages(
    mut read_rx: DvcIoReadRx,
    write_tx: DvcIoWriteTx,
    mut shutdown_signal: devolutions_gateway_task::ShutdownSignal,
) -> anyhow::Result<()> {
    let mut processor = MessageProcessor::new(write_tx);

    processor.send_initialization_sequence().await?;

    loop {
        select! {
            read_result = read_rx.recv() => {
                match read_result {
                    Some(message) => {
                        match processor.process_message(message).await {
                            Ok(()) => {}
                            Err(error) => {
                                error!(%error, "Failed to process DVC message");
                                return Err(error);
                            }
                        }
                    }
                    None => {
                        return Err(anyhow::anyhow!("Read channel has been closed"));
                    }
                }
            }

            _ = shutdown_signal.wait() => {
                info!("DVC task was cancelled");
                return Ok(());
            }
        }
    }
}

struct MessageProcessor {
    tx: DvcIoWriteTx,
    downgraded_caps: NowExecCapsetFlags,
}

impl MessageProcessor {
    pub(crate) fn new(tx: DvcIoWriteTx) -> Self {
        Self {
            tx,
            // Caps are empty until negotiated
            downgraded_caps: NowExecCapsetFlags::empty(),
        }
    }

    pub(crate) async fn send_initialization_sequence(&self) -> anyhow::Result<()> {
        // Caps supported by the server
        let capabilities_pdu = NowMessage::from(NowExecCapsetMsg::new(NowExecCapsetFlags::STYLE_RUN));

        self.tx.send(capabilities_pdu).await?;

        Ok(())
    }

    pub(crate) async fn process_message(&mut self, message: NowMessage) -> anyhow::Result<()> {
        match message {
            NowMessage::Exec(NowExecMessage::Capset(client_capset_message)) => {
                // Execute downgrade caps sequence
                let server_flags = NowExecCapsetFlags::STYLE_RUN;
                let downgraded_flags = server_flags & client_capset_message.flags();
                self.downgraded_caps = downgraded_flags;

                let downgraded_caps_pdu = NowMessage::from(NowExecCapsetMsg::new(downgraded_flags));

                self.tx.send(downgraded_caps_pdu).await?;
            }
            NowMessage::Exec(NowExecMessage::Run(params)) => {
                // Execute synchronously; ShellExecute will not block the calling thread,
                // For "Run" we are only interested in fire-and-forget execution
                process_exec_run(params, self.tx.clone()).await?;
            }
            NowMessage::Session(NowSessionMessage::MsgBoxReq(request)) => {
                let tx = self.tx.clone();

                // Spawn blocking task for message box to avoid blocking the IO loop
                let join_handle = tokio::task::spawn_blocking(move || process_msg_box_req(request, tx));
                drop(join_handle);
            }
            NowMessage::Session(NowSessionMessage::Logoff(_logoff_msg)) => {
                // SAFETY: `ExitWindowsEx` is always safe to call
                if let Err(error) = unsafe { ExitWindowsEx(EWX_LOGOFF, SHUTDOWN_REASON(0)) } {
                    error!(%error, "Failed to logoff user session");
                }
            }
            NowMessage::Session(NowSessionMessage::Lock(_lock_msg)) => {
                // SAFETY: `LockWorkStation` is always safe to call
                if let Err(error) = unsafe { LockWorkStation() } {
                    error!(%error, "Failed to lock workstation");
                }
            }
            NowMessage::System(NowSystemMessage::Shutdown(_shutdown_msg)) => {
                // TODO: Adjust `NowSession` token privileges in NowAgent to make shutdown possible
                // from this process
            }
            _ => {
                warn!("Unsupported message: {:?}", message);
            }
        }

        Ok(())
    }
}

async fn process_exec_run(params: NowExecRunMsg, tx: DvcIoWriteTx) -> anyhow::Result<()> {
    let command = HSTRING::from(params.command().value());

    // Empty null-terminated string
    let parameters: [u16; 1] = [0x0000];

    let operation = HSTRING::from("open");

    // SAFETY: All buffers are valid, therefore `ShellExecuteW` is safe to call.
    let hinstance = unsafe {
        ShellExecuteW(
            None,
            PCWSTR(operation.as_ptr()),
            PCWSTR(command.as_ptr()),
            PCWSTR(parameters.as_ptr()),
            None,
            // Activate and show window
            SW_RESTORE,
        )
    };

    if hinstance.0 as usize <= 32 {
        error!("ShellExecuteW failed, error code: {}", hinstance.0 as usize);
        // Send error status

        // TODO:
        // - Better error handling.
        // - Improve protocol to send an actual application return code (?)
        tx.send(NowMessage::from(NowExecResultMsg::new(
            params.session_id(),
            NowStatus::new(NowSeverity::Error, NowStatusCode::FAILURE),
        )))
        .await?;
    } else {
        tx.send(NowMessage::from(NowExecResultMsg::new(
            params.session_id(),
            NowStatus::new(NowSeverity::Info, NowStatusCode::SUCCESS),
        )))
        .await?;
    }

    Ok(())
}

fn process_msg_box_req(request: NowSessionMsgBoxReqMsg, tx: DvcIoWriteTx) {
    info!("Processing message box request `{}`", request.request_id());

    let title = HSTRING::from(
        request
            .title()
            .map(|varstr| varstr.value())
            .unwrap_or("Devolutions Agent"),
    );

    let text = HSTRING::from(request.message().value());

    // TODO: Use undocumented `MessageBoxTimeout` instead
    // or create custom window (?)
    // SAFETY: `MessageBoxW` is always safe to call
    let result = unsafe {
        MessageBoxW(
            None,
            PCWSTR(text.as_ptr()),
            PCWSTR(title.as_ptr()),
            MESSAGEBOX_STYLE(request.style().value()),
        )
    };

    #[allow(clippy::cast_sign_loss)]
    let message_box_response = result.0 as u32;

    let send_result = tx.blocking_send(NowMessage::from(NowSessionMsgBoxRspMsg::new(
        request.request_id(),
        NowMsgBoxResponse::new(message_box_response),
    )));

    if let Err(error) = send_result {
        error!(%error, "Failed to send MessageBox response");
    }
}
