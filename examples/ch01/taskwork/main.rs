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
        receiver.recv(&mut msg, 0).unwrap();

        let msec: u64 = msg.as_str().unwrap().parse().unwrap();

        // println!("Message = {:?}", msec);

        sleep(Duration::from_millis(msec));

        sender.send("".into(), 0).unwrap();
        // println!("{:?}", msg.as_bytes().unwrap());
    }
}
