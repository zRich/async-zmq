use zmq::{
    ctx::ZmqContext,
    message::ZmqMessage,
    socket::{zmq_poll, ZmqPollEvent, ZmqSocket, ZmqSocketType},
};

fn main() {
    let ctx = ZmqContext::new();

    let frontend = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_ROUTER);
    let backend = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_DEALER);

    frontend.bind("tcp://*:5559").unwrap();
    backend.bind("tcp://*:5560").unwrap();

    loop {
        let mut items = [
            frontend.to_zmq_poll_item(ZmqPollEvent::POLLIN),
            backend.to_zmq_poll_item(ZmqPollEvent::POLLIN),
        ];
        zmq_poll(&mut items, -1).unwrap();

        if items[0].is_readable() {
            loop {
                let mut msg = ZmqMessage::new();
                frontend.recv(&mut msg, 0).unwrap();
                let more = msg.get_more();
                let flags = match more {
                    true => zmq::ZMQ_SNDMORE as i32,
                    false => 0,
                };

                backend.send(msg, flags).unwrap();

                if !more {
                    break;
                }
            }
        }

        if items[1].is_readable() {
            loop {
                let mut msg = ZmqMessage::new();

                backend.recv(&mut msg, 0).unwrap();
                let more = msg.get_more();
                let flags = match more {
                    true => zmq::ZMQ_SNDMORE as i32,
                    false => 0,
                };

                frontend.send(msg, flags).unwrap();

                if !more {
                    break;
                }
            }
        }
    }
}
