#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/zmq.rs"));

// #![allow(trivial_numeric_casts)]
pub mod error;
pub mod message;
pub mod socket;
pub mod zmq;
pub mod ctx;