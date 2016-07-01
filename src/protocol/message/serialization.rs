use std::any::TypeId;
use std::borrow::Borrow;
use std::collections::HashMap;

use core::{self, NodeId};
use protocol::bits::{PreambleId, MessageTypeId, FunctionId};
use super::{Message, Frame, AnyMessage};
use super::{Ack, Nack, Cancel};
use protocol::command::CommandSerializer;
use protocol::serialization::Read;

trait SerializeMessage: Send + 'static {
    fn key(&self) -> PreambleId;
    fn serialize(&self, message: &Message, buffer: &mut Vec<u8>) -> core::Result<()>;
    fn deserialize(&self, reader: &mut Read) -> core::Result<AnyMessage>;
}

trait SerializeFrame: Send + 'static {
    fn type_id(&self) -> TypeId;
    fn key(&self) -> (MessageTypeId, FunctionId);
    fn serialize(&self, message: &Message, buffer: &mut Vec<u8>) -> core::Result<()>;
    fn deserialize(&self, buffer: &[u8]) -> core::Result<AnyMessage>;
}


struct AckSerializer;

impl SerializeMessage for AckSerializer {
    fn key(&self) -> PreambleId {
        PreambleId::Ack
    }

    fn serialize(&self, _message: &Message, _buffer: &mut Vec<u8>) -> core::Result<()> {
        Ok(())
    }

    fn deserialize(&self, _reader: &mut Read) -> core::Result<AnyMessage> {
        Ok(AnyMessage::new(Ack::new()))
    }
}

struct NackSerializer;

impl SerializeMessage for NackSerializer {
    fn key(&self) -> PreambleId {
        PreambleId::Nack
    }

    fn serialize(&self, _message: &Message, _buffer: &mut Vec<u8>) -> core::Result<()> {
        Ok(())
    }

    fn deserialize(&self, _reader: &mut Read) -> core::Result<AnyMessage> {
        Ok(AnyMessage::new(Nack::new()))
    }
}

struct CancelSerializer;

impl SerializeMessage for CancelSerializer {
    fn key(&self) -> PreambleId {
        PreambleId::Cancel
    }

    fn serialize(&self, _message: &Message, _buffer: &mut Vec<u8>) -> core::Result<()> {
        Ok(())
    }

    fn deserialize(&self, _reader: &mut Read) -> core::Result<AnyMessage> {
        Ok(AnyMessage::new(Cancel::new()))
    }
}


struct SendDataSerializer(CommandSerializer);

impl SerializeFrame for SendDataSerializer {
    fn type_id(&self) -> TypeId {
        TypeId::of::<super::SendData>()
    }

    fn key(&self) -> (MessageTypeId, FunctionId) {
        (super::SendData::MESSAGE_TYPE_ID, super::SendData::FUNCTION_ID)
    }

    fn serialize(&self, message: &Message, buffer: &mut Vec<u8>) -> core::Result<()> {
        let send_data = message.downcast_ref::<super::SendData>().unwrap();

        buffer.push(send_data.destination().value());

        let length_offset = buffer.len();
        buffer.push(0x00); // payload length; come back when it's known

        let payload_offset = buffer.len();
        try!(self.0.serialize(send_data.command().borrow(), buffer));

        buffer[length_offset] = (buffer.len() - payload_offset) as u8;

        buffer.push(send_data.packet_options());
        buffer.push(send_data.callback_id());

        Ok(())
    }

    fn deserialize(&self, buffer: &[u8]) -> core::Result<AnyMessage> {
        if buffer.len() < 4 {
            return Err(core::Error::new(core::ErrorKind::ShortRead));
        }

        let destination = NodeId(buffer[0]);
        let payload_length = buffer[1] as usize;
        let command = try!(self.0.deserialize(&buffer[2..2+payload_length]));
        let packet_options = buffer[2 + payload_length];
        let callback_id = buffer[3 + payload_length];

        Ok(AnyMessage::new(super::SendData {
            destination: destination,
            command: command,
            packet_options: packet_options,
            callback_id: callback_id,
        }))
    }
}

struct MessageTransmittedSerializer;

impl SerializeFrame for MessageTransmittedSerializer {
    fn type_id(&self) -> TypeId {
        TypeId::of::<super::MessageTransmitted>()
    }

    fn key(&self) -> (MessageTypeId, FunctionId) {
        (super::MessageTransmitted::MESSAGE_TYPE_ID, super::MessageTransmitted::FUNCTION_ID)
    }

    fn serialize(&self, message: &Message, buffer: &mut Vec<u8>) -> core::Result<()> {
        let message = message.downcast_ref::<super::MessageTransmitted>().unwrap();

        buffer.push(message.flags());

        Ok(())
    }

    fn deserialize(&self, buffer: &[u8]) -> core::Result<AnyMessage> {
        if buffer.len() < 1 {
            return Err(core::Error::new(core::ErrorKind::ShortRead));
        }

        Ok(AnyMessage::new(super::MessageTransmitted::new(buffer[0])))
    }
}

struct MessageReceivedSerializer;

impl SerializeFrame for MessageReceivedSerializer {
    fn type_id(&self) -> TypeId {
        TypeId::of::<super::MessageReceived>()
    }

