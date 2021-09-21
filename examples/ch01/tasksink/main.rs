use zmq::{ctx::ZmqContext, message::ZmqMessage, socket::ZmqSocket};

use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

fn main() {
    let ctx = ZmqContext::new();

    let receiver = ZmqSocket::new(&ctx, zmq::socket::ZmqSocketType::ZMQ_PULL);

    receiver.bind("tcp://localhost:5558");

    let sender = ZmqSocket::new(&ctx, zmq::socket::ZmqSocketType::ZMQ_PUSH);

    sender.connect("tcp://localhost:5558");

    sender.send("0".into(), 0).unwrap();

    let mut msg = ZmqMessage::new();

    let start_time = Instant::now();
    for i in 0..100 {
        receiver.recv(&mut msg, 0);
        if i % 10 == 0 {
            print!(":");
        } else {
            print!(".");
        }
    }

    let end_time = Instant::now();

    let difference = end_time.duration_since(start_time).as_millis();

    println!("Total elapsed time: {} msec", difference);
}
