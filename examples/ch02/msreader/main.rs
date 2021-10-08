use zmq::{
    ctx::ZmqContext,
    message::ZmqMessage,
    socket::{ZmqSocket, ZmqSocketType},
    ZMQ_DONTWAIT, ZMQ_SUBSCRIBE,
};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let ctx = ZmqContext::new();

    let receiver = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_PULL);
    receiver.connect("tcp://localhost::5557");

    let mut subscriber = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_SUB);

    subscriber.connect("tcp://localhost:5556");
    subscriber.setOption(ZMQ_SUBSCRIBE, b"10001 ");

    let mut msg = ZmqMessage::new();
    loop {

        loop {
            receiver.recv(&mut msg, ZMQ_DONTWAIT as i32);
            if !msg.is_empty() {
                todo!();
            } else {
                break;
            }
        }

        loop {
            subscriber.recv(&mut msg, ZMQ_DONTWAIT as i32);
            if !msg.is_empty() {
                todo!();
            } else {
                break;
            }
        }

        sleep(Duration::from_millis(1000));
    }
}
