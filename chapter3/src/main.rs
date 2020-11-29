fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn integers() {
    let dec = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    println!("{} {} {} {}", dec, hex, octal, binary);
}

fn char_type() { 
    let c = 'z'; //unicode, 4 bytes
    let emoji = '😻';
    println!("{},{}", c, emoji);
}

fn numeric_operations() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainer = 43 % 5;
    println!("{} {} {} {} {}", sum, difference, product, quotient, remainer);
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_,_,z) = tup;
    println!("The value of z is: {}", z);
    println!("The value of x is: {}", tup.0);
}

fn arrays() {
    let a: [i32; 5] = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    println!("First value {}", first);
    println!("Second value {}", second);
}

fn loop_collection() {
    let a = [10,20,30];
    let mut index = 0;
    while index < 3 { 
        println!("the value is {}", a[index]);
        index += 1;
    };

    for element in a.iter() {
        println!("the value is {}", element);
    }
}

fn floatingpoint() {
    /*
    Floating-point numbers are represented according 
    to the IEEE-754 standard. The f32 type is a single-precision float,
    and f64 has double precision.*/
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{} {}", x, y)
}

fn overflow() {
    let u: u8 = 255;
    println!("{}",u)
}

fn parameters(x: u32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 { 
    5
}

fn plus_one(x: i32) -> i32 { 
    x + 1
}

fn loops() {
    let mut counter = 0;
    let result = loop { 
        counter += 1;
        if counter == 10 { 
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn if_let() {
    let take_first: bool = true;
    let n = if take_first { 6 } else { 5 };
    println!("number taken {}", n);
}

fn while_test() {
    let mut number = 3;
    while number != 0 { 
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn ranges() {
    for number in (1..4).rev() { 
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    shadowing();
    integers();
    overflow();
    floatingpoint();
    numeric_operations();
    tuple();
    char_type();
    arrays();
    parameters(5);
    let x = plus_one(five());
    println!("The value of x is {}", x);
    if x == 6 {
        println!("number evaluated correctly");
    } else { 
        println!("number evaluated not correctly");
    }

    if_let();
    loops();
    while_test();
    loop_collection();
    ranges();
}