    fn key(&self) -> (MessageTypeId, FunctionId) {
        (super::MessageReceived::MESSAGE_TYPE_ID, super::MessageReceived::FUNCTION_ID)
    }

    fn serialize(&self, message: &Message, buffer: &mut Vec<u8>) -> core::Result<()> {
        let message = message.downcast_ref::<super::MessageReceived>().unwrap();

        buffer.push(message.callback_id());
        buffer.push(message.flags());

        Ok(())
    }

    fn deserialize(&self, buffer: &[u8]) -> core::Result<AnyMessage> {
        if buffer.len() < 1 {
            return Err(core::Error::new(core::ErrorKind::ShortRead));
        }

        Ok(AnyMessage::new(super::MessageReceived::new(buffer[0], buffer[1])))
    }
}


struct FrameSerializer {
    types: HashMap<(MessageTypeId,FunctionId), TypeId>,
    serializers: HashMap<TypeId, Box<SerializeFrame>>,
}

impl FrameSerializer {
    fn new() -> Self {
        FrameSerializer {
            types: HashMap::<(MessageTypeId,FunctionId), TypeId>::new(),
            serializers: HashMap::<TypeId, Box<SerializeFrame>>::new(),
        }
    }

    fn for_request() -> Self {
        let mut serializer = Self::new();

        serializer.register(SendDataSerializer(CommandSerializer::new()));

        serializer
    }

    fn for_response() -> Self {
        let mut serializer = Self::new();

        serializer.register(MessageTransmittedSerializer);
        serializer.register(MessageReceivedSerializer);

        serializer
    }

    fn register<S: SerializeFrame>(&mut self, serializer: S) {
        self.types.insert(serializer.key(), serializer.type_id());
        self.serializers.insert(serializer.type_id(), Box::new(serializer));
    }
}

impl SerializeMessage for FrameSerializer {
    fn key(&self) -> PreambleId {
        PreambleId::Frame
    }

    fn serialize(&self, message: &Message, buffer: &mut Vec<u8>) -> core::Result<()> {
        let serializer = self.serializers.get(&message.type_id()).unwrap();

        let (message_type_id, function_id) = serializer.key();

        let length_index = buffer.len();

        buffer.push(0x00); // frame length; come back when it's known
        buffer.push(message_type_id as u8);
        buffer.push(function_id as u8);

        try!(serializer.serialize(message, buffer));

        buffer[length_index] = (buffer.len() - length_index) as u8; // set frame length

        let parity = buffer.iter().skip(length_index).fold(0xFF, |acc, &x| acc ^ x);
        buffer.push(parity);

        Ok(())
    }

    fn deserialize(&self, reader: &mut Read) -> core::Result<AnyMessage> {
        let length = try!(reader.read_u8()) as usize;
        let buffer = try!(reader.read_slice(length));

        let parity = buffer.iter().fold(0xFF ^ length as u8, |acc, &x| acc ^ x);

        if parity != 0 {
            return Err(core::Error::new(core::ErrorKind::Corrupt));
        }

        match (MessageTypeId::from_u8(buffer[0]), FunctionId::from_u8(buffer[1])) {
            (Some(message_type_id), Some(function_id)) => {
                match self.types.get(&(message_type_id, function_id)) {
                    Some(type_id) => {
                        match self.serializers.get(type_id) {
                            Some(serializer) => serializer.deserialize(&buffer[2..length-1]),
                            None => Err(core::Error::new(core::ErrorKind::Protocol)),
                        }
                    },
                    None => Err(core::Error::new(core::ErrorKind::Protocol)),
                }
            }
            _ => Err(core::Error::new(core::ErrorKind::Protocol)),
        }
    }
}


pub struct MessageSerializer {
    serializers: HashMap<PreambleId, Box<SerializeMessage>>,
}

impl MessageSerializer {
    fn new() -> Self {
        let mut serializer = MessageSerializer {
            serializers: HashMap::<PreambleId, Box<SerializeMessage>>::new(),
        };

        serializer.register(AckSerializer);
        serializer.register(NackSerializer);
        serializer.register(CancelSerializer);

        serializer
    }

    pub fn for_request() -> Self {
        let mut serializer = Self::new();

        serializer.register(FrameSerializer::for_request());

        serializer
    }

    pub fn for_response() -> Self {
        let mut serializer = Self::new();

        serializer.register(FrameSerializer::for_response());

        serializer
    }

    pub fn serialize(&self, message: &Message, buffer: &mut Vec<u8>) -> core::Result<()> {
        buffer.push(message.preamble_id() as u8);

        let serializer = self.serializers.get(&message.preamble_id()).unwrap();

        serializer.serialize(message, buffer)
    }

    pub fn deserialize(&self, reader: &mut Read) -> core::Result<AnyMessage> {
        match PreambleId::from_u8(try!(reader.read_u8())) {
            Some(preamble_id) => {
                let serializer = self.serializers.get(&preamble_id).unwrap();
                serializer.deserialize(reader)
            },
            None => Err(core::Error::new(core::ErrorKind::Protocol)),
        }
    }

    fn register<S: SerializeMessage>(&mut self, serializer: S) {
        self.serializers.insert(serializer.key(), Box::new(serializer));
    }
}
