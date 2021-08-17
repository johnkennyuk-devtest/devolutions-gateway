use anyhow::{anyhow, ensure};
use byteorder::{BigEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::{Cursor, Read};

pub trait Marshall {
    fn marshall(&self) -> Vec<u8>;
}

pub trait Unmarshall {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error>
    where
        Self: Sized;

    fn get_size_of_fixed_part() -> usize;
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum JmuxChannelMessageType {
    Open = 100,
    OpenSuccess,
    OpenFailure,
    WindowAdjust,
    Data,
    Eof,
    Close,
}

impl TryFrom<u8> for JmuxChannelMessageType {
    type Error = anyhow::Error;
    fn try_from(val: u8) -> Result<JmuxChannelMessageType, anyhow::Error> {
        match val {
            100 => Ok(JmuxChannelMessageType::Open),
            101 => Ok(JmuxChannelMessageType::OpenSuccess),
            102 => Ok(JmuxChannelMessageType::OpenFailure),
            103 => Ok(JmuxChannelMessageType::WindowAdjust),
            104 => Ok(JmuxChannelMessageType::Data),
            105 => Ok(JmuxChannelMessageType::Eof),
            106 => Ok(JmuxChannelMessageType::Close),
            _ => Err(anyhow!("Incorrect JMUXChannelMessageType value: {}", val)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CommonDefinitions {
    pub msg_type: JmuxChannelMessageType,
    pub msg_flags: u8,
    pub msg_size: u16,
}

impl Marshall for CommonDefinitions {
    fn marshall(&self) -> Vec<u8> {
        let msg_type = self.msg_type.clone() as u8;
        let msg_flags = self.msg_flags;

        let mut packet = vec![msg_type, msg_flags];
        packet.extend_from_slice(&self.msg_size.to_be_bytes());

        packet
    }
}

impl Unmarshall for CommonDefinitions {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            buf.len() == Self::get_size_of_fixed_part(),
            "Incoming data too short to unmarshal CommonDefinitions. Expected {} bytes, but got:{}",
            Self::get_size_of_fixed_part(),
            buf.len()
        );

        let msg_type = JmuxChannelMessageType::try_from(buf[0])?;
        let msg_flags = buf[1];
        let msg_size = u16::from_be_bytes([buf[2], buf[3]]);

        Ok(Self {
            msg_type,
            msg_flags,
            msg_size,
        })
    }

    #[inline]
    fn get_size_of_fixed_part() -> usize {
        1 /*msg_type*/ + 1 /*msg_flags*/ + 2 /*msg_size*/
    }
}

#[derive(Debug, PartialEq)]
pub struct JmuxMsgChannelOpen {
    pub common_defs: CommonDefinitions,
    pub sender_channel_id: u32,
    pub initial_window_size: u32,
    pub maximum_packet_size: u32,
    pub destination_url: String,
}

impl JmuxMsgChannelOpen {
    pub fn new(sender_channel_id: u32) -> Self {
        Self {
            common_defs: CommonDefinitions {
                msg_type: JmuxChannelMessageType::Open,
                msg_flags: 0,
                msg_size: Self::get_size_of_fixed_part() as u16,
            },
            sender_channel_id,
            destination_url: "".to_string(),
            initial_window_size: 1 << 15,
            maximum_packet_size: 4096,
        }
    }
}

impl Marshall for JmuxMsgChannelOpen {
    fn marshall(&self) -> Vec<u8> {
        let mut packet = self.common_defs.marshall();

        packet.extend_from_slice(&self.sender_channel_id.to_be_bytes());
        packet.extend_from_slice(&self.initial_window_size.to_be_bytes());
        packet.extend_from_slice(&self.maximum_packet_size.to_be_bytes());
        packet.extend_from_slice(self.destination_url.as_bytes());
        packet
    }
}

impl Unmarshall for JmuxMsgChannelOpen {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            buf.len() >= Self::get_size_of_fixed_part(),
            "Incoming data too short to unmarshal JmuxMsgChannelOpen. Expected at least {} bytes, but got:{}",
            Self::get_size_of_fixed_part(),
            buf.len()
        );

        let (common_defs_buffer, buf) = buf.split_at(CommonDefinitions::get_size_of_fixed_part());
        let mut buf = Cursor::new(buf);
        let common_defs = CommonDefinitions::unmarshall(common_defs_buffer)?;
        let sender_channel_id = buf.read_u32::<BigEndian>().unwrap();
        let initial_window_size = buf.read_u32::<BigEndian>().unwrap();
        let maximum_packet_size = buf.read_u32::<BigEndian>().unwrap();
        let mut destination_url = "".to_owned();
        buf.read_to_string(&mut destination_url).unwrap();

        Ok(Self {
            common_defs,
            sender_channel_id,
            initial_window_size,
            maximum_packet_size,
            destination_url,
        })
    }

    #[inline]
    fn get_size_of_fixed_part() -> usize {
        4 /*common_defs*/ + 4 /*sender_channel_id*/ + 4 /*initial_window_size*/
            + 4 /*maximum_packet_size*/
    }
}

#[derive(Debug, PartialEq)]
pub struct JmuxMsgChannelOpenSuccess {
    pub common_defs: CommonDefinitions,
    pub recipient_channel_id: u32,
    pub sender_channel_id: u32,
    pub initial_window_size: u32,
    pub maximum_packet_size: u32,
}

impl JmuxMsgChannelOpenSuccess {
    pub fn new(recipient_channel_id: u32, sender_channel_id: u32) -> Self {
        Self {
            common_defs: CommonDefinitions {
                msg_type: JmuxChannelMessageType::OpenSuccess,
                msg_flags: 0,
                msg_size: Self::get_size_of_fixed_part() as u16,
            },
            recipient_channel_id,
            sender_channel_id,
            initial_window_size: 64 * (1 << 15),
            maximum_packet_size: 4096,
        }
    }
}

impl Marshall for JmuxMsgChannelOpenSuccess {
    fn marshall(&self) -> Vec<u8> {
        let mut packet = self.common_defs.marshall();

        packet.extend_from_slice(&self.recipient_channel_id.to_be_bytes());
        packet.extend_from_slice(&self.sender_channel_id.to_be_bytes());
        packet.extend_from_slice(&self.initial_window_size.to_be_bytes());
        packet.extend_from_slice(&self.maximum_packet_size.to_be_bytes());

        packet
    }
}

impl Unmarshall for JmuxMsgChannelOpenSuccess {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            buf.len() == Self::get_size_of_fixed_part(),
            "Incoming data too short to unmarshal JmuxMsgChannelOpenSuccess. Expected {} bytes, but got:{}",
            Self::get_size_of_fixed_part(),
            buf.len()
        );
        let (common_defs_buffer, buf) = buf.split_at(CommonDefinitions::get_size_of_fixed_part());
        let mut buf = Cursor::new(buf);
        let common_defs = CommonDefinitions::unmarshall(common_defs_buffer)?;

        let recipient_channel_id = buf.read_u32::<BigEndian>().unwrap();
        let sender_channel_id = buf.read_u32::<BigEndian>().unwrap();
        let initial_window_size = buf.read_u32::<BigEndian>().unwrap();
        let maximum_packet_size = buf.read_u32::<BigEndian>().unwrap();

        Ok(Self {
            common_defs,
            recipient_channel_id,
            sender_channel_id,
            initial_window_size,
            maximum_packet_size,
        })
    }

    #[inline]
    fn get_size_of_fixed_part() -> usize {
        4 /*CommonDefinitions*/ + 4 /*recipient_channel_id*/ + 4 /*sender_channel_id*/ +
            4 /*initial_window_size*/ + 4 /*maximum_packet_size*/
    }
}

