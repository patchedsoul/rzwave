use std::error;
use std::fmt;
use std::result;

use serial;

pub type Result<T> = result::Result<T,Error>;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ErrorKind {
    Protocol,
    ShortRead,
    Corrupt,
    Io,
    Timeout,
    Nack,
    Cancel,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error {
            kind: kind,
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Protocol => "protocol error",
            ErrorKind::ShortRead => "data is too short",
            ErrorKind::Corrupt => "data is corrupt",
            ErrorKind::Io => "I/O error",
            ErrorKind::Timeout => "operation timed out",
            ErrorKind::Nack => "request not acknowledged",
            ErrorKind::Cancel => "request canceled",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        fmt.write_str(self.description())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        Error::description(self)
    }
}

impl From<serial::Error> for Error {
    fn from(_: serial::Error) -> Self {
        Error::new(ErrorKind::Io)
    }
}

#[derive(Debug,Default,Clone,Copy,PartialEq,Eq)]
pub struct NodeId(pub u8);

impl NodeId {
    pub fn value(&self) -> u8 {
        let NodeId(value) = *self;
        value
    }
}
