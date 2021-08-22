use zmq_sys::{errno};
use std;

use bitflags::bitflags;
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

use crate::{ZmqContext, zmq_try, ZmqResult};
use crate::error::{ZmqError, errno_to_error};

pub struct ZmqSocket {
    sock: *mut c_void,
    ctx: Option<ZmqContext>,
    owned: bool,  //这个做什么用的？
}

unsafe impl Send for ZmqSocket {}

impl Drop for ZmqSocket {
    fn drop(&mut self) {
        if self.owned && unsafe { zmq_sys::zmq_close(self.sock) } == -1 {
            panic!(errno_to_error());
        }
    }
}

#[cfg(unix)]
impl AsRawFd for ZmqSocket {
    fn as_raw_fd(&self) -> UnixRawFd {
        self.get_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for ZmqSocket {
    fn as_raw_socket(&self) -> RawSocket {
        self.get_fd().unwrap() as RawSocket
    }
}

impl ZmqSocket {
    pub unsafe fn from_raw(sock: *mut c_void) -> Self {
        Self {
            sock,
            ctx: None,
            owned: true,
        }
    }

    pub unsafe fn into_raw(mut self) -> *mut c_void {
        self.owned = false;
        self.sock
    }


    fn get_fd(&self) -> UnixRawFd {
        zmq_sys::ZMQ_FD as UnixRawFd
    }

    pub fn as_mut_ptr(&mut self) -> *mut c_void {
        self.sock
    }

    pub fn bind(&self, endpoint: &str) -> ZmqResult<()> {
        let c_str = ffi::CString::new(endpoint.as_bytes()).unwrap();
        zmq_try!(unsafe {zmq_sys::zmq_bind(self.sock, c_str.as_ptr())});
        Ok(())
    }

    pub fn connect(&self, endpoint: &str) -> ZmqResult<()> {
        let c_str = ffi::CString::new(endpoint.as_bytes()).unwrap();
        zmq_try!(unsafe { zmq_sys::zmq_connect(self.sock, c_str.as_ptr()) });
        Ok(())
    }

    /// Disconnect a previously connected socket
    pub fn disconnect(&self, endpoint: &str) -> ZmqResult<()> {
        let c_str = ffi::CString::new(endpoint.as_bytes()).unwrap();
        zmq_try!(unsafe { zmq_sys::zmq_disconnect(self.sock, c_str.as_ptr()) });
        Ok(())
    }
}