use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn random_number() -> i32 {
    let secret = rand::thread_rng().gen_range(1, 101);
    return secret;
}

pub fn guessing_game() {
    // get random number from function
    let secret_number = random_number();
    println!("Secret number is {}", secret_number);
    // infinite loop
    loop {
        // new empty string
        let mut guess = String::new();

        println!("{}", "Please enter a number ".bright_yellow());

        // get input from user
        // &mut guess --> if we change or assign a value we must use &mut varialbe_name
        // except --> to catch error and throw it
        io::stdin()
            .read_line(&mut guess)
            .expect("Okkali read pana mudiyala da");

        // convert from string to i32 for compare with secret number
        // match is used for compare or check some condition
        // here we use error handling instead of except, because except stop the program but this one handle the error
        let guess_number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Okkali number matum type panu da".red());
                continue;
            }
        };

        // match with ordering compare two value
        match guess_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big ".red()),
            Ordering::Less => print!("{}", "Too small \n".cyan()),
        };
    }
}

/*
key to note:-
--> import rand::Rng and std::io for user input
-->
*/
