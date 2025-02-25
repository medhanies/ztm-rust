// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {

    let x = 5;

    if x > 5 {
        print!(">5");
    } else if x < 5 {
        print!("<5");
    } else {
        print!("=5");
    }

    let y = 350;
    if y > 95 {
        if y > 200 {
            print!("big number");
        } else {
            print!("kinda big number");
        }
    } else {
        print!("small number");
    }
}
