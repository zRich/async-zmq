/// An error returned by a 0MQ API function.
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

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ZmqError {
    EACCES,
    EADDRINUSE,
    EAGAIN,
    EBUSY,
    ECONNREFUSED,
    EFAULT,
    EINTR,
    EHOSTUNREACH,
    EINPROGRESS,
    EINVAL,
    EMFILE,
    EMSGSIZE,
    ENAMETOOLONG,
    ENODEV,
    ENOENT,
    ENOMEM,
    ENOTCONN,
    ENOTSOCK,
    EPROTO,
    EPROTONOSUPPORT,
    ENOTSUP,
    ENOBUFS,
    ENETDOWN,
    EADDRNOTAVAIL,

    // native zmq error codes
    EFSM,
    ENOCOMPATPROTO,
    ETERM,
    EMTHREAD,
}

impl ZmqError {
    pub fn to_raw(self) -> i32 {
        match self {
            ZmqError::EACCES => errno::EACCES,
            ZmqError::EADDRINUSE => errno::EADDRINUSE,
            ZmqError::EAGAIN => errno::EAGAIN,
            ZmqError::EBUSY => errno::EBUSY,
            ZmqError::ECONNREFUSED => errno::ECONNREFUSED,
            ZmqError::EFAULT => errno::EFAULT,
            ZmqError::EINTR => errno::EINTR,
            ZmqError::EHOSTUNREACH => errno::EHOSTUNREACH,
            ZmqError::EINPROGRESS => errno::EINPROGRESS,
            ZmqError::EINVAL => errno::EINVAL,
            ZmqError::EMFILE => errno::EMFILE,
            ZmqError::EMSGSIZE => errno::EMSGSIZE,
            ZmqError::ENAMETOOLONG => errno::ENAMETOOLONG,
            ZmqError::ENODEV => errno::ENODEV,
            ZmqError::ENOENT => errno::ENOENT,
            ZmqError::ENOMEM => errno::ENOMEM,
            ZmqError::ENOTCONN => errno::ENOTCONN,
            ZmqError::ENOTSOCK => errno::ENOTSOCK,
            ZmqError::EPROTO => errno::EPROTO,
            ZmqError::EPROTONOSUPPORT => errno::EPROTONOSUPPORT,
            ZmqError::ENOTSUP => errno::ENOTSUP,
            ZmqError::ENOBUFS => errno::ENOBUFS,
            ZmqError::ENETDOWN => errno::ENETDOWN,
            ZmqError::EADDRNOTAVAIL => errno::EADDRNOTAVAIL,

            ZmqError::EFSM => errno::EFSM,
            ZmqError::ENOCOMPATPROTO => errno::ENOCOMPATPROTO,
            ZmqError::ETERM => errno::ETERM,
            ZmqError::EMTHREAD => errno::EMTHREAD,
        }
    }

