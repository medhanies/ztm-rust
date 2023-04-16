// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {

    let m = add(45, 5999);
    println!("{:?}", add(1,4));
    println!("{m}");
    added(88, 100);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn added(x: i32, y: i32) {
    let r = x + y;
    println!("{:?}", r);
}