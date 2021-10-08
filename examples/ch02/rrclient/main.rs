use zmq::{ctx::ZmqContext, message::ZmqMessage, socket::{ZmqSocket, ZmqSocketType}};

fn main() {
    let ctx = ZmqContext::new();

    let requester = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_REQ);
    requester.connect("tcp://localhost:5559").unwrap();

    let mut msg = ZmqMessage::new();
    for i in 0..10 {
        requester.send(ZmqMessage::from("Hello"), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received reply {} [{}]", i, msg.as_str().unwrap());
    }
}