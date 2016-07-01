extern crate zwave;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

use zwave::core::{self, Error, ErrorKind};
use zwave::protocol::message::{Message, AnyMessage};
use zwave::io::driver::Driver;

struct DriverMock {
    send: VecDeque<(Box<Fn(&Message) -> core::Result<()> + Send>, Option<core::Result<AnyMessage>>)>,
    receive: VecDeque<core::Result<AnyMessage>>,
}

impl DriverMock {
    fn new() -> Self {
        DriverMock {
            send: VecDeque::<(Box<Fn(&Message) -> core::Result<()> + Send>, Option<core::Result<AnyMessage>>)>::new(),
            receive: VecDeque::<core::Result<AnyMessage>>::new(),
        }
    }
}

#[derive(Clone)]
struct FakeDriver {
    mock: Arc<Mutex<DriverMock>>,
    receive_cond: Arc<Condvar>,
}

impl FakeDriver {
    fn new() -> Self {
        FakeDriver {
            mock: Arc::new(Mutex::new(DriverMock::new())),
            receive_cond: Arc::new(Condvar::new()),
        }
    }

    fn expect_send<F: Fn(&Message) -> core::Result<()> + Send + 'static>(&mut self, f: F) {
        let mut mock = self.mock.lock().unwrap();
        mock.send.push_back((Box::new(f), None));
    }

    fn expect_send_with_response<F: Fn(&Message) -> core::Result<()> + Send + 'static>(&mut self, f: F, response: core::Result<AnyMessage>) {
        let mut mock = self.mock.lock().unwrap();
        mock.send.push_back((Box::new(f), Some(response)));
    }

    fn push_response(&mut self, response: core::Result<AnyMessage>) {
        let mut mock = self.mock.lock().unwrap();
        mock.receive.push_back(response);
    }

    fn wait_for_receive(&mut self) {
        let mut mock = self.mock.lock().unwrap();

        while mock.receive.len() != 0 {
            mock = self.receive_cond.wait(mock).unwrap();
        }
    }

    fn verify(&self) {
        let mock = self.mock.lock().unwrap();

        let missing_calls = mock.send.len();

        if missing_calls != 0 {
            panic!("missing {} expected call(s) to send()", missing_calls);
        }
    }
}

impl Driver for FakeDriver {
    fn send(&mut self, message: &Message) -> core::Result<()> {
        let mut mock = self.mock.lock().unwrap();
        let (f, response) = mock.send.pop_front().expect("unexpected call to send()");

        let retval = f(message);

        if let Some(res) = response {
            mock.receive.push_back(res);
        }

        retval
    }

    fn receive(&mut self) -> core::Result<AnyMessage> {
        let mut mock = self.mock.lock().unwrap();

        match mock.receive.pop_front() {
            Some(value) => {
                self.receive_cond.notify_one();

                value
            },
            None => {
                thread::sleep(Duration::from_millis(1));
                Err(Error::new(ErrorKind::Timeout))
            }
        }
    }
}

mod send_data {
    use std::thread;
    use std::time::Duration;

    use zwave::core::{NodeId, Error, ErrorKind};
    use zwave::protocol::message::{AnyMessage, Ack, Nack, Cancel, SendData};
    use zwave::protocol::command::basic::SetValue;
    use zwave::io::controller::Controller;

    use super::FakeDriver;

    fn with_fake_driver<F: FnOnce(&mut FakeDriver, &mut Controller<FakeDriver>) -> ()>(f: F) {
        let mut driver = FakeDriver::new();
        let mut controller = Controller::new(driver.clone());

        f(&mut driver, &mut controller);

        controller.stop();
        driver.verify();
    }

    #[test]
    fn it_sends_send_data_message() {
        with_fake_driver(|driver, controller| {
            driver.expect_send_with_response(|message| {
                assert!(message.is::<SendData>());
                Ok(())
            }, Ok(AnyMessage::new(Ack::new())));

            let _ = controller.send_data(NodeId(42), SetValue::new(42));
        });
    }

