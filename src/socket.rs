use crate::Error::{ZmqError, ZmqResult};
use crate::ctx::{ZmqContext};

use crate::zmq;

pub enum ZmqSocketOption {
    ZMQ_PAIR =  0,
    ZMQ_PUB =  1,
    ZMQ_SUB =  2,
    ZMQ_REQ =  3,
    ZMQ_REP =  4,
    ZMQ_DEALER =  5,
    ZMQ_ROUTER =  6,
    ZMQ_PULL =  7,
    ZMQ_PUSH =  8,
    ZMQ_XPUB =  9,
    ZMQ_XSUB =  10,
    ZMQ_STREAM =  11,
    ZMQ_XREQ =  5,
    ZMQ_XREP =  6,
}