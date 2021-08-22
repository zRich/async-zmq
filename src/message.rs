use libc::size_t;

use std::ffi;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::{ptr, slice, str};

use crate::error::{errno_to_error, ZmqError};

pub struct Message {
    msg: zmq_sys::zmq_msg_t,
}

impl Message {
    unsafe fn alloc<F>(f: F) -> Message
    where
        F: FnOnce(&mut zmq_sys::zmq_msg_t) -> i32,
    {
        let mut msg = zmq_sys::zmq_msg_t::default();
        let rc = f(&mut msg);
        if rc == -1 {
            panic!(errno_to_error())
        }
        Message { msg }
    }

    //zmq_msg_init
    pub fn new() -> Self {
        let mut msg = zmq_sys::zmq_msg_t::default();
        unsafe {
            Self::alloc(|msg| zmq_sys::zmq_msg_init(msg));
            Self { msg }
        }
    }

    //zmq_msg_init_size
    pub unsafe fn with_capacity(size: usize) -> Self {
        let mut msg = zmq_sys::zmq_msg_t::default();
        Self::alloc(|msg| zmq_sys::zmq_msg_init_size(msg, size));
        Self { msg }
    }

    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self).ok()
    }

    pub fn get_more(&self) -> bool {
        let rc = unsafe {
            zmq_sys::zmq_msg_more(&self.msg)
        };

        rc != 0
    }

    pub fn gets<'a>(&'a mut self, property: &str) -> Option<&'a str> {
        let c_str = ffi::CString::new(property.as_bytes()).unwrap();

        let value = unsafe {
            zmq_sys::zmq_msg_gets(&self.msg, c_str.as_ptr())
        };

        if value.is_null() {
            None
        } else {
            str::from_utf8(unsafe {
                ffi::CStr::from_ptr(value)
            }.to_bytes()).ok()
        }
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            let rc = zmq_sys::zmq_msg_close(&mut self.msg);
            assert_eq!(rc, 0);
        }
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Deref for Message {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe {
            let ptr = &self.msg as *const _ as *mut _;
            let data = zmq_sys::zmq_msg_data(ptr);
            let len = zmq_sys::zmq_msg_size(ptr) as usize;
            slice::from_raw_parts(data as *mut u8, len)
        }
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        self[..] == other[..]
    }
}

impl Eq for Message {}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut [u8] {
        // This is safe because we're constraining the slice to the lifetime of
        // this message.
        unsafe {
            let data = zmq_sys::zmq_msg_data(&mut self.msg);
            let len = zmq_sys::zmq_msg_size(&self.msg) as usize;
            slice::from_raw_parts_mut(data as *mut u8, len)
        }
    }
}

unsafe extern "C" fn drop_msg_data_box(data: *mut c_void, hint: *mut c_void) {
    let _ = Box::from_raw(slice::from_raw_parts_mut(data as *mut u8, hint as usize));
}

//zmq_msg_init_data
impl From<Box<[u8]>> for Message {
    fn from(data: Box<[u8]>) -> Self {
        let len = data.len();
        if len == 0 {
            return Message::new();
        }

        let raw = Box::into_raw(data);

        let mut msg = zmq_sys::zmq_msg_t::default();

        unsafe {
            Self::alloc(|msg| {
                zmq_sys::zmq_msg_init_data(
                    msg,
                    raw as *mut c_void,
                    len,
                    Some(drop_msg_data_box),
                    len as *mut c_void,
                )
            })
        };
        Self { msg }
    }
}

impl<'a> From<&'a [u8]> for Message {
    /// Construct a message from a byte slice by copying the data.
    fn from(data: &'a [u8]) -> Self {
        unsafe {
            let mut msg = Message::with_capacity(data.len());
            ptr::copy_nonoverlapping(data.as_ptr(), msg.as_mut_ptr(), data.len());
            msg
        }
    }
}

impl From<Vec<u8>> for Message {
    fn from(data: Vec<u8>) -> Self {
        Self::from(data.into_boxed_slice())
    }
}

impl<'a> From<&'a str> for Message {
    fn from(data: &'a str) -> Self {
        Self::from(data.as_bytes())
    }
}

impl<'a, T> From<&'a T> for Message
where
    T: Into<Message> + Clone,
{
    fn from(data: &'a T) -> Self {
        data.clone().into()
    }
}

pub fn msg_ptr(msg: &mut Message) -> *mut zmq_sys::zmq_msg_t {
    &mut msg.msg
}
