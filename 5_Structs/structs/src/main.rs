
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("some@bleh.com"),
        sign_in_count: 1,
    };
    let mut user2 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("some@bleh.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("rietn@setn.com");

    let rect1 = Rectangle {
        width : 30,
        height : 50,
    };

    println!(
        "the area of rect1 is {} pixels",
        rect1.area()
    );



}