    pub fn from_raw(raw: i32) -> ZmqError {
        match raw {
            errno::EACCES => ZmqError::EACCES,
            errno::EADDRINUSE => ZmqError::EADDRINUSE,
            errno::EAGAIN => ZmqError::EAGAIN,
            errno::EBUSY => ZmqError::EBUSY,
            errno::ECONNREFUSED => ZmqError::ECONNREFUSED,
            errno::EFAULT => ZmqError::EFAULT,
            errno::EHOSTUNREACH => ZmqError::EHOSTUNREACH,
            errno::EINPROGRESS => ZmqError::EINPROGRESS,
            errno::EINVAL => ZmqError::EINVAL,
            errno::EMFILE => ZmqError::EMFILE,
            errno::EMSGSIZE => ZmqError::EMSGSIZE,
            errno::ENAMETOOLONG => ZmqError::ENAMETOOLONG,
            errno::ENODEV => ZmqError::ENODEV,
            errno::ENOENT => ZmqError::ENOENT,
            errno::ENOMEM => ZmqError::ENOMEM,
            errno::ENOTCONN => ZmqError::ENOTCONN,
            errno::ENOTSOCK => ZmqError::ENOTSOCK,
            errno::EPROTO => ZmqError::EPROTO,
            errno::EPROTONOSUPPORT => ZmqError::EPROTONOSUPPORT,
            errno::ENOTSUP => ZmqError::ENOTSUP,
            errno::ENOBUFS => ZmqError::ENOBUFS,
            errno::ENETDOWN => ZmqError::ENETDOWN,
            errno::EADDRNOTAVAIL => ZmqError::EADDRNOTAVAIL,
            errno::EINTR => ZmqError::EINTR,

            // These may turn up on platforms that don't support these
            // errno codes natively (Windows)
            errno::ENOTSUP_ALT => ZmqError::ENOTSUP,
            errno::EPROTONOSUPPORT_ALT => ZmqError::EPROTONOSUPPORT,
            errno::ENOBUFS_ALT => ZmqError::ENOBUFS,
            errno::ENETDOWN_ALT => ZmqError::ENETDOWN,
            errno::EADDRINUSE_ALT => ZmqError::EADDRINUSE,
            errno::EADDRNOTAVAIL_ALT => ZmqError::EADDRNOTAVAIL,
            errno::ECONNREFUSED_ALT => ZmqError::ECONNREFUSED,
            errno::EINPROGRESS_ALT => ZmqError::EINPROGRESS,
            errno::ENOTSOCK_ALT => ZmqError::ENOTSOCK,
            errno::EMSGSIZE_ALT => ZmqError::EMSGSIZE,

            // TODO: these are present in `zmq-sys`, but not handled, as that
            // would break backwards-compatibility for the `Error` enum.

            // errno::EAFNOSUPPORT_ALT => Error::EAFNOSUPPORT,
            // errno::ENETUNREACH_ALT => Error::ENETUNREACH,
            // errno::ECONNABORTED_ALT => Error::ECONNABORTED,
            // errno::ECONNRESET_ALT => Error::ECONNRESET,
            // errno::ENOTCONN_ALT => Error::ENOTCONN,
            // errno::ETIMEDOUT_ALT => Error::ETIMEDOUT,
            // errno::EHOSTUNREACH_ALT => Error::EHOSTUNREACH,
            // errno::ENETRESET_ALT => Error::ENETRESET,

            // 0MQ native error codes
            errno::EFSM => ZmqError::EFSM,
            errno::ENOCOMPATPROTO => ZmqError::ENOCOMPATPROTO,
            errno::ETERM => ZmqError::ETERM,
            errno::EMTHREAD => ZmqError::EMTHREAD,

            x => unsafe {
                let s = zmq_sys::zmq_strerror(x);
                panic!(
                    "unknown error [{}]: {}",
                    x,
                    str::from_utf8(ffi::CStr::from_ptr(s).to_bytes()).unwrap()
                )
            },
        }
    }

    /// Returns the error message provided by 0MQ.
    pub fn message(self) -> &'static str {
        unsafe {
            let s = zmq_sys::zmq_strerror(self.to_raw());
            let v: &'static [u8] = mem::transmute(ffi::CStr::from_ptr(s).to_bytes());
            str::from_utf8(v).unwrap()
        }
    }
}

impl std::error::Error for ZmqError {
    fn description(&self) -> &str {
        self.message()
    }
}

impl std::fmt::Display for ZmqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl fmt::Debug for ZmqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: An unquoted string is not a good `Debug` output.
        write!(f, "{}", self.message())
    }
}

impl From<ZmqError> for std::io::Error {
    fn from(error: ZmqError) -> Self {
        use std::io::ErrorKind;

        let kind = match error {
            ZmqError::ENOENT => ErrorKind::NotFound,
            ZmqError::EACCES => ErrorKind::PermissionDenied,
            ZmqError::ECONNREFUSED => ErrorKind::ConnectionRefused,
            ZmqError::ENOTCONN => ErrorKind::NotConnected,
            ZmqError::EADDRINUSE => ErrorKind::AddrInUse,
            ZmqError::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
            ZmqError::EAGAIN => ErrorKind::WouldBlock,
            ZmqError::EINVAL => ErrorKind::InvalidInput,
            ZmqError::EINTR => ErrorKind::Interrupted,
            _ => ErrorKind::Other,
        };
        // TODO: With rust 1.14 and up there is an optimization
        // opportunity using `std::io::Error: From<ErrorKind>` when
        // `kind != Other`. We should do that once 1.14 has been
        // stable for a bit.
        std::io::Error::new(kind, error)
    }
}

pub fn errno_to_error() -> ZmqError {
    ZmqError::from_raw(unsafe { zmq_sys::zmq_errno() })
}