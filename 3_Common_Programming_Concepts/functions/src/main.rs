fn main() {
   print_function();
   print_labeled_measurements(5,'h');
   let y = {
       let x = 3;
       x + 1
   };
   println!("value of y: {y}");
   let z = return_five();
   println!("value of z: {z}");
   let a = plus_five(2);
   println!("value of a: {a}");
   
}

fn print_function() {
    println!("in another function");
}
fn print_labeled_measurements(value: u32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}
fn return_five() -> i32 {
    5
}
fn plus_five(x: i32) -> i32 {
    x + 5
}
