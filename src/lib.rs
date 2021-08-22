#![allow(trivial_numeric_casts)]
mod error;
mod socket;
mod message;

use bitflags::bitflags;
use error::{ZmqError, errno_to_error};

use libc::{c_int, c_long, c_short};

use std::ffi;
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::c_void;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd as UnixRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, RawSocket};
use std::result;
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::{mem, ptr, str};

use zmq_sys::{errno, RawFd};

#[macro_export]
macro_rules! zmq_try {
    ($($tt:tt)*) => {{
        let rc = $($tt)*;
        if rc == -1 {
            return Err(errno_to_error());
        }
        rc
    }}
}

pub type ZmqResult<T> = result::Result<T, ZmqError>;

struct RawContext {
    raw: *mut c_void,
}

impl RawContext {
    fn term(&self) -> ZmqResult<()> {
        zmq_try!(unsafe { zmq_sys::zmq_ctx_term(self.raw) });
        Ok(())
    }
}

//这两个trait有什么用？
unsafe impl Send for RawContext {}
unsafe impl Sync for RawContext {}


impl Drop for RawContext {
    fn drop(&mut self) {
        let mut e = self.term();
        while e == Err(ZmqError::EINTR) {
            e = self.term();
        }
    }
}

struct ZmqContext {
    raw: Arc<RawContext>,
}

impl ZmqContext {
    pub fn new() -> Self {
        Self {
            raw: Arc::new(RawContext {
                raw: unsafe { zmq_sys::zmq_ctx_new() },
            }),
        }
    }
}
