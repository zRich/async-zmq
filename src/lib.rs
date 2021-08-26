#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/zmq.rs"));

// #![allow(trivial_numeric_casts)]
pub mod error;
// mod message;
// mod socket;
pub mod zmq;
pub mod ctx;

use bitflags::bitflags;
// use error::{errno_to_error, ZmqError};

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


pub type ZmqError = u32;
pub type ZmqResult<T> = result::Result<T, ZmqError>;

// use zmq_sys::{errno, RawFd};

// #[macro_export]
// macro_rules! zmq_try {
//     ($($tt:tt)*) => {{
//         let rc = $($tt)*;
//         if rc == -1 {
//             return Err(errno_to_error());
//         }
//         rc
//     }}
// }