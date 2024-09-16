fn main() {
    println!("Hello, world!");
    let mut vec = vec![1,5,10,2,15];

    vec.sort();

    println!("{:?}", vec);
    let mut vec = vec![1.0,5.0,10.0,2.0,15.0];

    vec.sort_by(|a,b| a.partial_cmp(b).unwrap());

    println!("{:?}", vec);
}
