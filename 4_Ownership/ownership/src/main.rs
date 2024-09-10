fn main() {

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");


    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("length of {s1} is {len}");
    
    let mut s2 = String::from("hello");
    change(&mut s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
