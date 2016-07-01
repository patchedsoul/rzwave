use protocol::bits::{CommandClassId, CommandId};
use protocol::command::Command;

pub const COMMAND_CLASS_ID: CommandClassId = 0x20;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct SetValue {
    value: u8,
}

impl SetValue {
    pub fn new(value: u8) -> Self {
        SetValue { value: value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

impl Command for SetValue {
    const COMMAND_CLASS_ID: CommandClassId = COMMAND_CLASS_ID;
    const COMMAND_ID: CommandId = 0x01;
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct GetValue { }

impl GetValue {
    pub fn new() -> Self {
        GetValue { }
    }
}

impl Command for GetValue {
    const COMMAND_CLASS_ID: CommandClassId = COMMAND_CLASS_ID;
    const COMMAND_ID: CommandId = 0x02;
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Report {
    value: u8,
}

impl Report {
    pub fn new(value: u8) -> Self {
        Report { value: value }
    }
}

pub mod serialization {
    use std::any::TypeId;

    use core;
    use protocol::bits::{CommandClassId, CommandId};
    use protocol::command::{Serialize, Command, AnyCommand};

    pub struct SetValueSerializer;

    impl Serialize for SetValueSerializer {
        fn type_id(&self) -> TypeId {
            TypeId::of::<super::SetValue>()
        }

        fn key(&self) -> (CommandClassId, CommandId) {
            (super::SetValue::COMMAND_CLASS_ID, super::SetValue::COMMAND_ID)
        }

        fn serialize(&self, command: &Command, buffer: &mut Vec<u8>) -> core::Result<()> {
            let set_value = command.downcast_ref::<super::SetValue>().unwrap();
            buffer.push(set_value.value());
            Ok(())
        }

        fn deserialize(&self, buffer: &[u8]) -> core::Result<AnyCommand> {
            if buffer.len() == 1 {
                Ok(AnyCommand::new(super::SetValue::new(buffer[0])))
            }
            else {
                Err(core::Error::new(core::ErrorKind::ShortRead))
            }
        }
    }

    pub struct GetValueSerializer;

    impl Serialize for GetValueSerializer {
        fn type_id(&self) -> TypeId {
            TypeId::of::<super::GetValue>()
        }

        fn key(&self) -> (CommandClassId, CommandId) {
            (super::GetValue::COMMAND_CLASS_ID, super::GetValue::COMMAND_ID)
        }

        fn serialize(&self, command: &Command, _buffer: &mut Vec<u8>) -> core::Result<()> {
            command.downcast_ref::<super::GetValue>().unwrap();
            Ok(())
        }

        fn deserialize(&self, _buffer: &[u8]) -> core::Result<AnyCommand> {
            Ok(AnyCommand::new(super::GetValue::new()))
        }
    }
}
