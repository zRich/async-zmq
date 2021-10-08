use crate::ctx::ZmqContext;
use crate::error::{ZmqError, ZmqResult};

use crate::message::ZmqMessage;
use crate::zmq::{self, size_t, *};

use std::convert::{From, Into};
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::{c_int, c_long, c_short, c_uint, c_void};
use std::ptr::null_mut;

use bitflags::bitflags;

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
    pub fn new(ctx: &ZmqContext, socket_type: ZmqSocketType) -> Self {
        ctx.socket(socket_type).unwrap()
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

    pub fn send(&self, msg: ZmqMessage, flags: i32) -> ZmqResult<()> {
        let rc = unsafe {
            let mut data = msg.raw;
            zmq::zmq_msg_send(&mut data, self.raw, flags)
        };

        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }

        Ok(())
    }

    pub fn recv(&self, msg: &mut ZmqMessage, flags: i32) -> ZmqResult<()> {
        let rc = unsafe { zmq::zmq_msg_recv(&mut msg.raw, self.raw, flags) };

        if rc == -1 {
            panic!("{}", ZmqError::from(unsafe { zmq::zmq_errno() }))
        }

        Ok(())
    }

    pub fn setOption<'a>(&mut self, option: c_uint, value: &'a [u8]) -> i32 {
        unsafe {
            zmq::zmq_setsockopt(
                self.raw,
                option as i32,
                value.as_ptr() as *const c_void,
                value.len() as size_t,
            )
        }
    }

    pub fn to_zmq_poll_item(&self, events: ZmqPollEvent) -> ZmqPollItem {
        ZmqPollItem {
            socket: self.raw,
            fd: 0,
            events: events.bits(),
            revents: 0,
            _marker: PhantomData,
        }
    }
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

// 直接使用option的常量定义吧，把option定义成enum画蛇添足
// ///definions on http://api.zeromq.org/master:zmq-setsockopt
// ///not all options defined
// #[allow(non_camel_case_types)]
// pub enum ZmqSocketOption {
//     ZMQ_AFFINITY = zmq::ZMQ_AFFINITY as isize,
//     ZMQ_SUBSCRIBE = zmq::ZMQ_SUBSCRIBE as isize,
//     ZMQ_BACKLOG = zmq::ZMQ_BACKLOG as isize,
//     ZMQ_BINDTODEVICE = zmq::ZMQ_BINDTODEVICE as isize,
//     ZMQ_CONNECT_RID = zmq::ZMQ_CONNECT_RID as isize,
//     // ZMQ_CONNECT_ROUTING_ID = zmq::ZMQ_CONNECT_ROUTING_ID as isize,
//     ZMQ_CONFLATE = zmq::ZMQ_CONFLATE as isize,
//     ZMQ_CONNECT_TIMEOUT = zmq::ZMQ_CONNECT_TIMEOUT as isize,
//     ZMQ_CURVE_PUBLICKEY = zmq::ZMQ_CURVE_PUBLICKEY as isize,
//     ZMQ_CURVE_SECRETKEY = zmq::ZMQ_CURVE_SECRETKEY as isize,
//     ZMQ_CURVE_SERVER = zmq::ZMQ_CURVE_SERVER as isize,
//     ZMQ_CURVE_SERVERKEY = zmq::ZMQ_CURVE_SERVERKEY as isize,
//     ZMQ_GSSAPI_PLAINTEXT = zmq::ZMQ_GSSAPI_PLAINTEXT as isize,
//     ZMQ_GSSAPI_PRINCIPAL = zmq::ZMQ_GSSAPI_PRINCIPAL as isize,
//     ZMQ_GSSAPI_SERVER = zmq::ZMQ_GSSAPI_SERVER as isize,
//     ZMQ_GSSAPI_SERVICE_PRINCIPAL = zmq::ZMQ_GSSAPI_SERVICE_PRINCIPAL as isize,

