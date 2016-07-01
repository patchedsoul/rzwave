// vim: set foldmethod=syntax foldlevel=1 :

extern crate zwave;

mod ack {
    mod serialize {
        use zwave::protocol::message::MessageSerializer;
        use zwave::protocol::message::Ack;

        fn serialized() -> Vec<u8> {
            let serializer = MessageSerializer::for_request();
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
            let serializer = MessageSerializer::for_request();
            let buffer = &[0x06];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let request = serializer.deserialize(&mut reader).unwrap();

            assert!(request.is::<Ack>());
        }

        #[test]
        fn it_handles_short_packets() {
            let serializer = MessageSerializer::for_request();
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
            let serializer = MessageSerializer::for_request();
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
            let serializer = MessageSerializer::for_request();
            let buffer = &[0x15];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let request = serializer.deserialize(&mut reader).unwrap();

            assert!(request.is::<Nack>());
        }

        #[test]
        fn it_handles_short_packets() {
            let serializer = MessageSerializer::for_request();
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
            let serializer = MessageSerializer::for_request();
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
            let serializer = MessageSerializer::for_request();
            let buffer = &[0x18];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let request = serializer.deserialize(&mut reader).unwrap();

            assert!(request.is::<Cancel>());
        }

        #[test]
        fn it_handles_short_packets() {
            let serializer = MessageSerializer::for_request();
            let buffer = &[];
            let mut cursor = Cursor::new(buffer);
            let mut reader = Reader::new(&mut cursor);
            let result = serializer.deserialize(&mut reader);

            assert!(result.is_err());
            assert_eq!(ErrorKind::ShortRead, result.err().unwrap().kind());
        }
    }
}

mod send_data {
    mod set_value {
        mod serialize {
            use zwave::core::NodeId;

            struct TestParameters {
                node_id: NodeId,
                value: u8,
                packet_options: u8,
                callback_id: u8,
            }

            const DEFAULT: TestParameters = TestParameters {
                node_id: NodeId(2),
                value: 42,
                packet_options: 0x05,
                callback_id: 0x11,
            };

            fn serialized(parameters: TestParameters) -> Vec<u8> {
                use zwave::protocol::message::MessageSerializer;
                use zwave::protocol::message::SendData;
                use zwave::protocol::command::basic::SetValue;

                let serializer = MessageSerializer::for_request();
                let mut buffer = Vec::<u8>::with_capacity(16);

                let request = SendData::with_options(parameters.node_id, SetValue::new(parameters.value), parameters.callback_id, parameters.packet_options);

                serializer.serialize(&request, &mut buffer).unwrap();

                buffer
            }

            #[test]
            fn it_serializes_preamble() {
                assert_eq!(0x01, serialized(DEFAULT)[0]);
            }

            #[test]
            fn it_serializes_length() {
                assert_eq!(0x0A, serialized(DEFAULT)[1]);
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
            fn it_serializes_destination() {
                assert_eq!(DEFAULT.node_id.value(), serialized(DEFAULT)[4]);

                assert_eq!(0x00, serialized(TestParameters { node_id: NodeId(0),   .. DEFAULT })[4]);
                assert_eq!(0x2A, serialized(TestParameters { node_id: NodeId(42),  .. DEFAULT })[4]);
                assert_eq!(0xFF, serialized(TestParameters { node_id: NodeId(255), .. DEFAULT })[4]);
            }

            #[test]
            fn it_serializes_payload_length() {
                assert_eq!(0x03, serialized(DEFAULT)[5]);
            }

            #[test]
            fn it_serializes_command_class_id() {
                assert_eq!(0x20, serialized(DEFAULT)[6]);
            }

            #[test]
            fn it_serializes_command_id() {
                assert_eq!(0x01, serialized(DEFAULT)[7]);
            }

            #[test]
            fn it_serializes_command_payload() {
                assert_eq!(DEFAULT.value, serialized(DEFAULT)[8]);

                assert_eq!(0x00, serialized(TestParameters { value: 0,   .. DEFAULT })[8]);
                assert_eq!(0xFF, serialized(TestParameters { value: 255, .. DEFAULT })[8]);
            }

            #[test]
            fn it_serializes_packet_options() {
                assert_eq!(DEFAULT.packet_options, serialized(DEFAULT)[9]);

                assert_eq!(0x2A, serialized(TestParameters { packet_options: 0x2A, .. DEFAULT })[9]);
            }

            #[test]
            fn it_serializes_callback_id() {
                assert_eq!(DEFAULT.callback_id, serialized(DEFAULT)[10]);

                assert_eq!(0x2A, serialized(TestParameters { callback_id: 42, .. DEFAULT })[10]);
            }

            #[test]
            fn it_serializes_checksum() {
                assert_eq!(0xF8, serialized(DEFAULT)[11]);

                assert_eq!(0xD0, serialized(TestParameters { node_id:        NodeId(42), .. DEFAULT })[11]);
                assert_eq!(0xD2, serialized(TestParameters { value:          0,          .. DEFAULT })[11]);
                assert_eq!(0xD7, serialized(TestParameters { packet_options: 0x2A,       .. DEFAULT })[11]);
                assert_eq!(0xC3, serialized(TestParameters { callback_id:    42,         .. DEFAULT })[11]);
            }

            #[test]
            fn it_serializes_correct_length() {
                assert_eq!(12, serialized(DEFAULT).len());
            }
        }

