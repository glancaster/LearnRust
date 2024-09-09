fn main() {
    //println!("Hello, world!");

    // statically typed means that it needs to know types at compile time

    let guess: u32 = "52".parse().expect("not a number"); // if we didn't give type then we would
                                                          // get an error

    // scalar type => single value
    // integer / float / booleans / characters

    
    // integer i/u 8/16/32/64/128/size
    
    // float 
    let x = 2.8; // f64
    let y: f32 = 2.8; // f32

    // bool
    let t = true;
    let f: bool = false;

    // char 
    let c = 'z';
    let z: char = 'Z';


    // compound types => different values in one type
    
    //tuple 
    let tup: (i32, f64, u8) = (500, 6.2, 4);
    let five_hundred = tup.0;

    //array
    let a = [1,2,3,4,5];
    let b: [i32,5] = [1,2,3,4,5];
    let c = [3;5]; //[3,3,3,3,3]

}
