mod message;

pub use message::*;


#[allow(unused_imports)]
use zmq::{Context, Error, Socket, SocketEvent, SocketType};

use futures;

type ZmqSocketType = SocketType;
// pub enum ZmqSocketType {
//     PAIR,
//     PUB,
//     SUB,
//     REQ,
//     REP,
//     DEALER,
//     ROUTER,
//     PULL,
//     PUSH,
//     XPUB,
//     XSUB,
//     STREAM,
// }

type ZmqError = zmq::Error;

type ZmqResult<T> = Result<T, ZmqError>;

pub struct ZmqContext {
    ctx: zmq::Context,
}

impl ZmqContext {
    pub fn new() -> Self {
        Self {
            ctx: zmq::Context::new(),
        }
    }

    pub fn Socket(&self, socket_type: ZmqSocketType) -> Result<ZmqSocket, ZmqError> {
        self.Socket(socket_type)
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

pub trait ZmqEvent {
    fn r#do(&mut self) -> ZmqResult<()>;
}
pub trait ZmqSend {
    fn send(&mut self, message: ZmqMessage) -> ZmqResult<()>;
 }

 pub trait ZmqRecv {
    fn recv(&mut self) -> ZmqResult<ZmqMessage>;
 }