#[derive(Debug, PartialEq)]
pub struct JmuxMsgChannelOpenFailure {
    pub common_defs: CommonDefinitions,
    pub recipient_channel_id: u32,
    pub reason_code: u32,
    pub description: String,
}

impl JmuxMsgChannelOpenFailure {
    pub fn new(recipient_channel_id: u32, reason_code: u32, description: String) -> Self {
        Self {
            common_defs: CommonDefinitions {
                msg_type: JmuxChannelMessageType::OpenFailure,
                msg_flags: 0,
                msg_size: (Self::get_size_of_fixed_part() + description.len()) as u16,
            },
            recipient_channel_id,
            reason_code,
            description,
        }
    }
}

impl Marshall for JmuxMsgChannelOpenFailure {
    fn marshall(&self) -> Vec<u8> {
        let mut packet = self.common_defs.marshall();

        packet.extend_from_slice(&self.recipient_channel_id.to_be_bytes());
        packet.extend_from_slice(&self.reason_code.to_be_bytes());
        packet.extend_from_slice(self.description.as_bytes());

        packet
    }
}

impl Unmarshall for JmuxMsgChannelOpenFailure {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            buf.len() >= Self::get_size_of_fixed_part(),
            "Incoming data too short to unmarshal JmuxMsgChannelOpenFailure. Expected at least {} bytes, but got:{}",
            Self::get_size_of_fixed_part(),
            buf.len()
        );

        let (common_defs_buffer, buf) = buf.split_at(CommonDefinitions::get_size_of_fixed_part());
        let mut buf = Cursor::new(buf);
        let common_defs = CommonDefinitions::unmarshall(common_defs_buffer)?;

        let recipient_channel_id = buf.read_u32::<BigEndian>().unwrap();
        let reason_code = buf.read_u32::<BigEndian>().unwrap();
        let mut description = "".to_owned();
        buf.read_to_string(&mut description).unwrap();

        Ok(Self {
            common_defs,
            recipient_channel_id,
            reason_code,
            description,
        })
    }

    #[inline]
    fn get_size_of_fixed_part() -> usize {
        4 /*CommonDefinitions*/ + 4 /*recipient_channel_id*/ + 4 /*reason_code*/
    }
}