//     ZMQ_GSSAPI_SERVICE_PRINCIPAL_NAMETYPE = zmq::ZMQ_GSSAPI_SERVICE_PRINCIPAL_NAMETYPE as isize,
//     ZMQ_GSSAPI_PRINCIPAL_NAMETYPE = zmq::ZMQ_GSSAPI_PRINCIPAL_NAMETYPE as isize,
//     ZMQ_HANDSHAKE_IVL = zmq::ZMQ_HANDSHAKE_IVL as isize,
//     ZMQ_HEARTBEAT_IVL = zmq::ZMQ_HEARTBEAT_IVL as isize,
//     ZMQ_HEARTBEAT_TIMEOUT = zmq::ZMQ_HEARTBEAT_TIMEOUT as isize,
//     ZMQ_HEARTBEAT_TTL = zmq::ZMQ_HEARTBEAT_TTL as isize,
//     ZMQ_IDENTITY = zmq::ZMQ_IDENTITY as isize,
//     ZMQ_IMMEDIATE = zmq::ZMQ_IMMEDIATE as isize,
//     ZMQ_INVERT_MATCHING = zmq::ZMQ_INVERT_MATCHING as isize,
//     ZMQ_IPV6 = zmq::ZMQ_IPV6 as isize,
//     ZMQ_LINGER = zmq::ZMQ_LINGER as isize,
//     ZMQ_MAXMSGSIZE = zmq::ZMQ_MAXMSGSIZE as isize,

//     //
//     // ZMQ_METADATA = zmq::ZMQ_METADATA as isize,
//     // ZMQ_MULTICAST_HOPS = zmq::ZMQ_MULTICAST_HOPS as isize,
//     // ZMQ_MULTICAST_MAXTPDU = zmq::ZMQ_MULTICAST_MAXTPDU as isize,
//     // ZMQ_PLAIN_PASSWORD = zmq::ZMQ_PLAIN_PASSWORD as isize,
//     // ZMQ_PLAIN_SERVER = zmq::ZMQ_PLAIN_SERVER as isize,
//     // ZMQ_PLAIN_USERNAME = zmq::ZMQ_PLAIN_USERNAME as isize,
// }

/// http://api.zeromq.org/master:zmq-poll
/// define zmq_poller
#[repr(C)]
pub struct ZmqPollItem<'a> {
    // void //*socket//;
    // int //fd//;
    // short //events//;
    // short //revents//;
    socket: *mut c_void,
    fd: c_int,
    events: c_short,
    revents: c_short,
    _marker: PhantomData<&'a ZmqSocket>, //ZmqSocket是对的吗？
}

pub fn zmq_poll(items: &mut [ZmqPollItem], timeout: i64) -> ZmqResult<i32> {
    unsafe {
        let rc = zmq::zmq_poll(
            items.as_mut_ptr() as *mut zmq::zmq_pollitem_t,
            items.len() as c_int,
            timeout as c_long,
        );

        if rc == -1 {
            panic!("{}", ZmqError::from(zmq::zmq_errno()))
        };

        Ok(rc)
    }
}

impl<'a> ZmqPollItem<'a> {
    pub fn from_fd(fd: c_int, events: ZmqPollEvent) -> Self {
        ZmqPollItem {
            socket: null_mut(),
            fd,
            events: events.bits(),
            revents: 0,
            _marker: PhantomData,
        }
    }

    pub fn set_events(&mut self, events: ZmqPollEvent) {
        self.events = events.bits();
    }

    pub fn get_revents(&self) -> ZmqPollEvent {
        ZmqPollEvent::from_bits_truncate(self.revents)
    }

    pub fn is_readable(&self) -> bool {
        (self.revents & ZmqPollEvent::POLLIN.bits()) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.revents & ZmqPollEvent::POLLOUT.bits()) != 0
    }

    pub fn is_error(&self) -> bool {
        (self.revents & ZmqPollEvent::POLLERR.bits()) != 0
    }

    pub fn has_socket(&self, socket: &ZmqSocket) -> bool {
        self.socket == socket.raw
    }

    pub fn has_fd(&self, fd: c_int) -> bool {
        self.socket.is_null() && self.fd == fd
    }
}

///http://api.zeromq.org/master:zmq-poller
/// define zmq_poller_event_t
///
bitflags! {
    pub struct ZmqPollEvent :i16 {
        const POLLIN = zmq::ZMQ_POLLIN as i16;
        const POLLOUT = zmq::ZMQ_POLLOUT as i16;
        const POLLERR = zmq::ZMQ_POLLERR as i16;
    }
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
