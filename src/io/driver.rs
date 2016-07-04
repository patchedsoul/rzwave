use std::time::Duration;

use serial::{self, SerialPort};

use core;
use protocol::message::{MessageSerializer, Message, AnyMessage};
use protocol::serialization::Reader;

pub trait Driver: Send + 'static {
    fn send(&mut self, message: &Message) -> core::Result<()>;
    fn receive(&mut self) -> core::Result<AnyMessage>;
}

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud115200,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

pub struct SerialDriver<S: SerialPort+Send> {
    port: S,
    request: MessageSerializer,
    response: MessageSerializer,
}

impl<S: SerialPort+Send> SerialDriver<S> {
    pub fn new(mut port: S) -> core::Result<Self> {
        try!(port.configure(&SETTINGS));
        try!(port.set_timeout(Duration::from_millis(10)));

        Ok(SerialDriver {
            port: port,
            request: MessageSerializer::for_request(),
            response: MessageSerializer::for_response(),
        })
    }
}

impl<S: SerialPort+Send+'static> Driver for SerialDriver<S> {
    fn send(&mut self, message: &Message) -> core::Result<()> {
        let mut buffer = Vec::<u8>::with_capacity(16);
        try!(self.request.serialize(message, &mut buffer));

        if self.port.write_all(&buffer).is_err() {
            return Err(core::Error::new(core::ErrorKind::Io));
        }

        Ok(())
    }

    fn receive(&mut self) -> core::Result<AnyMessage> {
        let mut reader = Reader::new(&mut self.port);
        let response = try!(self.response.deserialize(&mut reader));

        Ok(response)
    }
}