#[derive(Debug, PartialEq)]
pub struct JmuxMsgChannelWindowAdjust {
    pub common_defs: CommonDefinitions,
    pub recipient_channel_id: u32,
    pub window_adjustment: u32,
}

impl JmuxMsgChannelWindowAdjust {
    pub fn new(recipient_channel_id: u32, window_adjustment: u32) -> Self {
        JmuxMsgChannelWindowAdjust {
            common_defs: CommonDefinitions {
                msg_type: JmuxChannelMessageType::WindowAdjust,
                msg_flags: 0,
                msg_size: Self::get_size_of_fixed_part() as u16,
            },
            recipient_channel_id,
            window_adjustment,
        }
    }
}

impl Marshall for JmuxMsgChannelWindowAdjust {
    fn marshall(&self) -> Vec<u8> {
        let mut packet = self.common_defs.marshall();

        packet.extend_from_slice(&self.recipient_channel_id.to_be_bytes());
        packet.extend_from_slice(&self.window_adjustment.to_be_bytes());

        packet
    }
}

impl Unmarshall for JmuxMsgChannelWindowAdjust {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            buf.len() == Self::get_size_of_fixed_part(),
            "Incoming data too short to unmarshal JmuxMsgChannelWindowAdjust. Expected {} bytes, but got:{}",
            Self::get_size_of_fixed_part(),
            buf.len()
        );

        let (common_defs_buffer, buf) = buf.split_at(CommonDefinitions::get_size_of_fixed_part());
        let mut buf = Cursor::new(buf);
        let common_defs = CommonDefinitions::unmarshall(common_defs_buffer)?;

        let recipient_channel_id = buf.read_u32::<BigEndian>().unwrap();
        let window_adjustment = buf.read_u32::<BigEndian>().unwrap();

        Ok(Self {
            common_defs,
            recipient_channel_id,
            window_adjustment,
        })
    }

    #[inline]
    fn get_size_of_fixed_part() -> usize {
        4 /*CommonDefinitions*/ + 4 /*recipient_channel_id*/ + 4 /*window_adjustment*/
    }
}

#[derive(Debug, PartialEq)]
pub struct JmuxMsgChannelData {
    pub common_defs: CommonDefinitions,
    pub recipient_channel_id: u32,
    pub data_length: u32,
    pub transfer_data: Vec<u8>,
}