        mod deserialize {
            use std::io::Cursor;

            use zwave::core::{NodeId, ErrorKind};
            use zwave::protocol::message::MessageSerializer;
            use zwave::protocol::message::SendData;
            use zwave::protocol::command::basic::SetValue;
            use zwave::protocol::serialization::Reader;

            fn deserialized(buffer: &[u8]) -> Box<SendData> {
                let serializer = MessageSerializer::for_request();
                let mut cursor = Cursor::new(buffer);
                let mut reader = Reader::new(&mut cursor);
                let request = serializer.deserialize(&mut reader).unwrap();

                request.downcast::<SendData>().unwrap()
            }

            #[test]
            fn it_deserializes_destination() {
                assert_eq!(NodeId(2),  deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11, 0xF8]).destination());
                assert_eq!(NodeId(42), deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x2A, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11, 0xD0]).destination());
            }

            #[test]
            fn it_deserializes_command_type() {
                assert!(deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11, 0xF8]).command().is::<SetValue>());
            }

            #[test]
            fn it_deserializes_command_payload() {
                assert_eq!(42,  deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11, 0xF8]).command().downcast_ref::<SetValue>().unwrap().value());
                assert_eq!(255, deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0xFF, 0x05, 0x11, 0x2D]).command().downcast_ref::<SetValue>().unwrap().value());
            }

            #[test]
            fn it_deserializes_packet_options() {
                assert_eq!(0x05, deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11, 0xF8]).packet_options());
                assert_eq!(0x2A, deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x2A, 0x11, 0xD7]).packet_options());
            }

            #[test]
            fn it_deserializes_callback_id() {
                assert_eq!(17, deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11, 0xF8]).callback_id());
                assert_eq!(42, deserialized(&[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x2A, 0xC3]).callback_id());
            }

            #[test]
            fn it_verifies_parity() {
                let serializer = MessageSerializer::for_request();
                let buffer = &[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11, 0x2A];
                let mut cursor = Cursor::new(buffer);
                let mut reader = Reader::new(&mut cursor);
                let result = serializer.deserialize(&mut reader);

                assert!(result.is_err());
                assert_eq!(ErrorKind::Corrupt, result.err().unwrap().kind());
            }

            #[test]
            fn it_handles_short_packets() {
                let serializer = MessageSerializer::for_request();
                let buffer = &[0x01, 0x0A, 0x00, 0x13, 0x02, 0x03, 0x20, 0x01, 0x2A, 0x05, 0x11];
                let mut cursor = Cursor::new(buffer);
                let mut reader = Reader::new(&mut cursor);
                let result = serializer.deserialize(&mut reader);

                assert!(result.is_err());
                assert_eq!(ErrorKind::ShortRead, result.err().unwrap().kind());
            }
        }
    }

    mod get_value {
        mod serialize {
            use zwave::core::NodeId;

            struct TestParameters {
                node_id: NodeId,
                packet_options: u8,
                callback_id: u8,
            }

            const DEFAULT: TestParameters = TestParameters {
                node_id: NodeId(2),
                packet_options: 0x05,
                callback_id: 0x11,
            };

            fn serialized(parameters: TestParameters) -> Vec<u8> {
                use zwave::protocol::message::MessageSerializer;
                use zwave::protocol::message::SendData;
                use zwave::protocol::command::basic::GetValue;

                let serializer = MessageSerializer::for_request();
                let mut buffer = Vec::<u8>::with_capacity(16);

                let request = SendData::with_options(parameters.node_id, GetValue::new(), parameters.callback_id, parameters.packet_options);

                serializer.serialize(&request, &mut buffer).unwrap();

                buffer
            }

            #[test]
            fn it_serializes_preamble() {
                assert_eq!(0x01, serialized(DEFAULT)[0]);
            }

            #[test]
            fn it_serializes_length() {
                assert_eq!(0x09, serialized(DEFAULT)[1]);
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
            fn it_serializes_destination() {
                assert_eq!(DEFAULT.node_id.value(), serialized(DEFAULT)[4]);

                assert_eq!(0x00, serialized(TestParameters { node_id: NodeId(0),   .. DEFAULT })[4]);
                assert_eq!(0x2A, serialized(TestParameters { node_id: NodeId(42),  .. DEFAULT })[4]);
                assert_eq!(0xFF, serialized(TestParameters { node_id: NodeId(255), .. DEFAULT })[4]);
            }

            #[test]
            fn it_serializes_payload_length() {
                assert_eq!(0x02, serialized(DEFAULT)[5]);
            }

            #[test]
            fn it_serializes_command_class_id() {
                assert_eq!(0x20, serialized(DEFAULT)[6]);
            }

            #[test]
            fn it_serializes_command_id() {
                assert_eq!(0x02, serialized(DEFAULT)[7]);
            }

            #[test]
            fn it_serializes_packet_options() {
                assert_eq!(DEFAULT.packet_options, serialized(DEFAULT)[8]);

                assert_eq!(0x2A, serialized(TestParameters { packet_options: 0x2A, .. DEFAULT })[8]);
            }

            #[test]
            fn it_serializes_callback_id() {
                assert_eq!(DEFAULT.callback_id, serialized(DEFAULT)[9]);

                assert_eq!(0x2A, serialized(TestParameters { callback_id: 42, .. DEFAULT })[9]);
            }

            #[test]
            fn it_serializes_checksum() {
                assert_eq!(0xD3, serialized(DEFAULT)[10]);

                assert_eq!(0xFB, serialized(TestParameters { node_id:        NodeId(42), .. DEFAULT })[10]);
                assert_eq!(0xFC, serialized(TestParameters { packet_options: 0x2A,       .. DEFAULT })[10]);
                assert_eq!(0xE8, serialized(TestParameters { callback_id:    42,         .. DEFAULT })[10]);
            }

            #[test]
            fn it_serializes_correct_length() {
                assert_eq!(11, serialized(DEFAULT).len());
            }
        }

        mod deserialize {
            use std::io::Cursor;

            use zwave::core::{NodeId, ErrorKind};
            use zwave::protocol::message::MessageSerializer;
            use zwave::protocol::message::SendData;
            use zwave::protocol::command::basic::GetValue;
            use zwave::protocol::serialization::Reader;

            fn deserialized(buffer: &[u8]) -> Box<SendData> {
                let serializer = MessageSerializer::for_request();
                let mut cursor = Cursor::new(buffer);
                let mut reader = Reader::new(&mut cursor);
                let request = serializer.deserialize(&mut reader).unwrap();

                request.downcast::<SendData>().unwrap()
            }

            #[test]
            fn it_deserializes_destination() {
                assert_eq!(NodeId(2),  deserialized(&[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x05, 0x11, 0xD3]).destination());
                assert_eq!(NodeId(42), deserialized(&[0x01, 0x09, 0x00, 0x13, 0x2A, 0x02, 0x20, 0x02, 0x05, 0x11, 0xFB]).destination());
            }

            #[test]
            fn it_deserializes_command_type() {
                assert!(deserialized(&[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x05, 0x11, 0xD3]).command().is::<GetValue>());
            }

            #[test]
            fn it_deserializes_packet_options() {
                assert_eq!(0x05, deserialized(&[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x05, 0x11, 0xD3]).packet_options());
                assert_eq!(0x2A, deserialized(&[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x2A, 0x11, 0xFC]).packet_options());
            }

            #[test]
            fn it_deserializes_callback_id() {
                assert_eq!(17, deserialized(&[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x05, 0x11, 0xD3]).callback_id());
                assert_eq!(42, deserialized(&[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x05, 0x2A, 0xE8]).callback_id());
            }

            #[test]
            fn it_verifies_parity() {
                let serializer = MessageSerializer::for_request();
                let buffer = &[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x05, 0x11, 0x2A];
                let mut cursor = Cursor::new(buffer);
                let mut reader = Reader::new(&mut cursor);
                let result = serializer.deserialize(&mut reader);

                assert!(result.is_err());
                assert_eq!(ErrorKind::Corrupt, result.err().unwrap().kind());
            }

            #[test]
            fn it_handles_short_packets() {
                let serializer = MessageSerializer::for_request();
                let buffer = &[0x01, 0x09, 0x00, 0x13, 0x02, 0x02, 0x20, 0x02, 0x05, 0x11];
                let mut cursor = Cursor::new(buffer);
                let mut reader = Reader::new(&mut cursor);
                let result = serializer.deserialize(&mut reader);

                assert!(result.is_err());
                assert_eq!(ErrorKind::ShortRead, result.err().unwrap().kind());
            }
        }
    }
}
