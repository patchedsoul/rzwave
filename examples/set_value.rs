extern crate serial;
extern crate zwave;

use std::env;

use zwave::core::NodeId;
use zwave::protocol::command::basic::SetValue;
use zwave::io::driver::SerialDriver;
use zwave::io::controller::Controller;

fn main() {
    let node_id = NodeId(2);
    let command = SetValue::new(4);

    for arg in env::args_os().skip(1).take(1) {
        let port = serial::open(&arg).unwrap();
        let driver = SerialDriver::new(port).unwrap();
        let mut controller = Controller::new(driver);

        controller.send_data(node_id, command).unwrap();
        controller.stop();
    }
}
