#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct UserRef<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

// Duplicating impl just because yes
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("jon connor"),
        sign_in_count: 0,
        active: true
    };

    let mut user2 = User{
        email: String::from("carl@example.com"),
        username: String::from("Carl Marx"),
        sign_in_count: 0,
        active: true
    };

    user2.active = false;

    let user3 = build_user( String::from("paul@gmail.com"), String::from("Paul Carl"));

    let user4 = User{
        username: String::from("xpto"),
        ..user3
    };


    println!("User1: {:?}", user1);
    println!("User1: {:?}", user2);
    // println!("User1: {:?}", user3); // got moved to user4
    println!("User1: {:?}", user4);

    let c1 = Color(-1, 10, 1);
    let red = c1.0;
    let Color(r, g, b) = c1;
    assert_eq!(r, -1);
    assert_eq!(g, 10);
    assert_eq!(b, 1);
    assert_eq!(red, -1);

    let user5 = UserRef{
        email: "someone@example.com",
        username: "jon connor",
        sign_in_count: 0,
        active: true
    };
    println!("User5: {:#?}", user5);

    let rectangle = Rectangle{width: 20, height: 30};
    let r1 = Rectangle{width: 21, height: 31};
    assert_eq!(rectangle.can_hold(&r1), false);
    assert_eq!(r1.can_hold(&rectangle), true);
    println!("Area: {}", rectangle.area());

    let square = Rectangle::square(100);
    println!("Square: {:?}", square);
}
