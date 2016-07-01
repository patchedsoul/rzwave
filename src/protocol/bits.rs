#[derive(Debug,Hash,PartialEq,Eq)]
#[repr(u8)]
pub enum PreambleId {
    Frame = 0x01,
    Ack = 0x06,
    Nack = 0x15,
    Cancel = 0x18,
}

impl PreambleId {
    pub fn from_u8(value: u8) -> Option<PreambleId> {
        match value {
            0x01 => Some(PreambleId::Frame),
            0x06 => Some(PreambleId::Ack),
            0x15 => Some(PreambleId::Nack),
            0x18 => Some(PreambleId::Cancel),

            _ => None,
        }
    }
}

#[derive(Debug,Hash,PartialEq,Eq)]
#[repr(u8)]
pub enum MessageTypeId {
    Request = 0x00,
    Response = 0x01,
}

impl MessageTypeId {
    pub fn from_u8(value: u8) -> Option<MessageTypeId> {
        match value {
            0x00 => Some(MessageTypeId::Request),
            0x01 => Some(MessageTypeId::Response),

            _ => None,
        }
    }
}

#[derive(Debug,Hash,PartialEq,Eq)]
#[repr(u8)]
pub enum FunctionId {
    SendData = 0x13,
}

impl FunctionId {
    pub fn from_u8(value: u8) -> Option<FunctionId> {
        match value {
            0x13 => Some(FunctionId::SendData),

            _ => None,
        }
    }
}

pub type CommandClassId = u8;
pub type CommandId = u8;
