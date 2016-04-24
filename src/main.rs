extern crate rand;
// :: calls static function
// . calls non-static ones
use std::io; // load IO library
use std::cmp::Ordering;
use rand::Rng; // Load gen_rand(...) method into scope

fn main() {
    println!("Guess the number!"); // println! is a macro

    let secret_number = rand::thread_rng().gen_range(1,101); // generate random number between 1 and 101

    loop{
        println!("Please input your Guess.");

        let mut guess = String::new(); // New mutable string

        // calling instance function read_line on a static function stdin()
        io::stdin().read_line(&mut guess) // & used to send guess by reference,, used mut to makw the reference mutable
            .expect("Failed to read line"); // .expect is being called on the returned value from .read_line()

        let guess: u32 = match guess.trim().parse() { // anotatted type u32 hints the compiler the parse should be into a u32
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // just like C#

        // Ordering::something are possible types in the Ordering enumeration
        match guess.cmp(&secret_number) { // compare with secret_number's reference
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
