use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Welcome to the guess game");
    println!("Please a number");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1,101);    
    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println!("You guess {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("That's right...you are doing well"),
    }
}
