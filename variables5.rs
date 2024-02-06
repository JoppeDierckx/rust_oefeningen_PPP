// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut number = String::from("T-H-R-E-E"); // don't change this line
    println!("Spell a Number : {}", number);
    number = String::from(3.to_string()); // don't rename this variable
    println!("Number plus two is : {}", number.parse::<i32>().unwrap() + 2);
}
