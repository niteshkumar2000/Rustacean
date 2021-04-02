struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64
}

// Not neccessarily we should put all methods and associated functions
// into one impl block
// One struct can have multiple impl blocks as well
// But putting them in all one one impl block makes sense
// Makes it more readable.
impl Rectangle {
    fn area(&self) -> u64 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function -> which does not take a self parameter
    // This is not a method
    // Mostly used for constructor to return an instance of the struct
    // sqaure fn returns Rectangle object with square dimens
    fn square(size :u64) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("nitesh156200@gmail.com"),
        username: String::from("Niteshkumar S"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("17pw24@psgtech.ac.in");

    println!("{}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let mut user2 = build_user(String::from("nitesh156200@gmail.com"), String::from("Niti"));

    println!("{}, {}, {}, {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2
    };

    user2.sign_in_count = 2;

    println!("{}, {}, {}, {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}, {}, {}", black.0, origin.0, black.1);

    let rect1 = Rectangle {
        height: 20,
        width: 10
    };

    let rect2 = Rectangle {
        height: 10,
        width: 5
    };

    let rect3 = Rectangle {
        height: 25,
        width: 15
    };

    println!("Rect 1 dimentions {:#?}", rect1);
    println!("{}", rect1.area());

    println!("Rect 2 -> {:#?}, Rect 3 -> {:#?}", rect2, rect3);
    println!("rect 1 can fit rect2? {}", rect1.can_hold(&rect2));
    println!("rect 1 can fit rect3? {}", rect1.can_hold(&rect3));

    let sqr1 = Rectangle::square(40); // This is how we use associated functions

    println!("{:#?}, Area -> {}", sqr1, sqr1.area());
}
