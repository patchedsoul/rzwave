// vim: set foldmethod=syntax foldlevel=1 :

extern crate zwave;

mod ack {
    mod serialize {
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::Ack;

        fn serialized() -> Vec<u8> {
            let serializer = MessageSerializer::for_response();
            let mut buffer = Vec::<u8>::with_capacity(16);

            serializer.serialize(&Ack::new(), &mut buffer).unwrap();

            buffer
        }

        #[test]
        fn it_serializes_preamble() {
            assert_eq!(0x06, serialized()[0]);
        }

        #[test]
        fn it_serializes_correct_length() {
            assert_eq!(1, serialized().len());
        }
    }

    mod deserialize {
        use std::io::Cursor;

        use zwave::core::ErrorKind;
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::Ack;
        use zwave::protocol::serialization::Reader;

        #[test]
        fn it_deserializes_preamble() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[0x06];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let response = serializer.deserialize(&mut reader).unwrap();

            assert!(response.is::<Ack>());
        }

        #[test]
        fn it_handles_short_packets() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let result = serializer.deserialize(&mut reader);

            assert!(result.is_err());
            assert_eq!(ErrorKind::ShortRead, result.err().unwrap().kind());
        }
    }
}

mod nack {
    mod serialize {
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::Nack;

        fn serialized() -> Vec<u8> {
            let serializer = MessageSerializer::for_response();
            let mut buffer = Vec::<u8>::with_capacity(16);

            serializer.serialize(&Nack::new(), &mut buffer).unwrap();

            buffer
        }

        #[test]
        fn it_serializes_preamble() {
            assert_eq!(0x15, serialized()[0]);
        }

        #[test]
        fn it_serializes_correct_length() {
            assert_eq!(1, serialized().len());
        }
    }

    mod deserialize {
        use std::io::Cursor;

        use zwave::core::ErrorKind;
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::Nack;
        use zwave::protocol::serialization::Reader;

        #[test]
        fn it_deserializes_preamble() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[0x15];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let response = serializer.deserialize(&mut reader).unwrap();

            assert!(response.is::<Nack>());
        }

        #[test]
        fn it_handles_short_packets() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let result = serializer.deserialize(&mut reader);

            assert!(result.is_err());
            assert_eq!(ErrorKind::ShortRead, result.err().unwrap().kind());
        }
    }
}

mod cancel {
    mod serialize {
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::Cancel;

        fn serialized() -> Vec<u8> {
            let serializer = MessageSerializer::for_response();
            let mut buffer = Vec::<u8>::with_capacity(16);

            serializer.serialize(&Cancel::new(), &mut buffer).unwrap();

            buffer
        }

        #[test]
        fn it_serializes_preamble() {
            assert_eq!(0x18, serialized()[0]);
        }

        #[test]
        fn it_serializes_correct_length() {
            assert_eq!(1, serialized().len());
        }
    }

    mod deserialize {
        use std::io::Cursor;

        use zwave::core::ErrorKind;
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::Cancel;
        use zwave::protocol::serialization::Reader;

        #[test]
        fn it_deserializes_preamble() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[0x18];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let response = serializer.deserialize(&mut reader).unwrap();

            assert!(response.is::<Cancel>());
        }

        #[test]
        fn it_handles_short_packets() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let result = serializer.deserialize(&mut reader);

            assert!(result.is_err());
            assert_eq!(ErrorKind::ShortRead, result.err().unwrap().kind());
        }
    }
}

mod message_transmitted {
    mod serialize {
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::MessageTransmitted;

        fn serialized(flags: u8) -> Vec<u8> {
            let serializer = MessageSerializer::for_response();
            let mut buffer = Vec::<u8>::with_capacity(16);

            serializer.serialize(&MessageTransmitted::new(flags), &mut buffer).unwrap();

            buffer
        }

        #[test]
        fn it_serializes_preamble() {
            assert_eq!(0x01, serialized(0x01)[0]);
        }

        #[test]
        fn it_serializes_length() {
            assert_eq!(0x04, serialized(0x01)[1]);
        }

        #[test]
        fn it_serializes_message_type() {
            assert_eq!(0x01, serialized(0x01)[2]);
        }

        #[test]
        fn it_serializes_function_id() {
            assert_eq!(0x13, serialized(0x01)[3]);
        }

        #[test]
        fn it_serializes_flags() {
            assert_eq!(0x01, serialized(0x01)[4]);
            assert_eq!(0x42, serialized(0x42)[4]);
        }

        #[test]
        fn it_serializes_checksum() {
            assert_eq!(0xE8, serialized(0x01)[5]);
            assert_eq!(0xAB, serialized(0x42)[5]);
        }

        #[test]
        fn it_serializes_correct_length() {
            assert_eq!(6, serialized(0x01).len());
        }
    }

    mod deserialize {
        use std::io::Cursor;

        use zwave::core::ErrorKind;
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::MessageTransmitted;
        use zwave::protocol::serialization::Reader;