impl JmuxMsgChannelData {
    pub fn new(id: u32, vec: Vec<u8>) -> Self {
        assert!(
            vec.len() < (u16::MAX as usize - Self::get_size_of_fixed_part()),
            "Data buffer too large for JmuxMsgChannelData, maximum allowed {}, get {}",
            u16::MAX,
            vec.len()
        );
        JmuxMsgChannelData {
            common_defs: CommonDefinitions {
                msg_type: JmuxChannelMessageType::Data,
                msg_flags: 0,
                msg_size: (Self::get_size_of_fixed_part() + vec.len()) as u16,
            },
            recipient_channel_id: id,
            data_length: vec.len() as u32,
            transfer_data: vec,
        }
    }
}

impl Marshall for JmuxMsgChannelData {
    fn marshall(&self) -> Vec<u8> {
        let mut packet = self.common_defs.marshall();

        packet.extend_from_slice(&self.recipient_channel_id.to_be_bytes());
        packet.extend_from_slice(&self.data_length.to_be_bytes());
        packet.extend_from_slice(&self.transfer_data);

        packet
    }
}

impl Unmarshall for JmuxMsgChannelData {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            buf.len() >= Self::get_size_of_fixed_part(),
            "Incoming data too short to unmarshal JmuxMsgChannelData. Expected at least {} bytes, but got:{}",
            Self::get_size_of_fixed_part(),
            buf.len()
        );

        let (common_defs_buffer, buf) = buf.split_at(CommonDefinitions::get_size_of_fixed_part());
        let mut buf = Cursor::new(buf);
        let common_defs = CommonDefinitions::unmarshall(common_defs_buffer)?;

        let recipient_channel_id = buf.read_u32::<BigEndian>().unwrap();
        let data_length = buf.read_u32::<BigEndian>().unwrap();
        let mut transfer_data = Vec::new();
        buf.read_to_end(&mut transfer_data).unwrap();

        Ok(Self {
            common_defs,
            recipient_channel_id,
            data_length,
            transfer_data,
        })
    }

    #[inline]
    fn get_size_of_fixed_part() -> usize {
        4 /*CommonDefinitions*/ + 4 /*recipient_channel_id*/ + 4 /*data_length*/
    }
}

#[derive(Debug, PartialEq)]
pub struct JmuxMsgChannelEof {
    pub common_defs: CommonDefinitions,
    pub recipient_channel_id: u32,
}

impl JmuxMsgChannelEof {
    pub fn new(recipient_channel_id: u32) -> Self {
        Self {
            common_defs: CommonDefinitions {
                msg_type: JmuxChannelMessageType::Eof,
                msg_flags: 0,
                msg_size: Self::get_size_of_fixed_part() as u16,
            },
            recipient_channel_id,
        }
    }
}

impl Marshall for JmuxMsgChannelEof {
    fn marshall(&self) -> Vec<u8> {
        let mut packet = self.common_defs.marshall();

        packet.extend_from_slice(&self.recipient_channel_id.to_be_bytes());

        packet
    }
}

impl Unmarshall for JmuxMsgChannelEof {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            buf.len() == Self::get_size_of_fixed_part(),
            "Incoming data too short to unmarshal JmuxMsgChannelEof. Expected {} bytes, but got:{}",
            Self::get_size_of_fixed_part(),
            buf.len()
        );

        let (common_defs_buffer, buf) = buf.split_at(CommonDefinitions::get_size_of_fixed_part());
        let mut buf = Cursor::new(buf);
        let common_defs = CommonDefinitions::unmarshall(common_defs_buffer)?;
        let recipient_channel_id = buf.read_u32::<BigEndian>().unwrap();

        Ok(Self {
            common_defs,
            recipient_channel_id,
        })
    }

    #[inline]
    fn get_size_of_fixed_part() -> usize {
        4 /*common_defs*/ + 4 /*recipient_channel_id*/
    }
}

#[derive(Debug, PartialEq)]
pub struct JmuxMsgChannelClose {
    pub common_defs: CommonDefinitions,
    pub recipient_channel_id: u32,
}

