#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home : IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback : IpAddr = IpAddr::V6(String::from("::1"));
    let four = &home;
    let six = &loopback;
    println!("The value of four is: {:?} and the value of six is: {:?}", four, six);
}
