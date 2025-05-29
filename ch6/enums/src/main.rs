#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Option<T> { //Rust doesn't have Null, but it has Option<T> which can be either. It's considered a Type as well and the compiler will check if you've handled it properly.
    None,
    Some(T)
}

fn main() {
    let home : IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback : IpAddr = IpAddr::V6(String::from("::1"));
    let four = &home;
    let six = &loopback;
    println!("The value of four is: {:?} and the value of six is: {:?}", four, six);

    let some_number = Some(5);
    let some_char = Some('a');

    let absent_number: Option<i32> = None; // This is how you represent a null value in Rust

    //there is a gotcha here especially since it's a type
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; This will not compile because you cannot add an i8 and an Option<i8> directly.
}
