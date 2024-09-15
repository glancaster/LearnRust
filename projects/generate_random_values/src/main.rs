
use rand::Rng;
use rand_distr::{ Normal, NormalError};
use rand::distributions::{Distribution, Uniform, Alphanumeric};

fn main() -> Result<(), NormalError> {

    let mut rng = rand::thread_rng();

    let n1 : u8 = rng.gen();
    let n2 : u16 = rng.gen();
    println!("random u8: {}", n1);
    println!("random u16: {}", n2);
    println!("random u32: {}",  rng.gen::<u32>());
    println!("random i32: {}",  rng.gen::<i32>());
    println!("random f64: {}",  rng.gen::<f64>());


    println!("random integer: {}", rng.gen_range(0..10));
    println!("random float: {}", rng.gen_range(0.0..10.0));


    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }


    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2,9) distribution", v);

    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("{}", rand_string);


    Ok(())
}