impl JmuxMsgChannelClose {
    pub fn new(recipient_channel_id: u32) -> Self {
        Self {
            common_defs: CommonDefinitions {
                msg_type: JmuxChannelMessageType::Close,
                msg_flags: 0,
                msg_size: Self::get_size_of_fixed_part() as u16,
            },
            recipient_channel_id,
        }
    }
}

impl Marshall for JmuxMsgChannelClose {
    fn marshall(&self) -> Vec<u8> {
        let mut packet = self.common_defs.marshall();
        packet.extend_from_slice(&self.recipient_channel_id.to_be_bytes());

        packet
    }
}

impl Unmarshall for JmuxMsgChannelClose {
    fn unmarshall(buf: &[u8]) -> Result<Self, anyhow::Error> {
        ensure!(
            buf.len() == Self::get_size_of_fixed_part(),
            "Incoming data too short to unmarshal JmuxMsgChannelClose. Expected {} bytes, but got:{}",
            Self::get_size_of_fixed_part(),
            buf.len()
        );

        let (common_defs_buffer, buf) = buf.split_at(CommonDefinitions::get_size_of_fixed_part());
        let mut buf = Cursor::new(buf);
        let common_defs = CommonDefinitions::unmarshall(common_defs_buffer)?;
        let recipient_channel_id = buf.read_u32::<BigEndian>().unwrap();

        Ok(Self {
            common_defs,
            recipient_channel_id,
        })
    }

    #[inline]
    fn get_size_of_fixed_part() -> usize {
        4 /*common_defs*/ + 4 /*recipient_channel_id*/
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommonDefinitions, JmuxChannelMessageType, JmuxMsgChannelClose, JmuxMsgChannelData, JmuxMsgChannelEof,
        JmuxMsgChannelOpen, JmuxMsgChannelOpenFailure, JmuxMsgChannelOpenSuccess, JmuxMsgChannelWindowAdjust, Marshall,
        Unmarshall,
    };
    use std::convert::TryFrom;

    #[test]
    fn try_from_should_return_correct_message_type_on_valid_bytes() {
        let msg_type = JmuxChannelMessageType::try_from(100);
        assert!(msg_type.is_ok());
        assert_eq!(JmuxChannelMessageType::Open, msg_type.unwrap());

        let msg_type = JmuxChannelMessageType::try_from(103);
        assert!(msg_type.is_ok());
        assert_eq!(JmuxChannelMessageType::WindowAdjust, msg_type.unwrap());

        let msg_type = JmuxChannelMessageType::try_from(106);
        assert!(msg_type.is_ok());
        assert_eq!(JmuxChannelMessageType::Close, msg_type.unwrap());
    }

    #[test]
    fn try_from_should_return_err_on_invalid_bytes() {
        let msg_type = JmuxChannelMessageType::try_from(99);
        assert!(msg_type.is_err());

        let msg_type = JmuxChannelMessageType::try_from(107);
        assert!(msg_type.is_err());
    }

    #[test]
    fn common_definitions_unmarshal_return_err_on_short_buf() {
        assert!(CommonDefinitions::unmarshall(&[]).is_err());
    }

    #[test]
    fn common_definitions_unmarshal_return_correct_message() {
        let msg = CommonDefinitions::unmarshall(&[102, 0, 7, 16]);
        assert!(msg.is_ok());
        assert_eq!(
            CommonDefinitions {
                msg_type: JmuxChannelMessageType::OpenFailure,
                msg_flags: 0,
                msg_size: 1808
            },
            msg.unwrap()
        );
    }

    #[test]
    fn common_definitions_marshal_return_correct_buf() {
        let raw_mgs = CommonDefinitions {
            msg_type: JmuxChannelMessageType::OpenSuccess,
            msg_flags: 0,
            msg_size: 512,
        };
        assert_eq!(vec![101, 0, 2, 0], raw_mgs.marshall());
    }

    #[test]
    fn jmux_msg_channel_open_unmarshal_return_err_on_short_buf() {
        assert!(JmuxMsgChannelOpen::unmarshall(&[32, 42]).is_err());
    }

