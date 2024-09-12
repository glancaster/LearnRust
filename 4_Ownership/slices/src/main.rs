fn main() {
    println!("Hello, world!");

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} {world}");

    // &s[0..2] == &s[..2]
    // &s[3..s.len()] == &s[3..]
    // &s[0..s.len()] == &s[..]
}