        fn deserialized(buffer: &[u8]) -> Box<MessageTransmitted> {
            let serializer = MessageSerializer::for_response();
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let response = serializer.deserialize(&mut reader).unwrap();

            response.downcast::<MessageTransmitted>().unwrap()
        }

        #[test]
        fn it_deserializes_flags() {
            assert_eq!(0x01, deserialized(&[0x01, 0x04, 0x01, 0x13, 0x01, 0xE8]).flags());
            assert_eq!(0x42, deserialized(&[0x01, 0x04, 0x01, 0x13, 0x42, 0xAB]).flags());
        }

        #[test]
        fn it_verifies_parity() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[0x01, 0x04, 0x01, 0x13, 0x01, 0x2A];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let result = serializer.deserialize(&mut reader);

            assert!(result.is_err());
            assert_eq!(ErrorKind::Corrupt, result.err().unwrap().kind());
        }

        #[test]
        fn it_handles_short_packets() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[0x01, 0x04, 0x01, 0x13, 0x01];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let result = serializer.deserialize(&mut reader);

            assert!(result.is_err());
            assert_eq!(ErrorKind::ShortRead, result.err().unwrap().kind());
        }
    }
}

mod message_received {
    mod serialize {
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::MessageReceived;

        struct TestParameters {
            callback_id: u8,
            flags: u8,
        }

        const DEFAULT: TestParameters = TestParameters {
            callback_id: 0x11,
            flags: 0x01,
        };

        fn serialized(parameters: TestParameters) -> Vec<u8> {
            let serializer = MessageSerializer::for_response();
            let mut buffer = Vec::<u8>::with_capacity(16);

            serializer.serialize(&MessageReceived::new(parameters.callback_id, parameters.flags), &mut buffer).unwrap();

            buffer
        }

        #[test]
        fn it_serializes_preamble() {
            assert_eq!(0x01, serialized(DEFAULT)[0]);
        }

        #[test]
        fn it_serializes_length() {
            assert_eq!(0x05, serialized(DEFAULT)[1]);
        }

        #[test]
        fn it_serializes_message_type() {
            assert_eq!(0x00, serialized(DEFAULT)[2]);
        }

        #[test]
        fn it_serializes_function_id() {
            assert_eq!(0x13, serialized(DEFAULT)[3]);
        }

        #[test]
        fn it_serializes_callback_id() {
            assert_eq!(DEFAULT.callback_id, serialized(DEFAULT)[4]);

            assert_eq!(0x2A, serialized(TestParameters { callback_id: 0x2A, .. DEFAULT })[4]);
        }

        #[test]
        fn it_serializes_flags() {
            assert_eq!(DEFAULT.flags, serialized(DEFAULT)[5]);

            assert_eq!(0x2A, serialized(TestParameters { flags: 0x2A, .. DEFAULT })[5]);
        }

        #[test]
        fn it_serializes_checksum() {
            assert_eq!(0xF9, serialized(DEFAULT)[6]);

            assert_eq!(0xC2, serialized(TestParameters { callback_id: 0x2A, .. DEFAULT })[6]);
            assert_eq!(0xD2, serialized(TestParameters { flags:       0x2A, .. DEFAULT })[6]);
        }

        #[test]
        fn it_serializes_correct_length() {
            assert_eq!(7, serialized(DEFAULT).len());
        }
    }

    mod deserialize {
        use std::io::Cursor;

        use zwave::core::ErrorKind;
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::MessageReceived;
        use zwave::protocol::serialization::Reader;

        fn deserialized(buffer: &[u8]) -> Box<MessageReceived> {
            let serializer = MessageSerializer::for_response();
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let response = serializer.deserialize(&mut reader).unwrap();

            response.downcast::<MessageReceived>().unwrap()
        }

        #[test]
        fn it_deserializes_callback_id() {
            assert_eq!(0x11, deserialized(&[0x01, 0x05, 0x00, 0x13, 0x11, 0x01, 0xF9]).callback_id());
            assert_eq!(0x2A, deserialized(&[0x01, 0x05, 0x00, 0x13, 0x2A, 0x01, 0xC2]).callback_id());
        }

        #[test]
        fn it_deserializes_flags() {
            assert_eq!(0x01, deserialized(&[0x01, 0x05, 0x00, 0x13, 0x11, 0x01, 0xF9]).flags());
            assert_eq!(0x2A, deserialized(&[0x01, 0x05, 0x00, 0x13, 0x11, 0x2A, 0xD2]).flags());
        }

        #[test]
        fn it_verifies_parity() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[0x01, 0x05, 0x00, 0x13, 0x11, 0x01, 0x2A];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let result = serializer.deserialize(&mut reader);

            assert!(result.is_err());
            assert_eq!(ErrorKind::Corrupt, result.err().unwrap().kind());
        }

        #[test]
        fn it_handles_short_packets() {
            let serializer = MessageSerializer::for_response();
            let buffer = &[0x01, 0x05, 0x00, 0x13, 0x11, 0x01];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let result = serializer.deserialize(&mut reader);

            assert!(result.is_err());
            assert_eq!(ErrorKind::ShortRead, result.err().unwrap().kind());
        }
    }
}
