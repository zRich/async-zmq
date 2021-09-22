use zmq::{ctx::ZmqContext, message::ZmqMessage, socket::ZmqSocket};

use rand::Rng;
use std::io::{self, stdin};

fn main() {
    let ctx = ZmqContext::new();

    let sender = ZmqSocket::new(&ctx, zmq::socket::ZmqSocketType::ZMQ_PUSH);

    sender.bind("tcp://*:5557");

    let sink = ZmqSocket::new(&ctx, zmq::socket::ZmqSocketType::ZMQ_PUSH);

    sink.connect("tcp://localhost:5558");

    println!("Press Enter when the workers are ready: ");
    
    let mut input_string = String::new();
    let mut stdin = io::stdin();

    stdin.read_line(&mut input_string).expect("Failed to read line");

    println!("Sending tasks to workers...\n");

    sink.send("0".into(), 0).unwrap();

    let mut total_msec = 0;

    let mut rng = rand::thread_rng();

    for i in 0..100 {
        let workload = rng.gen_range(1..100);

        total_msec += workload;
        sender.send(format!("{}", workload).as_bytes().into(), 0)
            .unwrap();
    }

    println!("Total expected cost: {} msec", total_msec);
}
