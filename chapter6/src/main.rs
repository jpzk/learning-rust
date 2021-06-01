enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // // method body would be defined here
    }
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin { 
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> { 
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(127, 0, 0, 1);

    let loopback2 = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));   
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;    

    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));
    value_in_cents(Coin::Quarter(UsState::Alabama));

    println!("Hello, world!");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(6) = six { 
        println!("addition worked")
    } else {
        println!("it's none here")
    }

    if let Some(6) = none { 
        println!("addition worked")
    } else {
        println!("it's none here")
    }

}
