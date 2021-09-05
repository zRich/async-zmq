#![crate_name = "helloworld_server"]

use zmq::{
    ctx::{ZmqContext},
    socket::{ZmqSocket, ZmqSocketType},
    error::{ZmqError, ZmqResult},
    message::{ZmqMessage}
};

use std::thread;
use std::time::Duration;

fn main() {
    let ctx = ZmqContext::new();
    let sock = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_REP);
    
    assert!(sock.bind("tcp://*:5555").is_ok());
    println!("hello world server started...");
    let mut msg = ZmqMessage::new();

    loop {
        sock.recv(&mut msg, 0).unwrap();
        println!("Received {}.", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        sock.send("World".into(), 0).unwrap();
    }
}