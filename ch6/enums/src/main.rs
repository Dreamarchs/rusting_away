#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Option<T> { //Rust doesn't have Null, but it has Option<T> which can be either. It's considered a Type as well and the compiler will check if you've handled it properly.
    None,
    Some(T)
}

enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // This is an example of an enum variant that holds data
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


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        9 => move_player(9),
        _ => println!("Do nothing!"),
    }
}

fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}

fn move_player(num_spaces: u8) {

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


