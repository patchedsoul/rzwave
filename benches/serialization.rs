#![feature(test)]

extern crate zwave;
extern crate test;

use std::io::Cursor;

use test::Bencher;

use zwave::core::NodeId;
use zwave::protocol::message::MessageSerializer;
use zwave::protocol::message::{Ack, Nack, Cancel};
use zwave::protocol::message::{SendData, MessageTransmitted, MessageReceived};
use zwave::protocol::command::basic::{SetValue, GetValue};
use zwave::protocol::serialization::Reader;

#[bench]
fn bench_serialize_ack(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let mut buffer = Vec::<u8>::with_capacity(16);
    let message = Ack::new();

    b.iter(|| {
        buffer.clear();
        serializer.serialize(&message, &mut buffer).unwrap();
    });
}

#[bench]
fn bench_deserialize_ack(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let buffer = &[0x06];

    b.iter(|| {
        let mut cursor = Cursor::new(buffer);
        let mut reader = Reader::new(&mut cursor);
        serializer.deserialize(&mut reader).unwrap();
    });
}

#[bench]
fn bench_serialize_nack(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let mut buffer = Vec::<u8>::with_capacity(16);
    let message = Nack::new();

    b.iter(|| {
        buffer.clear();
        serializer.serialize(&message, &mut buffer).unwrap();
    });
}

#[bench]
fn bench_deserialize_nack(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let buffer = &[0x15];

    b.iter(|| {
        let mut cursor = Cursor::new(buffer);
        let mut reader = Reader::new(&mut cursor);
        serializer.deserialize(&mut reader).unwrap();
    });
}

#[bench]
fn bench_serialize_cancel(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let mut buffer = Vec::<u8>::with_capacity(16);
    let message = Cancel::new();

    b.iter(|| {
        buffer.clear();
        serializer.serialize(&message, &mut buffer).unwrap();
    });
}

#[bench]
fn bench_deserialize_cancel(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let buffer = &[0x18];

    b.iter(|| {
        let mut cursor = Cursor::new(buffer);
        let mut reader = Reader::new(&mut cursor);
        serializer.deserialize(&mut reader).unwrap();
    });
}

#[bench]
fn bench_serialize_set_value(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let mut buffer = Vec::<u8>::with_capacity(16);
    let command = SetValue::new(42);
    let message = SendData::new(NodeId(2), command, 0x11);

    b.iter(|| {
        buffer.clear();
        serializer.serialize(&message, &mut buffer).unwrap();
    });
}

#[bench]
fn bench_deserialize_set_value(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let buffer = &[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11, 0xF8];

    b.iter(|| {
        let mut cursor = Cursor::new(buffer);
        let mut reader = Reader::new(&mut cursor);
        serializer.deserialize(&mut reader).unwrap();
    });
}

#[bench]
fn bench_serialize_get_value(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let mut buffer = Vec::<u8>::with_capacity(16);
    let command = GetValue::new();
    let message = SendData::new(NodeId(2), command, 0x11);

    b.iter(|| {
        buffer.clear();
        serializer.serialize(&message, &mut buffer).unwrap();
    });
}

#[bench]
fn bench_deserialize_get_value(b: &mut Bencher) {
    let serializer = MessageSerializer::for_request();
    let buffer = &[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x05, 0x11, 0xD3];

    b.iter(|| {
        let mut cursor = Cursor::new(buffer);
        let mut reader = Reader::new(&mut cursor);
        serializer.deserialize(&mut reader).unwrap();
    });
}

#[bench]
fn bench_serialize_message_transmitted(b: &mut Bencher) {
    let serializer = MessageSerializer::for_response();
    let mut buffer = Vec::<u8>::with_capacity(16);
    let message = MessageTransmitted::new(0x01);

    b.iter(|| {
        buffer.clear();
        serializer.serialize(&message, &mut buffer).unwrap();
    });
}

#[bench]
fn bench_deserialize_message_transmitted(b: &mut Bencher) {
    let serializer = MessageSerializer::for_response();
    let buffer = &[0x01, 0x04, 0x01, 0x13, 0x01, 0xE8];

    b.iter(|| {
        let mut cursor = Cursor::new(buffer);
        let mut reader = Reader::new(&mut cursor);
        serializer.deserialize(&mut reader).unwrap();
    });
}

#[bench]
fn bench_serialize_message_received(b: &mut Bencher) {
    let serializer = MessageSerializer::for_response();
    let mut buffer = Vec::<u8>::with_capacity(16);
    let message = MessageReceived::new(0x11, 0x01);

    b.iter(|| {
        buffer.clear();
        serializer.serialize(&message, &mut buffer).unwrap();
    });
}

#[bench]
fn bench_deserialize_message_received(b: &mut Bencher) {
    let serializer = MessageSerializer::for_response();
    let buffer = &[0x01, 0x05, 0x00, 0x13, 0x11, 0x01, 0xF9];

    b.iter(|| {
        let mut cursor = Cursor::new(buffer);
        let mut reader = Reader::new(&mut cursor);
        serializer.deserialize(&mut reader).unwrap();
    });
}
