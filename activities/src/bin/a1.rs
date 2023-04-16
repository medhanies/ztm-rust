// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {

    println!("{}", fname("Medhanie"));
    println!("{}", lname("Solomon"));
    println!("{} {}", fname("Medhanie"), lname("Solomon"));
    println!("{:?}", name());

}

fn fname(first: &str) -> &str {
    first 
}

fn lname(last: &str) -> &str {
    last
}

fn name() {
    let full = "Medhanie Solomon";
    println!("{full}");
}