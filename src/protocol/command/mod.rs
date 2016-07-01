use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::Reflect;
use std::mem;
use std::raw::TraitObject;

use core;
use protocol::bits::{CommandClassId,CommandId};

pub mod basic;

pub trait Command: Send + Sync + Debug + Reflect + 'static {
    const COMMAND_CLASS_ID: CommandClassId;
    const COMMAND_ID: CommandId;

    fn command_class_id(&self) -> CommandClassId {
        Self::COMMAND_CLASS_ID
    }

    fn command_id(&self) -> CommandId {
        Self::COMMAND_ID
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl Command {
    pub fn downcast_ref<C: Command>(&self) -> Option<&C> {
        if TypeId::of::<C>() == self.type_id() {
            unsafe {
                let fat_ptr: TraitObject = mem::transmute(self);

                Some(&*(fat_ptr.data as *const C))
            }
        }
        else {
            None
        }
    }
}

def_any!(AnyCommand: Command);

pub trait Serialize: Send + 'static {
    fn type_id(&self) -> TypeId;
    fn key(&self) -> (CommandClassId, CommandId);
    fn serialize(&self, command: &Command, buffer: &mut Vec<u8>) -> core::Result<()>;
    fn deserialize(&self, buffer: &[u8]) -> core::Result<AnyCommand>;
}

pub struct CommandSerializer {
    types: HashMap<(CommandClassId,CommandId), TypeId>,
    serializers: HashMap<TypeId, Box<Serialize>>,
}

impl CommandSerializer {
    pub fn new() -> Self {
        let mut serializer = CommandSerializer {
            types: HashMap::<(CommandClassId,CommandId), TypeId>::new(),
            serializers: HashMap::<TypeId, Box<Serialize>>::new(),
        };

        serializer.register(basic::serialization::SetValueSerializer);
        serializer.register(basic::serialization::GetValueSerializer);

        serializer
    }

    pub fn serialize(&self, command: &Command, buffer: &mut Vec<u8>) -> core::Result<()> {
        buffer.push(command.command_class_id());
        buffer.push(command.command_id());

        match self.serializers.get(&command.type_id()) {
            Some(serializer) => serializer.serialize(command, buffer),
            None => Err(core::Error::new(core::ErrorKind::Protocol)),
        }
    }

    pub fn deserialize(&self, buffer: &[u8]) -> core::Result<AnyCommand> {
        if buffer.len() < 2 {
            return Err(core::Error::new(core::ErrorKind::ShortRead));
        }

        let command_class_id = buffer[0];
        let command_id = buffer[1];


        match self.types.get(&(command_class_id, command_id)) {
            Some(type_id) => {
                match self.serializers.get(type_id) {
                    Some(serializer) => serializer.deserialize(&buffer[2..]),
                    None => Err(core::Error::new(core::ErrorKind::Protocol)),
                }
            },
            None => Err(core::Error::new(core::ErrorKind::Protocol)),
        }
    }

    fn register<S: Serialize>(&mut self, serializer: S) {
        self.types.insert(serializer.key(), serializer.type_id());
        self.serializers.insert(serializer.type_id(), Box::new(serializer));
    }
}
