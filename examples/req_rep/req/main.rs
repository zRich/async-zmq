#![crate_name = "req_client"]

use zmq::{
    ctx::{ZmqContext},
    socket::{ZmqSocket, ZmqSocketType},
    error::{ZmqError, ZmqResult},
    message::{ZmqMessage}
};

use std::thread;
use std::time::Duration;

pub(crate) fn main() {
    let ctx = ZmqContext::new();

    let sock = ZmqSocket::new(ctx, ZmqSocketType::ZMQ_REQ);

    assert!(sock.bind("tcp://localhost:5555").is_ok());

    let mut msg = ZmqMessage::new();

    for n in 0..10 {
        println!("Sending Hello {}...", n);
        sock.send( "Hello".into(), 0).unwrap();

        sock.recv(&mut msg, 0).unwrap();
        println!("Received world {}: {}", msg.as_str().unwrap(), n);
    }
}