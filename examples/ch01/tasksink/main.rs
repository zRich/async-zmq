use zmq::{
    ctx::ZmqContext,
    message::ZmqMessage,
    socket::{ZmqSocket, ZmqSocketType},
};

// use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let ctx = ZmqContext::new();

    let receiver = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_PULL);

    assert!(receiver.bind("tcp://*:5558").is_ok());

    let mut msg = ZmqMessage::new();
    receiver.recv(&mut msg, 0).unwrap();

    let start_time = Instant::now();
    for i in 0..100 {
        receiver.recv(&mut msg, 0).unwrap();
        if i % 10 == 0 {
            print!(":");
        } else {
            print!(".");
        }

        println!("Message {} = {:?}", i, msg.as_bytes().unwrap());
    }

    // drop(receiver);
    
    let end_time = Instant::now();

    let difference = end_time.duration_since(start_time).as_millis();

    println!("Total elapsed time: {} msec", difference);
}
