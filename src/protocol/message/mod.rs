use std::any::TypeId;
use std::fmt::Debug;
use std::marker::Reflect;
use std::mem;
use std::raw::TraitObject;

use core::NodeId;
use protocol::bits::{PreambleId, MessageTypeId, FunctionId};
use protocol::command::{Command, AnyCommand};

pub use self::serialization::MessageSerializer;

#[doc(hidden)]
pub mod serialization;

pub trait Message: Send + Sync + Debug + Reflect + 'static {
    #[doc(hidden)]
    const PREAMBLE_ID: PreambleId;

    #[doc(hidden)]
    fn preamble_id(&self) -> PreambleId {
        Self::PREAMBLE_ID
    }

    #[doc(hidden)]
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl Message {
    pub fn is<M: Message>(&self) -> bool {
        TypeId::of::<M>() == self.type_id()
    }

    pub fn downcast_ref<M: Message>(&self) -> Option<&M> {
        if TypeId::of::<M>() == self.type_id() {
            unsafe {
                let fat_ptr: TraitObject = mem::transmute(self);

                Some(&*(fat_ptr.data as *const M))
            }
        }
        else {
            None
        }
    }
}

pub trait Frame: Send + Sync + Debug + Reflect + 'static {
    #[doc(hidden)]
    const MESSAGE_TYPE_ID: MessageTypeId;

    #[doc(hidden)]
    const FUNCTION_ID: FunctionId;

    #[doc(hidden)]
    fn message_type_id(&self) -> MessageTypeId {
        Self::MESSAGE_TYPE_ID
    }

    #[doc(hidden)]
    fn function_id(&self) -> FunctionId {
        Self::FUNCTION_ID
    }

    #[doc(hidden)]
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl<F: Frame> Message for F {
    const PREAMBLE_ID: PreambleId = PreambleId::Frame;
}

def_any!(AnyMessage: Message);

#[derive(Debug)]
pub struct Ack { }

impl Ack {
    pub fn new() -> Self {
        Ack { }
    }
}

impl Message for Ack {
    const PREAMBLE_ID: PreambleId = PreambleId::Ack;
}

#[derive(Debug)]
pub struct Nack { }

impl Nack {
    pub fn new() -> Self {
        Nack { }
    }
}

impl Message for Nack {
    const PREAMBLE_ID: PreambleId = PreambleId::Nack;
}

#[derive(Debug)]
pub struct Cancel { }

impl Cancel {
    pub fn new() -> Self {
        Cancel { }
    }
}

impl Message for Cancel {
    const PREAMBLE_ID: PreambleId = PreambleId::Cancel;
}

#[derive(Debug)]
pub struct SendData {
    destination: NodeId,
    command: AnyCommand,
    callback_id: u8,
    packet_options: u8,
}

impl SendData {
    pub fn new<C: Command>(destination: NodeId, command: C, callback_id: u8) -> Self {
        SendData::with_options(destination, command, callback_id, 0x05)
    }

    pub fn with_options<C: Command>(destination: NodeId, command: C, callback_id: u8, packet_options: u8) -> Self {
        SendData {
            destination: destination,
            command: AnyCommand::new(command),
            callback_id: callback_id,
            packet_options: packet_options,
        }
    }

    pub fn destination(&self) -> NodeId {
        self.destination
    }

    pub fn command(&self) -> &AnyCommand {
        &self.command
    }

    pub fn callback_id(&self) -> u8 {
        self.callback_id
    }

    pub fn packet_options(&self) -> u8 {
        self.packet_options
    }
}

impl Frame for SendData {
    const MESSAGE_TYPE_ID: MessageTypeId = MessageTypeId::Request;
    const FUNCTION_ID: FunctionId = FunctionId::SendData;
}


#[derive(Debug)]
pub struct MessageTransmitted {
    flags: u8,
}

impl MessageTransmitted {
    pub fn new(flags: u8) -> Self {
        MessageTransmitted {
            flags: flags,
        }
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }
}

impl Frame for MessageTransmitted {
    const MESSAGE_TYPE_ID: MessageTypeId = MessageTypeId::Response;
    const FUNCTION_ID: FunctionId = FunctionId::SendData;
}

#[derive(Debug)]
pub struct MessageReceived {
    callback_id: u8,
    flags: u8,
}

impl MessageReceived {
    pub fn new(callback_id: u8, flags: u8) -> Self {
        MessageReceived {
            callback_id: callback_id,
            flags: flags,
        }
    }

    pub fn callback_id(&self) -> u8 {
        self.callback_id
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }
}

impl Frame for MessageReceived {
    const MESSAGE_TYPE_ID: MessageTypeId = MessageTypeId::Request;
    const FUNCTION_ID: FunctionId = FunctionId::SendData;
}
