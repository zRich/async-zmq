///
// zmq_ctx_destroy - terminate a 0MQ context
// zmq_ctx_get - get context options
// zmq_ctx_new - create new 0MQ context
// zmq_ctx_set - set context options
// zmq_ctx_shutdown - shutdown a 0MQ context
// zmq_ctx_term - terminate a 0MQ context
///
extern crate libc;
use crate::error::{ZmqError, ZmqResult};
use crate::socket::{ZmqSocket, ZmqSocketType};

use crate::zmq;

use std::convert::Into;
use std::ops::Drop;
use std::os::raw::c_void;
use std::sync::Arc;

#[derive(Debug)]
pub struct RawPointer {
    pub rptr: *mut c_void,
}
#[derive(Clone, Debug)]
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

    pub fn term(&mut self) -> ZmqResult<()> {
        unsafe {
            let rc = zmq::zmq_ctx_term(self.raw.rptr);
            if rc != 0 {
                return Err(ZmqError::from(rc));
            }
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

    pub fn socket(&self, socket_type: ZmqSocketType) -> ZmqResult<ZmqSocket> {
        let socket = unsafe { zmq::zmq_socket(self.raw.rptr, socket_type.into()) };

        if socket.is_null() {
            return Err(ZmqError::from(unsafe { zmq::zmq_errno() }));
        }

        Ok(ZmqSocket {
            raw: socket,
            ctx: Some(self.clone()),
        })
    }
}

impl Drop for ZmqContext {
    fn drop(&mut self) {
        let mut rc = self.term();
        while rc == Err(ZmqError::EINTR) {
            rc = self.term();
        }
    }
}

impl Into<*mut c_void> for ZmqContext {
    fn into(self) -> *mut c_void {
        self.raw.rptr
    }
}
