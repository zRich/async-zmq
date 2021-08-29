use crate::ctx::ZmqContext;
use crate::error::{ZmqError, ZmqResult};

use crate::zmq;

use std::convert::{From, Into};
use std::ffi::CString;
use std::os::raw::{c_int, c_void};

#[allow(non_camel_case_types)]
pub enum ZmqSocketType {
    ZMQ_PAIR = 0,
    ZMQ_PUB = 1,
    ZMQ_SUB = 2,
    ZMQ_REQ = 3,
    ZMQ_REP = 4,
    ZMQ_DEALER = 5,
    ZMQ_ROUTER = 6,
    ZMQ_PULL = 7,
    ZMQ_PUSH = 8,
    ZMQ_XPUB = 9,
    ZMQ_XSUB = 10,
    ZMQ_STREAM = 11,
    // ZMQ_XREQ =  5,
    // ZMQ_XREP =  6,
}

impl Into<i32> for ZmqSocketType {
    fn into(self) -> i32 {
        match self {
            ZmqSocketType::ZMQ_PAIR => 0,
            ZmqSocketType::ZMQ_PUB => 1,
            ZmqSocketType::ZMQ_SUB => 2,
            ZmqSocketType::ZMQ_REQ => 3,
            ZmqSocketType::ZMQ_REP => 4,
            ZmqSocketType::ZMQ_DEALER => 5,
            ZmqSocketType::ZMQ_ROUTER => 6,
            ZmqSocketType::ZMQ_PULL => 7,
            ZmqSocketType::ZMQ_PUSH => 8,
            ZmqSocketType::ZMQ_XPUB => 9,
            ZmqSocketType::ZMQ_XSUB => 10,
            ZmqSocketType::ZMQ_STREAM => 11,
        }
    }
}

#[derive(Debug)]
pub struct ZmqSocket {
    pub raw: *mut c_void,
    pub ctx: Option<ZmqContext>,
}

impl ZmqSocket {
    pub fn new(ctx: ZmqContext, socket_type: ZmqSocketType) -> Self {
        let socket = ctx.socket(socket_type);
        socket.unwrap()
    }

    pub fn into(&self) -> *mut c_void {
        self.raw
    }

    pub fn bind(&self, endpoint: &str) -> ZmqResult<()> {
        let c_endpoint = CString::new(endpoint.as_bytes()).unwrap();
        let rc = unsafe { zmq::zmq_bind(self.into(), c_endpoint.as_ptr()) };
        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }

        Ok(())
    }

    pub fn unbind(&self, endpoint: &str) -> ZmqResult<()> {
        let c_endpoint = CString::new(endpoint.as_bytes()).unwrap();
        let rc = unsafe { zmq::zmq_unbind(self.into(), c_endpoint.as_ptr()) };
        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }

        Ok(())
    }

    pub fn connect(&self, endpoint: &str) -> ZmqResult<()> {
        let c_endpoint = CString::new(endpoint.as_bytes()).unwrap();
        let rc = unsafe { zmq::zmq_connect(self.into(), c_endpoint.as_ptr()) };
        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }

        Ok(())
    }

    pub fn disconnect(&self, endpoint: &str) -> ZmqResult<()> {
        let c_endpoint = CString::new(endpoint.as_bytes()).unwrap();
        let rc = unsafe { zmq::zmq_disconnect(self.into(), c_endpoint.as_ptr()) };
        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }

        Ok(())
    }

    pub fn monitor(&self, endpoint: &str, event: i32) -> ZmqResult<()> {
        let c_endpoint = CString::new(endpoint.as_bytes()).unwrap();
        let rc =
            unsafe { zmq::zmq_socket_monitor(self.into(), c_endpoint.as_ptr(), event as c_int) };
        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }

        Ok(())
    }

    //zmq_send
    //zmq_msg_send
    //send_multipart
    //recv
    //zmq_recv
}

impl Drop for ZmqSocket {
    fn drop(&mut self) {
        let rc = unsafe { zmq::zmq_close(self.raw) };
        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }
    }
}

#[allow(non_camel_case_types)]
pub enum ZmqSocketEvent {
    ZMQ_EVENT_CONNECTED = zmq::ZMQ_EVENT_CONNECTED as isize,
    ZMQ_EVENT_CONNECT_DELAYED = zmq::ZMQ_EVENT_CONNECT_DELAYED as isize,
    ZMQ_EVENT_CONNECT_RETRIED = zmq::ZMQ_EVENT_CONNECT_RETRIED as isize,
    ZMQ_EVENT_LISTENING = zmq::ZMQ_EVENT_LISTENING as isize,
    ZMQ_EVENT_BIND_FAILED = zmq::ZMQ_EVENT_BIND_FAILED as isize,
    ZMQ_EVENT_ACCEPTED = zmq::ZMQ_EVENT_ACCEPTED as isize,
    ZMQ_EVENT_ACCEPT_FAILED = zmq::ZMQ_EVENT_ACCEPT_FAILED as isize,
    ZMQ_EVENT_CLOSED = zmq::ZMQ_EVENT_CLOSED as isize,
    ZMQ_EVENT_CLOSE_FAILED = zmq::ZMQ_EVENT_CLOSE_FAILED as isize,
    ZMQ_EVENT_DISCONNECTED = zmq::ZMQ_EVENT_DISCONNECTED as isize,
    ZMQ_EVENT_MONITOR_STOPPED = zmq::ZMQ_EVENT_MONITOR_STOPPED as isize,
    ZMQ_EVENT_ALL = zmq::ZMQ_EVENT_ALL as isize,
    ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL = zmq::ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL as isize,
    ZMQ_EVENT_HANDSHAKE_SUCCEEDED = zmq::ZMQ_EVENT_HANDSHAKE_SUCCEEDED as isize,
    ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL = zmq::ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL as isize,
    ZMQ_EVENT_HANDSHAKE_FAILED_AUTH = zmq::ZMQ_EVENT_HANDSHAKE_FAILED_AUTH as isize,
}

