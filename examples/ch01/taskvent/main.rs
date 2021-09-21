use zmq::{ctx::ZmqContext, message::ZmqMessage, socket::ZmqSocket};

use rand::Rng;

fn main() {
    let ctx = ZmqContext::new();

    let sender = ZmqSocket::new(&ctx, zmq::socket::ZmqSocketType::ZMQ_PUSH);

    sender.bind("tcp://*:5557");

    let sink = ZmqSocket::new(&ctx, zmq::socket::ZmqSocketType::ZMQ_PUSH);

    sink.connect("tcp://localhost:5558");

    sink.send("0".into(), 0).unwrap();

    let mut total_msec = 0;

    let mut rng = rand::thread_rng();

    for i in 0..100 {
        let workload = rng.gen_range(1..100);

        total_msec += workload;
        sink.send(format!("{}", workload).as_bytes().into(), 0)
            .unwrap();
    }

    println!("Total expected cost: {} msec\n", total_msec);
}
