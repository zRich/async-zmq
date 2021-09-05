#![crate_name = "wuserver"]

use async_std::future::ready;
use zmq::{
    ctx::{ZmqContext},
    socket::{ZmqSocket, ZmqSocketType},
    error::{ZmqError, ZmqResult},
    message::{ZmqMessage}
};
use rand::Rng;

fn main() {
    let ctx = ZmqContext::new();
    let sock = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_PUB);
    
    assert!(sock.bind("tcp://*:5556").is_ok());
    assert!(sock.bind("ipc://weather.ipc").is_ok());
    println!("Weather update server started...");
    let mut msg = ZmqMessage::new();

    let mut rng = rand::thread_rng();
    loop {

        let zipcode = rng.gen_range(0..100000);
        let temperature  = rng.gen_range(-80..215);
        let relhumidity  = rng.gen_range(10..50);
        let udpate = format!("{:05} {} {}", zipcode, temperature, relhumidity);

        sock.send(udpate.as_bytes().into(), 0).unwrap();
    }
}