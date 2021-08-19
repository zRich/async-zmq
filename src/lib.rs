mod message;
mod error;
mod req;

pub use message::*;
use async_trait::async_trait;

#[allow(unused_imports)]
use zmq::{Context, Error, Socket, SocketEvent, SocketType};

use error::{ZmqError, ZmqResult};

use futures::{self, Future};

pub enum ZmqSocketType {
    PAIR = SocketType::PAIR,
    PUB = SocketType::PUB,
    SUB = SocketType::SUB,
    REQ = SocketType::REQ,
    REP = SocketType::REP,
    DEALER = SocketType::DEALER,
    ROUTER = SocketType::ROUTER,
    PULL = SocketType::PULL,
    PUSH = SocketType::PUSH,
    XPUB = SocketType::XPUB,
    XSUB = SocketType::XSUB,
    STREAM = SocketType::STREAM,
}

pub struct ZmqContext {
    ctx: zmq::Context,
}

impl ZmqContext {
    pub fn new() -> Self {
        Self {
            ctx: zmq::Context::new(),
        }
    }

    pub fn Socket(&self, socket_type: ZmqSocketType) -> ZmqResult<()> {
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

pub trait Sendable {
    fn send(self, socket: &ZmqSocket, flags: i32) -> ZmqResult<()>;
}

impl<T> Sendable for T
where 
    T: Into<ZmqMessage>,
{
    fn send(self, socket: &ZmqSocket, flags: i32) -> ZmqResult<()> {
        let mut msg: ZmqMessage = self.into();

    }
}



#[async_trait]
pub trait ZmqSend {
    async fn send(&mut self, message: ZmqMessage); 
 }

#[async_trait]
 pub trait ZmqRecv {
    async fn recv(&mut self) -> ZmqResult<ZmqMessage>;
 }
