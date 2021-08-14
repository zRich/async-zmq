#[allow(unused_imports)]
use zmq;

pub enum ZmqSocketType {
    PAIR,
    PUB,
    SUB,
    REQ,
    REP,
    DEALER,
    ROUTER,
    PULL,
    PUSH,
    XPUB,
    XSUB,
    STREAM,
}

pub struct ZmqContext;

impl ZmqContext {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct ZmqSocket {
    ctx: ZmqContext,
    socket_type: ZmqSocketType,
}

impl ZmqSocket {
    fn new(ctx: ZmqContext, socket_type: ZmqSocketType) -> Self {
        Self { ctx, socket_type }
    }
}
