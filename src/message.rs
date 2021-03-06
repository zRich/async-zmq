// zmq_msg_close - release 0MQ message
// zmq_msg_copy - copy content of a message to another message
// zmq_msg_data - retrieve pointer to message content
// zmq_msg_gets - get message metadata property
// zmq_msg_get - get message property
// zmq_msg_init_data - initialise 0MQ message from a supplied buffer
// zmq_msg_init_size - initialise 0MQ message of a specified size
// zmq_msg_init - initialise empty 0MQ message
// zmq_msg_more - indicate if there are more message parts to receive
// zmq_msg_move - move content of a message to another message
// zmq_msg_recv - receive a message part from a socket
// zmq_msg_routing_id - return routing ID for message, if any
// zmq_msg_send - send a message part on a socket
// zmq_msg_set_routing_id - set routing ID property on message
// zmq_msg_set - set message property
// zmq_msg_size - retrieve message content size in bytes

use crate::error::{ZmqError, ZmqResult};
use crate::zmq::{self, size_t};

use std::ffi::{CStr, CString};
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str, {ptr, slice},
};

pub struct ZmqMessage {
    pub raw: zmq::zmq_msg_t,
}

impl ZmqMessage {
    pub fn new() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            zmq::zmq_msg_init(&mut msg);
            Self { raw: msg }
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            zmq::zmq_msg_init_size(&mut msg, cap as size_t);
            Self { raw: msg }
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self).ok()
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        // let m = vec![];
        // self.as_str().iter().map(|b | *b as u8).collect::<Vec<_>>();
        let m = self.as_str().unwrap().as_bytes().to_vec();
        Some(m)
    }

    pub fn get<'a>(&'a mut self, property: i32) -> ZmqResult<i32> {
        unsafe {
            let rc = zmq::zmq_msg_get(&self.raw, property);
            if rc == -1 {
                panic!("{}", ZmqError::from(zmq::zmq_errno()))
            }
            Ok(rc)
        }
    }

    pub fn gets<'a>(&'a mut self, property: &str) -> Option<&'a str> {
        let _prop = CString::new(property.as_bytes()).unwrap();
        let _value = unsafe { zmq::zmq_msg_gets(&self.raw, _prop.as_ptr()) };
        if _value.is_null() {
            None
        } else {
            str::from_utf8(unsafe { CStr::from_ptr(_value) }.to_bytes()).ok()
        }
    }

    pub fn get_more(&self) -> bool {
        unsafe {
            let rc = zmq::zmq_msg_more(&self.raw);
            rc != 0
        }
    }
}

impl<'a> From<&'a [u8]> for ZmqMessage {
    fn from(data: &'a [u8]) -> Self {
        unsafe {
            let mut msg = ZmqMessage::with_capacity(data.len());
            ptr::copy_nonoverlapping(data.as_ptr(), msg.as_mut_ptr(), data.len());
            msg
        }
    }
}

impl<'a> From<&'a String> for ZmqMessage {
    fn from(data: &'a String) -> Self {
        Self::from(data.as_bytes())
    }
}

impl<'a> From<&'a str> for ZmqMessage {
    fn from(data: &'a str) -> Self {
        Self::from(data.as_bytes())
    }
}

impl From<Vec<u8>> for ZmqMessage {
    fn from(data: Vec<u8>) -> Self {
        Self::from(data.as_slice())
    }
}

impl Into<Vec<u8>> for ZmqMessage {
    fn into(self) -> Vec<u8> {
        // self.as_bytes().unwrap()
        self.as_str().unwrap().as_bytes().to_vec()
    }
}

impl Drop for ZmqMessage {
    fn drop(&mut self) {
        let rc = unsafe { zmq::zmq_msg_close(&mut self.raw) };

        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }
    }
}

impl Debug for ZmqMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.raw)
    }
}

impl Display for ZmqMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.raw)
    }
}

impl Deref for ZmqMessage {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let rptr = &self.raw as *const _ as *mut _;
            let data = zmq::zmq_msg_data(rptr);
            let len = zmq::zmq_msg_size(rptr) as usize;
            slice::from_raw_parts(data as *mut u8, len)
        }
    }
}

impl DerefMut for ZmqMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let data = zmq::zmq_msg_data(&mut self.raw);
            let len = zmq::zmq_msg_size(&self.raw) as usize;
            slice::from_raw_parts_mut(data as *mut u8, len)
        }
    }
}
