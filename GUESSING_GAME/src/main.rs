use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("please input your guess.");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    let mut count: i32 = 0;

    loop {
        let mut guess: String = String::new();

        
        count += 1;
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
    
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                println!("{}", format!("You have guessed {} times", count).yellow());
                break;
            }
        }
    
    }



}
