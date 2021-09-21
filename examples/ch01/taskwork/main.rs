use zmq::{
    ctx::ZmqContext,
    message::ZmqMessage,
    socket::{ZmqSocket, ZmqSocketType},
};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let ctx = ZmqContext::new();

    let receiver = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_PULL);

    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    let sender = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_PUSH);

    assert!(sender.connect("tcp://localhost:5558").is_ok());

    sender.send("0".into(), 0).unwrap();

    let mut msg = ZmqMessage::new();

    loop {
        receiver.recv(&mut msg, 0);

        sleep(Duration::from_millis(1000));

        sender.send("".into(), 0);
        println!("{}", msg);
    }
}