    #[test]
    fn test_jmux_msg_channel_open_unmarshal_return_correct_message() {
        let raw_mgs = [
            100, // msg type
            0,   // msg flags
            0, 36, // msg size
            0, 0, 0, 1, // sender channel id
            0, 0, 4, 0, // initial window size
            0, 0, 4, 0, // maximum packet size
            116, 99, 112, 58, 47, 47, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 58, 52, 52,
            51, // destination url: tcp://google.com:443
        ];
        let msg_example = JmuxMsgChannelOpen {
            initial_window_size: 1024,
            common_defs: CommonDefinitions {
                msg_size: 36,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::Open,
            },
            sender_channel_id: 1,
            maximum_packet_size: 1024,
            destination_url: "tcp://google.com:443".to_owned(),
        };

        let msg = JmuxMsgChannelOpen::unmarshall(&raw_mgs);
        assert!(msg.is_ok());
        assert_eq!(msg_example, msg.unwrap());
    }

    #[test]
    fn test_jmux_msg_channel_open_marshal_return_correct_buf() {
        let raw_mgs = [
            100, // msg type
            0,   // msg flags
            0, 36, // msg size
            0, 0, 0, 1, // sender channel id
            0, 0, 4, 0, // initial window size
            0, 0, 4, 0, // maximum packet size
            116, 99, 112, 58, 47, 47, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 58, 52, 52,
            51, // destination url: tcp://google.com:443
        ];
        let msg_example = JmuxMsgChannelOpen {
            initial_window_size: 1024,
            common_defs: CommonDefinitions {
                msg_size: 36,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::Open,
            },
            sender_channel_id: 1,
            maximum_packet_size: 1024,
            destination_url: "tcp://google.com:443".to_owned(),
        };
        assert_eq!(raw_mgs.to_vec(), msg_example.marshall());
    }

    #[test]
    pub fn jmux_msg_channel_open_success_unmarshal_return_err_on_short_buf() {
        assert!(JmuxMsgChannelOpenSuccess::unmarshall(&[32, 42]).is_err());
    }

    #[test]
    pub fn jmux_msg_channel_open_success_unmarshal_return_correct_message() {
        let raw_mgs = [
            101, // msg type
            0,   // msg flags
            0, 20, // msg size
            0, 0, 0, 1, // recipient channel id
            0, 0, 0, 2, // sender channel id
            0, 0, 4, 0, // initial window size
            0, 0, 127, 255, // maximum packet size
        ];
        let msg_example = JmuxMsgChannelOpenSuccess {
            initial_window_size: 1024,
            common_defs: CommonDefinitions {
                msg_size: 20,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::OpenSuccess,
            },
            sender_channel_id: 2,
            maximum_packet_size: 32767,
            recipient_channel_id: 1,
        };

        let msg = JmuxMsgChannelOpenSuccess::unmarshall(&raw_mgs);
        assert!(msg.is_ok());
        assert_eq!(msg_example, msg.unwrap());
    }

    #[test]
    pub fn jmux_msg_channel_open_success_marshal_return_correct_buf() {
        let raw_mgs = [
            101, // msg type
            0,   // msg flags
            0, 20, // msg size
            0, 0, 0, 1, // recipient channel id
            0, 0, 0, 2, // sender channel id
            0, 0, 4, 0, // initial window size
            0, 0, 127, 255, // maximum packet size
        ];
        let msg_example = JmuxMsgChannelOpenSuccess {
            initial_window_size: 1024,
            common_defs: CommonDefinitions {
                msg_size: 20,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::OpenSuccess,
            },
            sender_channel_id: 2,
            maximum_packet_size: 32767,
            recipient_channel_id: 1,
        };

        assert_eq!(raw_mgs.to_vec(), msg_example.marshall());
    }

    #[test]
    pub fn jmux_msg_channel_open_failure_unmarshal_return_err_on_short_buf() {
        assert!(JmuxMsgChannelOpenFailure::unmarshall(&[32, 42]).is_err());
    }

