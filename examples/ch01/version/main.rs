fn main() {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;
    unsafe {
        zmq::zmq_version(&mut major, &mut minor, &mut patch);
    }

    println!("version: {}.{}.{}", major, minor, patch);    
}