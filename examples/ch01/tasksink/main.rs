use zmq::{
    ctx::ZmqContext,
    message::ZmqMessage,
    socket::{ZmqSocket, ZmqSocketType},
};

use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

fn main() {
    let ctx = ZmqContext::new();

    let receiver = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_PULL);

    receiver.bind("tcp://*:5558");

    let mut msg = ZmqMessage::new();
    receiver.recv(&mut msg, 0);

    let start_time = Instant::now();
    for i in 0..100 {
        receiver.recv(&mut msg, 0);
        if i % 10 == 0 {
            print!(":");
        } else {
            print!(".");
        }

        println!("{}", String::from(msg.as_bytes()));
    }

    let end_time = Instant::now();

    let difference = end_time.duration_since(start_time).as_millis();

    println!("Total elapsed time: {} msec", difference);
}
