#![crate_name = "helloworld_client"]

use zmq::{
    ctx::{ZmqContext},
    socket::{ZmqSocket, ZmqSocketType},
    error::{ZmqError, ZmqResult},
    message::{ZmqMessage}
};

use std::thread;
use std::time::Duration;

fn main() {
    println!("Connecting to hello world server...");
    let ctx = ZmqContext::new();
    let sock = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_REQ);
    // let sock = ctx.socket(ZmqSocketType::ZMQ_REQ);

    assert!(sock.connect("tcp://localhost:5556").is_ok());

    let mut msg = ZmqMessage::new();

    for n in 0..10 {
        println!("Sending Hello {}...", n);
        sock.send( ZmqMessage::from("Hello"), 0).unwrap();

        sock.recv(&mut msg, 0).unwrap();
        println!("Received world {}: {}", msg.as_str().unwrap(), n);
    }
}