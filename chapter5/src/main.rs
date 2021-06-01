struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool

}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { 
    fn square(size: u32) -> Rectangle { 
        Rectangle { 
            width: size,
            height: size
        }
    }
    fn area(&self) -> u32 { 
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}",user1.email);

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1, 
    };
    user2.email = String::from("newaddress@example.com");

    let user3 = User { 
        username: String::from("slight-copy of user1"),
        ..user1
    };

    let width1 = 30;
    let height1 = 50;

    println!("The area has {} square pixels.", area(width1, height1));

    let rect1 = (30,50);
    println!("The are has {} square pixels with tuple", area2(rect1));

    let rect2 = Rectangle { 
        width: 30,
        height: 50,
    };

    println!("The area of rect {:?} has {} square pixels with struct", rect2, area3(&rect2));

    println!("Area with method of struct {}", rect2.area())
}

fn build_user(email: String, username: String) -> User { 
    User { 
        email: email,
        username: username,
        active: true, 
        sign_in_count: 1
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    User { 
        email, // when variable and field name are equal
        username, 
        active: true,
        sign_in_count: 1
    }
}

fn area(width: u32, height: u32) -> u32 { 
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 { 
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}