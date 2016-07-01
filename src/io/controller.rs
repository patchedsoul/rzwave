use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use core::{self, NodeId, Error, ErrorKind};
use io::driver::Driver;
use protocol::bits::PreambleId;
use protocol::command::Command;
use protocol::message::{Ack, SendData};

const REPLY_TIMEOUT_MS: u64 = 100;

enum Reply {
    Ack,
    Nack,
    Cancel,
}

struct SharedState<D: Driver> {
    driver: Mutex<D>,
    running: AtomicBool,
    reply: Condvar,
}

pub struct Controller<D: Driver> {
    state: Arc<SharedState<D>>,
    replies: Receiver<Reply>,
    thread: thread::JoinHandle<()>,
}

impl<D: Driver> Controller<D> {
    pub fn new(driver: D) -> Self {
        let state = Arc::new(SharedState {
            driver: Mutex::new(driver),
            running: AtomicBool::new(true),
            reply: Condvar::new(),
        });


        let (tx, rx) = channel::<Reply>();
        let thread = Reader::start(state.clone(), tx);

        Controller {
            state: state,
            replies: rx,
            thread: thread,
        }
    }

    pub fn stop(self) {
        self.state.running.store(false, Ordering::Relaxed);
        self.thread.join().unwrap();
    }

    pub fn send_data<C: Command>(&mut self, node_id: NodeId, command: C) -> core::Result<()> {
        let message = SendData::new(node_id, command, 0x11);

        let mut driver = self.state.driver.lock().unwrap();

        // clear missed replies from previous messages
        while self.replies.try_recv().is_ok() { }

        try!(driver.send(&message));

        let mut timeout = Duration::from_millis(REPLY_TIMEOUT_MS);
        let deadline = Instant::now() + timeout;

        loop {
            let (guard, wait) = self.state.reply.wait_timeout(driver, timeout).unwrap();

            driver = guard;

            match self.replies.try_recv() {
                Ok(Reply::Ack) => return Ok(()),
                Ok(Reply::Nack) => return Err(Error::new(ErrorKind::Nack)),
                Ok(Reply::Cancel) => return Err(Error::new(ErrorKind::Cancel)),
                Err(TryRecvError::Disconnected) => return Err(Error::new(ErrorKind::Timeout)),
                Err(TryRecvError::Empty) => {
                    if wait.timed_out() {
                        return Err(Error::new(ErrorKind::Timeout));
                    }

                    // could be spurious wakeup
                    let now = Instant::now();

                    if now > deadline {
                        return Err(Error::new(ErrorKind::Timeout));
                    }

                    timeout = deadline - now;
                },
            }
        }
    }
}

struct Reader<D: Driver> {
    state: Arc<SharedState<D>>,
    replies: Sender<Reply>,
}

impl<D: Driver> Reader<D> {
    fn start(state: Arc<SharedState<D>>, sender: Sender<Reply>) -> JoinHandle<()> {
        thread::spawn(move || {
            Reader::new(state, sender).run()
        })
    }

    fn new(state: Arc<SharedState<D>>, replies: Sender<Reply>) -> Self {
        Reader {
            state: state,
            replies: replies,
        }
    }

    fn run(&self) {
        while self.state.running.load(Ordering::Relaxed) {
            let mut driver = self.state.driver.lock().unwrap();

            match driver.receive() {
                Ok(message) => {
                    match message.preamble_id() {
                        PreambleId::Ack => {
                            self.replies.send(Reply::Ack).unwrap();
                            self.state.reply.notify_one();
                        }
                        PreambleId::Nack => {
                            self.replies.send(Reply::Nack).unwrap();
                            self.state.reply.notify_one();
                        }
                        PreambleId::Cancel => {
                            self.replies.send(Reply::Cancel).unwrap();
                            self.state.reply.notify_one();
                        }
                        PreambleId::Frame => {
                            // TODO: handle error and packet
                            driver.send(&Ack::new()).unwrap();
                        },
                    }
                },
                Err(err) => {
                    if err.kind() != ErrorKind::Timeout {
                        panic!("error = {:?}", err);
                    }
                },
            }
        }
    }
}
