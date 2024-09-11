
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
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



}