impl Into<i32> for ZmqSocketEvent {
    fn into(self) -> i32 {
        // match self {
        //     ZmqSocketEvent::ZMQ_EVENT_CONNECTED => zmq::ZMQ_EVENT_CONNECTED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_CONNECT_DELAYED => zmq::ZMQ_EVENT_CONNECT_DELAYED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_CONNECT_RETRIED => zmq::ZMQ_EVENT_CONNECT_RETRIED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_LISTENING => zmq::ZMQ_EVENT_LISTENING as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_BIND_FAILED => zmq::ZMQ_EVENT_BIND_FAILED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_ACCEPTED => zmq::ZMQ_EVENT_ACCEPTED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_ACCEPT_FAILED => zmq::ZMQ_EVENT_ACCEPT_FAILED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_CLOSED => zmq::ZMQ_EVENT_CLOSED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_CLOSE_FAILED => zmq::ZMQ_EVENT_CLOSE_FAILED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_DISCONNECTED => zmq::ZMQ_EVENT_DISCONNECTED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_MONITOR_STOPPED => zmq::ZMQ_EVENT_MONITOR_STOPPED as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_ALL => zmq::ZMQ_EVENT_ALL as i32,
        //     ZmqSocketEvent::ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL => {
        //         zmq::ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL as i32
        //     }
        //     ZmqSocketEvent::ZMQ_EVENT_HANDSHAKE_SUCCEEDED => {
        //         zmq::ZMQ_EVENT_HANDSHAKE_SUCCEEDED as i32
        //     }
        //     ZmqSocketEvent::ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL => {
        //         zmq::ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL as i32
        //     }
        //     ZmqSocketEvent::ZMQ_EVENT_HANDSHAKE_FAILED_AUTH => {
        //         zmq::ZMQ_EVENT_HANDSHAKE_FAILED_AUTH as i32
        //     }
        // }

        self as i32
    }
}

impl From<i32> for ZmqSocketEvent {
    fn from(event: i32) -> Self {
        let event = event as u32;
        match event {
            zmq::ZMQ_EVENT_CONNECTED => ZmqSocketEvent::ZMQ_EVENT_CONNECTED,
            zmq::ZMQ_EVENT_CONNECT_DELAYED => ZmqSocketEvent::ZMQ_EVENT_CONNECT_DELAYED,
            zmq::ZMQ_EVENT_CONNECT_RETRIED => ZmqSocketEvent::ZMQ_EVENT_CONNECT_RETRIED,
            zmq::ZMQ_EVENT_LISTENING => ZmqSocketEvent::ZMQ_EVENT_LISTENING,
            zmq::ZMQ_EVENT_BIND_FAILED => ZmqSocketEvent::ZMQ_EVENT_BIND_FAILED,
            zmq::ZMQ_EVENT_ACCEPTED => ZmqSocketEvent::ZMQ_EVENT_ACCEPTED,
            zmq::ZMQ_EVENT_ACCEPT_FAILED => ZmqSocketEvent::ZMQ_EVENT_ACCEPT_FAILED,
            zmq::ZMQ_EVENT_CLOSED => ZmqSocketEvent::ZMQ_EVENT_CLOSED,
            zmq::ZMQ_EVENT_CLOSE_FAILED => ZmqSocketEvent::ZMQ_EVENT_CLOSE_FAILED,
            zmq::ZMQ_EVENT_DISCONNECTED => ZmqSocketEvent::ZMQ_EVENT_DISCONNECTED,
            zmq::ZMQ_EVENT_MONITOR_STOPPED => ZmqSocketEvent::ZMQ_EVENT_MONITOR_STOPPED,
            zmq::ZMQ_EVENT_ALL => ZmqSocketEvent::ZMQ_EVENT_ALL,
            zmq::ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL => {
                ZmqSocketEvent::ZMQ_EVENT_HANDSHAKE_FAILED_NO_DETAIL
            }
            zmq::ZMQ_EVENT_HANDSHAKE_SUCCEEDED => ZmqSocketEvent::ZMQ_EVENT_HANDSHAKE_SUCCEEDED,
            zmq::ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL => {
                ZmqSocketEvent::ZMQ_EVENT_HANDSHAKE_FAILED_PROTOCOL
            }
            zmq::ZMQ_EVENT_HANDSHAKE_FAILED_AUTH => ZmqSocketEvent::ZMQ_EVENT_HANDSHAKE_FAILED_AUTH,
            e => panic!("unknown event {}", e),
        }
    }
}
