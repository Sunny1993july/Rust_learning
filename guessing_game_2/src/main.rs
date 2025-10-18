use std::io; // Brings the standard library to scope.
use rand::Rng;
use std::cmp::Ordering;

fn main() {

loop{
   println!("Guess the number!");
   
    let secret_number = rand::thread_rng()
                                .gen_range(1..=100);

   println!("Please input your guess.");

    let mut guess = String::new(); // mutable guess variable and new String function.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line.");
    
    let guess : u32 = match guess.trim()
                            .parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

    match guess.cmp(&secret_number){

        Ordering::Less => println!("It is {} smaller.", secret_number - guess),
        Ordering::Greater => println!("It is {} bigger.", guess - secret_number),
        Ordering::Equal =>{
                println!("You Win!");
                break;
            }
                                      
    };
    
    
    //println!("You guessed {guess}.");
    println!("The secret number is {secret_number}.");
}

}
