use std::io;
fn main() {
    println!("Please enter your guess : ");
    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guess : {}" , guess);
}
