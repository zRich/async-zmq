use zmq::{ctx::ZmqContext, message::ZmqMessage, socket::{ZmqSocket, ZmqSocketType}};

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let ctx = ZmqContext::new();

    let responder = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_REP);
    responder.connect("tcp://localhost:5560").unwrap();

    let mut msg = ZmqMessage::new();
    loop {
        responder.recv(&mut msg , 0).unwrap();
        println!("Received request: [{}]", msg.as_str().unwrap());

        sleep(Duration::from_millis(1000));       
        responder.send(ZmqMessage::from("World"), 0).unwrap();
    }
}