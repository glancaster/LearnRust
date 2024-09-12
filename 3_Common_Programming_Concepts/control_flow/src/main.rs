fn main() {
    let number = 3;

    if number % 2 == 0 {
        println!("condition true");
    } else if number % 3 == 0 {
        println!("div % 3");
    } else {
        println!("not div by 3,2");
    }


    let condition = true;

    let num = if condition { 5 } else { 4 };
    println!(" num is {num}");

    loop {
        println!("loop");
        break; 
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}");

    'outer : loop {
        'inner : loop {
            break 'outer;
        }
    }

    let mut liftoff = 3;

    while liftoff != 0 {
        println!("{liftoff}");
        liftoff -= 1;
    }
    println!("liftoff");

    let a = [10,20,30,40,50];

    for ele in a {
        println!("{ele}");
    }

    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("liftoff again");


}
