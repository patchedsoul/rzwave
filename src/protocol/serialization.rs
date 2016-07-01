use std::io;
use std::mem;
use std::slice;

use core::{self, Error, ErrorKind};

pub trait Read {
    fn read_u8(&mut self) -> core::Result<u8>;
    fn read_slice(&mut self, length: usize) -> core::Result<&[u8]>;
}

pub struct Reader<'a, R: io::Read + 'a> {
    read: &'a mut R,
    buffer: Vec<u8>,
}

impl<'a, R: io::Read + 'a> Reader<'a, R> {
    pub fn new(read: &'a mut R) -> Self {
        Reader {
            read: read,
            buffer: Vec::<u8>::new(),
        }
    }

    pub fn as_slice<'b>(&'b self) -> &'b [u8] {
        self.buffer.as_slice()
    }
}

impl<'a, R: io::Read + 'a> Read for Reader<'a, R> {
    fn read_u8(&mut self) -> core::Result<u8> {
        let mut byte: u8 = unsafe { mem::uninitialized() };

        let buffer = unsafe {
            slice::from_raw_parts_mut(&mut byte, mem::size_of_val(&byte))
        };

        match self.read.read(buffer) {
            Ok(1) => Ok(byte),
            Ok(0) => Err(Error::new(ErrorKind::ShortRead)),
            Ok(_) => unreachable!(),
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::TimedOut => Err(Error::new(ErrorKind::Timeout)),
                    _ => Err(Error::new(ErrorKind::Io)),
                }
            }
        }
    }

    fn read_slice(&mut self, length: usize) -> core::Result<&[u8]> {
        self.buffer.reserve(length);

        let read_offset = self.buffer.len();

        let read_buffer = unsafe {
            slice::from_raw_parts_mut(
                self.buffer.as_mut_ptr().offset(read_offset as isize),
                length)
        };

        match self.read.read(read_buffer) {
            Ok(n) => {
                if n == length {
                    unsafe {
                        self.buffer.set_len(read_offset + length);
                    }

                    Ok(&read_buffer[read_offset..read_offset+length])
                }
                else {
                    Err(Error::new(ErrorKind::ShortRead))
                }
            },
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::TimedOut => Err(Error::new(ErrorKind::Timeout)),
                    _ => Err(Error::new(ErrorKind::Io)),
                }
            }
        }
    }
}
