use std::io;

fn main() {
    println!("Hello, world!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("You guess : {}", guess)
}