    #[test]
    pub fn jmux_msg_channel_open_failure_unmarshal_return_correct_message() {
        let raw_mgs = [
            102, // msg type
            0,   // msg flags
            0, 17, // msg size
            0, 0, 0, 1, // recipient channel id
            0, 0, 0, 2, // reason code
            101, 114, 114, 111, 114, // failure description
        ];
        let msg_example = JmuxMsgChannelOpenFailure {
            common_defs: CommonDefinitions {
                msg_size: 17,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::OpenFailure,
            },
            recipient_channel_id: 1,
            reason_code: 2,
            description: "error".to_owned(),
        };

        let msg = JmuxMsgChannelOpenFailure::unmarshall(&raw_mgs);
        assert!(msg.is_ok());
        assert_eq!(msg_example, msg.unwrap());

        let raw_example = msg_example.marshall();
        assert_eq!(raw_mgs.to_vec(), raw_example);
    }

    #[test]
    pub fn jmux_msg_channel_open_failure_marshal_return_correct_buf() {
        let raw_mgs = [
            102, // msg type
            0,   // msg flags
            0, 17, // msg size
            0, 0, 0, 1, // recipient channel id
            0, 0, 0, 2, // reason code
            101, 114, 114, 111, 114, // failure description
        ];
        let msg_example = JmuxMsgChannelOpenFailure {
            common_defs: CommonDefinitions {
                msg_size: 17,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::OpenFailure,
            },
            recipient_channel_id: 1,
            reason_code: 2,
            description: "error".to_owned(),
        };

        assert_eq!(raw_mgs.to_vec(), msg_example.marshall());
    }

    #[test]
    pub fn jmux_msg_channel_window_adjust_unmarshal_return_err_on_short_buf() {
        assert!(JmuxMsgChannelWindowAdjust::unmarshall(&[32, 42]).is_err());
    }

    #[test]
    pub fn jmux_msg_channel_window_adjust_unmarshal_return_correct_message() {
        let raw_mgs = [
            103, // msg type
            0,   // msg flags
            0, 12, // msg size
            0, 0, 0, 1, // recipient channel id
            0, 0, 2, 0, // window adjustment
        ];
        let msg_example = JmuxMsgChannelWindowAdjust {
            common_defs: CommonDefinitions {
                msg_size: 12,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::WindowAdjust,
            },
            recipient_channel_id: 1,
            window_adjustment: 512,
        };

        let msg = JmuxMsgChannelWindowAdjust::unmarshall(&raw_mgs);
        assert!(msg.is_ok());
        assert_eq!(msg_example, msg.unwrap());

        let raw_example = msg_example.marshall();
        assert_eq!(raw_mgs.to_vec(), raw_example);
    }

    #[test]
    pub fn jmux_msg_channel_window_adjust_marshal_return_correct_buf() {
        let raw_mgs = [
            103, // msg type
            0,   // msg flags
            0, 12, // msg size
            0, 0, 0, 1, // recipient channel id
            0, 0, 2, 0, // window adjustment
        ];
        let msg_example = JmuxMsgChannelWindowAdjust {
            common_defs: CommonDefinitions {
                msg_size: 12,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::WindowAdjust,
            },
            recipient_channel_id: 1,
            window_adjustment: 512,
        };

        assert_eq!(raw_mgs.to_vec(), msg_example.marshall());
    }

    #[test]
    #[should_panic]
    pub fn panic_when_reached_max_size_of_jmux_msg_channel_data() {
        JmuxMsgChannelData::new(1, vec![0; u16::MAX as usize]);
    }

    #[test]
    pub fn jmux_msg_channel_data_unmarshal_return_err_on_short_buf() {
        assert!(JmuxMsgChannelData::unmarshall(&[32, 42]).is_err());
    }

