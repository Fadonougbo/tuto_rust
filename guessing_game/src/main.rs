use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main() {

    
    println!("Guess the number!");
    
    let secret_number = rand::rng().random_range(1..=100);
    loop {
           println!("Please input your guess. ");
            
            let  mut guess = String::new(); //String.new() pour une string vide
        
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            println!("You guessed: {guess} ");


            let guess:u32=match guess.trim().parse() {
                Ok(parse_value) => parse_value,
                Err(_) => continue
            };

              match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }

            println!("The secret is: {secret_number} ");

        }


}