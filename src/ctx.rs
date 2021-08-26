///
// zmq_ctx_destroy - terminate a 0MQ context
// zmq_ctx_get - get context options
// zmq_ctx_new - create new 0MQ context
// zmq_ctx_set - set context options
// zmq_ctx_shutdown - shutdown a 0MQ context
// zmq_ctx_term - terminate a 0MQ context
///
extern crate libc;
use crate::error::ZmqError;
use crate::{zmq, ZmqResult};

use std::ffi;
use std::fmt::{Error, Result};
use std::ops::Drop;
use std::os::raw::c_void as void;
use std::sync::Arc;

pub struct RawPointer {
    pub rptr: *mut void,
}

pub struct ZmqContext {
    raw: Arc<RawPointer>,
}

unsafe impl Send for ZmqContext {}
unsafe impl Sync for ZmqContext {}

impl ZmqContext {
    pub fn new() -> Self {
        Self {
            raw: Arc::new(RawPointer {
                rptr: unsafe { zmq::zmq_ctx_new() },
            }),
            // raw: Arc::new(unsafe { zmq::zmq_ctx_new() }),
        }
    }

    pub fn term(&mut self) -> std::result::Result<(), std::io::Error> {
        unsafe {
            // let Some(r) = self.raw.
            zmq::zmq_ctx_term(self.raw.rptr)
        };
        Ok(())
    }

    pub fn get_io_threads(&self) -> ZmqResult<i32> {
        let rc = unsafe { zmq::zmq_ctx_get(self.raw.rptr, zmq::ZMQ_IO_THREADS as _) };
        Ok(rc as i32)
    }

    pub fn set_io_threads(&self, value: i32) -> ZmqResult<()> {
        unsafe { zmq::zmq_ctx_set(self.raw.rptr, zmq::ZMQ_IO_THREADS as _, value as i32) };
        Ok(())
    }
}

impl Drop for ZmqContext {
    fn drop(&mut self) {
        unsafe {
            // let Some(r) = self.raw.
            zmq::zmq_ctx_term(self.raw.rptr)
        };
    }
}
// pub enum ZmqContextOptions {
//     ZMQ_IO_THREADS = zmq::ZMQ_IO_THREADS,
//     ZMQ_MAX_SOCKETS = zmq::ZMQ_MAX_SOCKETS,
//     ZMQ_MAX_MSGSZ = zmq::ZMQ_MAX_MSGSZ,
//     ZMQ_ZERO_COPY_RCV = zmq::ZMQ_ZERO_COPY_RCV,
//     ZMQ_SOCKET_LIMIT = zmq::ZMQ_SOCKET_LIMIT,
//     ZMQ_IPV6 = zmq::ZMQ_IPV6,
//     ZMQ_BLOCKY = zmq::ZMQ_BLOCKY,
//     ZMQ_THREAD_SCHED_POLICY = zmq::ZMQ_THREAD_SCHED_POLICY,
//     ZMQ_THREAD_PRIORITY = zmq::ZMQ_THREAD_PRIORITY,
//     ZMQ_THREAD_NAME_PREFIX = zmq::ZMQ_THREAD_NAME_PREFIX,
//     ZMQ_MSG_T_SIZE = zmq::ZMQ_MSG_T_SIZE,
// }