    #[test]
    pub fn jmux_msg_channel_data_unmarshal_return_correct_message() {
        let raw_mgs = [
            104, // msg type
            0,   // msg flags
            0, 16, // msg size
            0, 0, 0, 1, // recipient channel id
            0, 0, 0, 4, // data length
            11, 12, 13, 14, // transfer data
        ];
        let msg_example = JmuxMsgChannelData {
            common_defs: CommonDefinitions {
                msg_size: 16,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::Data,
            },
            recipient_channel_id: 1,
            data_length: 4,
            transfer_data: vec![11, 12, 13, 14],
        };

        let msg = JmuxMsgChannelData::unmarshall(&raw_mgs);
        assert!(msg.is_ok());
        assert_eq!(msg_example, msg.unwrap());
    }

    #[test]
    pub fn jmux_msg_channel_data_marshal_return_correct_buf() {
        let raw_mgs = [
            104, // msg type
            0,   // msg flags
            0, 16, // msg size
            0, 0, 0, 1, // recipient channel id
            0, 0, 0, 4, // data length
            11, 12, 13, 14, // transfer data
        ];
        let msg_example = JmuxMsgChannelData {
            common_defs: CommonDefinitions {
                msg_size: 16,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::Data,
            },
            recipient_channel_id: 1,
            data_length: 4,
            transfer_data: vec![11, 12, 13, 14],
        };

        assert_eq!(raw_mgs.to_vec(), msg_example.marshall());
    }

    #[test]
    pub fn jmux_msg_channel_eof_unmarshal_return_err_on_short_buf() {
        assert!(JmuxMsgChannelEof::unmarshall(&[32, 42]).is_err());
    }

    #[test]
    pub fn jmux_msg_channel_eof_unmarshal_return_correct_message() {
        let raw_mgs = [
            105, // msg type
            0,   // msg flags
            0, 8, // msg size
            0, 0, 0, 1, // recipient channel id
        ];
        let msg_example = JmuxMsgChannelEof {
            common_defs: CommonDefinitions {
                msg_size: 8,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::Eof,
            },
            recipient_channel_id: 1,
        };

        let msg = JmuxMsgChannelEof::unmarshall(&raw_mgs);
        assert!(msg.is_ok());
        assert_eq!(msg_example, msg.unwrap());

        let raw_example = msg_example.marshall();
        assert_eq!(raw_mgs.to_vec(), raw_example);
    }

    #[test]
    pub fn jmux_msg_channel_eof_marsal_return_correct_buf() {
        let raw_mgs = [
            105, // msg type
            0,   // msg flags
            0, 8, // msg size
            0, 0, 0, 1, // recipient channel id
        ];
        let msg_example = JmuxMsgChannelEof {
            common_defs: CommonDefinitions {
                msg_size: 8,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::Eof,
            },
            recipient_channel_id: 1,
        };

        assert_eq!(raw_mgs.to_vec(), msg_example.marshall());
    }

    #[test]
    pub fn jmux_msg_channel_close_unmarshal_return_err_on_short_buf() {
        assert!(JmuxMsgChannelClose::unmarshall(&[32, 42]).is_err());
    }

    #[test]
    pub fn jmux_msg_channel_close_unmarshal_return_correct_message() {
        let raw_mgs = [
            106, // msg type
            0,   // msg flags
            0, 8, // msg size
            0, 0, 0, 1, // recipient channel id
        ];
        let msg_example = JmuxMsgChannelClose {
            common_defs: CommonDefinitions {
                msg_size: 8,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::Close,
            },
            recipient_channel_id: 1,
        };

        let msg = JmuxMsgChannelClose::unmarshall(&raw_mgs);
        assert!(msg.is_ok());
        assert_eq!(msg_example, msg.unwrap());

        let raw_example = msg_example.marshall();
        assert_eq!(raw_mgs.to_vec(), raw_example);
    }

    #[test]
    pub fn jmux_msg_channel_close_marshal_return_correct_buf() {
        let raw_mgs = [
            106, // msg type
            0,   // msg flags
            0, 8, // msg size
            0, 0, 0, 1, // recipient channel id
        ];
        let msg_example = JmuxMsgChannelClose {
            common_defs: CommonDefinitions {
                msg_size: 8,
                msg_flags: 0,
                msg_type: JmuxChannelMessageType::Close,
            },
            recipient_channel_id: 1,
        };

        assert_eq!(raw_mgs.to_vec(), msg_example.marshall());
    }
}