    #[test]
    fn it_sends_correct_node_id() {
        with_fake_driver(|driver, controller| {
            driver.expect_send_with_response(|message| {
                let send_data = message.downcast_ref::<SendData>().unwrap();
                assert_eq!(NodeId(42), send_data.destination());
                Ok(())
            }, Ok(AnyMessage::new(Ack::new())));

            let _ = controller.send_data(NodeId(42), SetValue::new(42));

            driver.expect_send_with_response(|message| {
                let send_data = message.downcast_ref::<SendData>().unwrap();
                assert_eq!(NodeId(7), send_data.destination());
                Ok(())
            }, Ok(AnyMessage::new(Ack::new())));

            let _ = controller.send_data(NodeId(7), SetValue::new(42));
        });
    }

    #[test]
    fn it_sends_command_as_payload() {
        with_fake_driver(|driver, controller| {
            driver.expect_send_with_response(|message| {
                let send_data = message.downcast_ref::<SendData>().unwrap();

                assert!(send_data.command().is::<SetValue>());
                let command = send_data.command().downcast_ref::<SetValue>().unwrap();

                assert_eq!(42, command.value());
                Ok(())
            }, Ok(AnyMessage::new(Ack::new())));

            let _ = controller.send_data(NodeId(42), SetValue::new(42));

            driver.expect_send_with_response(|message| {
                let send_data = message.downcast_ref::<SendData>().unwrap();

                assert!(send_data.command().is::<SetValue>());
                let command = send_data.command().downcast_ref::<SetValue>().unwrap();

                assert_eq!(7, command.value());
                Ok(())
            }, Ok(AnyMessage::new(Ack::new())));

            let _ = controller.send_data(NodeId(42), SetValue::new(7));
        });
    }

    #[test]
    fn it_returns_ok_if_reply_is_ack() {
        with_fake_driver(|driver, controller| {
            driver.expect_send_with_response(|_| {
                Ok(())
            }, Ok(AnyMessage::new(Ack::new())));

            assert_eq!(Ok(()), controller.send_data(NodeId(42), SetValue::new(42)));
        });
    }

    #[test]
    fn it_returns_nack_error_if_reply_is_nack() {
        with_fake_driver(|driver, controller| {
            driver.expect_send_with_response(|_| {
                Ok(())
            }, Ok(AnyMessage::new(Nack::new())));

            assert_eq!(Err(Error::new(ErrorKind::Nack)), controller.send_data(NodeId(42), SetValue::new(42)));
        });
    }

    #[test]
    fn it_returns_cancel_error_if_reply_is_cancel() {
        with_fake_driver(|driver, controller| {
            driver.expect_send_with_response(|_| {
                Ok(())
            }, Ok(AnyMessage::new(Cancel::new())));

            assert_eq!(Err(Error::new(ErrorKind::Cancel)), controller.send_data(NodeId(42), SetValue::new(42)));
        });
    }

    #[test]
    fn it_returns_timeout_error_if_no_response_is_received() {
        with_fake_driver(|driver, controller| {
            driver.expect_send(|_| { Ok(()) });
            assert_eq!(Err(Error::new(ErrorKind::Timeout)), controller.send_data(NodeId(42), SetValue::new(42)));
        });
    }

    #[test]
    fn it_ignores_replies_from_previous_timed_out_requests() {
        with_fake_driver(|driver, controller| {
            driver.expect_send(|_| { Ok(()) });
            let _ = controller.send_data(NodeId(42), SetValue::new(42));

            driver.push_response(Ok(AnyMessage::new(Cancel::new())));
            driver.wait_for_receive();

            driver.expect_send_with_response(|_| {
                Ok(())
            }, Ok(AnyMessage::new(Ack::new())));

            assert_eq!(Ok(()), controller.send_data(NodeId(42), SetValue::new(42)));
        });
    }
}
