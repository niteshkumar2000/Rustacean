/*
enum IPAddrVer {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IPAddr {
    kind: IPAddrVer,
    address: String,
}

enum Option<T> {
    Some(T),
    None,
}
*/
#[derive(Debug)]
enum UseState {
    Alaska
}

enum Coin {
    Penny,
    Quarter(UseState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);    
            25
        }
    }
}

/* fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
} */

fn main() {
    /*
    let ver_four = IPAddrVer::V4;
    let ver_six = IPAddrVer::V6;
    let home = IPAddrVer::V4(127, 0, 0, 1);
    let loopback = IPAddrVer::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");
    */
    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));

    let c = Coin::Quarter(UseState::Alaska);
    println!("{}", value_in_cents(c));

    // let five = Some(5);
    //let six = plus_one(five);
    //let none = plus_one(None);

    let some_val: u8 = 0;

    match some_val {
        1 => println!("one"),
        _ => (),
    }

    // to make the _ operator more readable we have if let
    if let 1 = some_val {
        println!("one"); 
    } else {
        println!("not one")
    }
}
