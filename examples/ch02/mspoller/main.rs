use zmq::{ctx::ZmqContext, message::ZmqMessage, socket::{ZmqSocket, ZmqSocketType, zmq_poll}};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let ctx = ZmqContext::new();

    let receiver = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_PULL);
    receiver.connect("tcp://localhost::5557");

    let mut subscriber = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_SUB);

    subscriber.connect("tcp://localhost:5556");
    subscriber.setOption( zmq::ZMQ_SUBSCRIBE, b"10001");

    let mut msg = ZmqMessage::new();
    

    loop {
        todo!("poll receiver and subscriber");
        let mut items = [
            receiver.to_zmq_poll_item(ZmqPollEvent::POLLIN),
            subscriber.to_zmq_poll_item(ZmqPollEvent::POLLIN),
        ];

        zmq_poll(&mut items, -1).unwrap();
        if items[0].is_readable() && receiver.recv(&mut msg, 0).is_ok() {
            //process task
        }
        if items[1].is_readable() && subscriber.recv(&mut msg, 0).is_ok() {
            //process weather udpate
        }
    }

}