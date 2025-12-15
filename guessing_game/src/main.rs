use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
 println!("Guess the Number");
 let secret_number = rand::rng().random_range(1..=100);
 println!("The secret is {secret_number}");
 println!("Enter a Number:");
 let mut guess = String::new();
 io::stdin()
 .read_line(&mut guess)
 .expect("There is some error in reading");
println!("You guessed {guess}");
match guess.cmp(&secret_number) {
    Ordering::Less => println!("You guessed less"),
    Ordering::Greater => println!("You guessed a greater"),
    Ordering::Equal => println!("You guessed correctly")
    
}

}
