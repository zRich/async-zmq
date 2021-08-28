use crate::zmq;

use std::convert::{From};
use std::result;
use std::{
    self,
    ffi::{CStr},
    str,
};

pub type ZmqResult<T> = result::Result<T, ZmqError>;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ZmqError {
    EFAULT = zmq::EFAULT as isize, //invalid ctx
    EINTR = zmq::EINTR as isize,   //term ctx failed, try again.
}

impl ZmqError {
    pub fn err_no(self) -> i32 {
        let _no = match self {
            ZmqError::EFAULT => zmq::EFAULT,
            ZmqError::EINTR => zmq::EINTR,
        };

        _no as i32
    }

    pub fn err_msg(self) -> &'static str {
        unsafe {
            let msg = zmq::zmq_strerror(self.err_no());
            let msg = CStr::from_ptr(msg);
            msg.to_str().unwrap()
            // std::ffi::CStr::from_ptr(msg).to_string_lossy().into_owned().as_str()  //to be tested
            // let v: &'static [u8] = mem::transmute(ffi::CStr::from_ptr(msg).to_bytes());
            // str::from_utf8(v).unwrap()
        }
    }
}

impl std::error::Error for ZmqError {
    fn description(&self) -> &str {
        self.err_msg()
    }
}

impl std::fmt::Debug for ZmqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err_msg())
    }
}
impl std::fmt::Display for ZmqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err_msg())
    }
}

impl From<i32> for ZmqError {
    fn from(error: i32) -> Self {
        match error as u32 {
            zmq::EINTR => ZmqError::EINTR,
            zmq::EINVAL => ZmqError::EFAULT,

            _other => panic!("unknown error {}", _other),
        }
    }
}