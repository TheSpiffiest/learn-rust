use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:37").unwrap();
    println!("Hello, world!");
}
