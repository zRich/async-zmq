use crate::ctx::ZmqContext;
use crate::error::{ZmqError, ZmqResult};

use crate::zmq;

use std::convert::{From, Into};
use std::os::raw::c_void;

pub enum ZmqSocketType {
    ZMQ_PAIR = 0,
    ZMQ_PUB = 1,
    ZMQ_SUB = 2,
    ZMQ_REQ = 3,
    ZMQ_REP = 4,
    ZMQ_DEALER = 5,
    ZMQ_ROUTER = 6,
    ZMQ_PULL = 7,
    ZMQ_PUSH = 8,
    ZMQ_XPUB = 9,
    ZMQ_XSUB = 10,
    ZMQ_STREAM = 11,
    // ZMQ_XREQ =  5,
    // ZMQ_XREP =  6,
}

impl Into<i32> for ZmqSocketType {
    fn into(self) -> i32 {
        match self {
            ZmqSocketType::ZMQ_PAIR => 0,
            ZmqSocketType::ZMQ_PUB => 1,
            ZmqSocketType::ZMQ_SUB => 2,
            ZmqSocketType::ZMQ_REQ => 3,
            ZmqSocketType::ZMQ_REP => 4,
            ZmqSocketType::ZMQ_DEALER => 5,
            ZmqSocketType::ZMQ_ROUTER => 6,
            ZmqSocketType::ZMQ_PULL => 7,
            ZmqSocketType::ZMQ_PUSH => 8,
            ZmqSocketType::ZMQ_XPUB => 9,
            ZmqSocketType::ZMQ_XSUB => 10,
            ZmqSocketType::ZMQ_STREAM => 11,
        }
    }
}

#[derive(Debug)]
pub struct ZmqSocket {
    pub raw: *mut c_void,
    pub ctx: Option<ZmqContext>,
}

impl ZmqSocket {
    pub fn new(ctx: ZmqContext, socket_type: ZmqSocketType) -> Self {
        let socket = ctx.socket(socket_type);
        socket.unwrap()
    }
}

impl Drop for ZmqSocket {
    fn drop(&mut self) {
        let rc = unsafe { zmq::zmq_close(self.raw) };
        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }
    }
}
