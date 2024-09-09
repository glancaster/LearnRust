const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // must have a type with it, immutable


fn main() {
    println!("const three hours in secs {THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    let y = 5;
    let y = y + 1;

    { 
        let y = y * 2;
        println!("inner scope y {y}");
    }

     println!("outer scope y {y}");


    let spaces = "   ";
    let spaces = spaces.len();  //we can do this but mut would lock a variables type

}

