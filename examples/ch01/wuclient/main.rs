#![crate_name = "wuclient"]

use async_std::future::ready;
use zmq::{
    ctx::{ZmqContext},
    socket::{ZmqSocket, ZmqSocketType},
    error::{ZmqError, ZmqResult},
    message::{ZmqMessage}
};

fn main() {
    let ctx = ZmqContext::new();
    let sock = ZmqSocket::new(&ctx, ZmqSocketType::ZMQ_SUB);
    
    println!("Connecting Weather update server...");

    assert!(sock.connect("tcp://localhost:5556").is_ok());

    let mut msg = ZmqMessage::new();

    let mut total_temp = 0;
    for update_nbr in 0..10 {

        sock.send("Hello".into(), 0);

        sock.recv(&mut msg, 0).unwrap();
        let update = String::from_utf8(msg.as_bytes().unwrap()).unwrap();
        // let update = String::from_utf8(msg.into()).unwrap();
        let chks: Vec<i64> = update.split(' ').map(|s| s.parse().unwrap()).collect();

        let zipcode = chks[0];
        let temperature = chks[1];
        let relhumidity = chks[2];

        total_temp += temperature;

        println!("zipcode: {} =  Weather: {}", zipcode, temperature);
    }

    println!("Average temperature was {}", total_temp / 10);
}