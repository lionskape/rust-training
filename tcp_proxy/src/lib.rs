#![forbid(unsafe_code)]

use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::copy;

pub fn run_proxy(port: u32, destination: String) {
    let lis = TcpListener::bind(format!("localhost:{}", port)).unwrap();
    for conn in lis.incoming() {
        let mut c1 = conn.unwrap();
        let mut s1 = TcpStream::connect(destination.as_str()).unwrap();
        let mut c2 = c1.try_clone().unwrap();
        let mut s2 = s1.try_clone().unwrap();
        thread::spawn(move || {
            copy(&mut c1, &mut s1).unwrap();
        });
        thread::spawn(move || {
            copy(&mut s2, &mut c2).unwrap();
        });
    }
}
