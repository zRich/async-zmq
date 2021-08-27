use crate::zmq;
// pub type ZmqError = u32;

use std::{self, ffi::{self, CStr}, mem, str, io::{Error, ErrorKind}};
// use std::result::Result;

#[derive(Clone, Copy)]
pub enum ZmqError {
    EFAULT =  zmq::EFAULT as isize,  //invalid ctx
    EINTR =  zmq::EINTR as isize,  //term ctx failed, try again.
}

impl ZmqError {
    pub fn err_no(self) -> i32 {
        let _no = 
        match self {
            EFAULT => zmq::EFAULT,
            EFAULT => zmq::EFAULT,
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

    pub fn from_raw(error: i32) ->Self {
        match error as u32 {
            zmq::EINTR => ZmqError::EINTR,
            zmq::EINVAL => ZmqError::EFAULT,
        }
    }

    pub fn into_raw(error: ZmqError) -> Error {
        let kind = match error {
            ZmqError::EINTR => ErrorKind::Interrupted,
            ZmqError::EFAULT => ErrorKind::InvalidInput,  //invalid ctx
        };

        Error::new(kind, error)
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

impl From<ZmqError> for Error {
    fn from(error: ZmqError) -> Self {
        let kind = match error {
            ZmqError::EINTR => ErrorKind::Interrupted,
            ZmqError::EFAULT => ErrorKind::InvalidInput,  //invalid ctx
        };

        Error::new(kind, error)
    }
}

// impl From<Error> for ZmqError {
//     fn from(error: Error) -> Self {
//         match error {
//             EFAULT => Error::new(ErrorKind::Interrupted, EFAULT),
//             EFAULT => Error::new(ErrorKind::InvalidInput, EFAULT),  //invalid ctx
//         }
//     }
// }