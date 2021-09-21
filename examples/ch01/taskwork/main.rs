use zmq::{ctx::ZmqContext, message::ZmqMessage, socket::ZmqSocket};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let ctx = ZmqContext::new();

    let receiver = ZmqSocket::new(&ctx, zmq::socket::ZmqSocketType::ZMQ_PULL);

    receiver.bind("tcp://localhost:5557");

    let sender = ZmqSocket::new(&ctx, zmq::socket::ZmqSocketType::ZMQ_PUSH);

    sender.connect("tcp://localhost:5558");

    sender.send("0".into(), 0).unwrap();

    let mut msg = ZmqMessage::new();

    loop {
        receiver.recv(&mut msg, 0);

        sleep(Duration::from_millis(1000));

        sender.send("".into(), 0);
        println!("{}", msg);
    }
}