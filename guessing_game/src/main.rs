// use std::io;
use std::{io::stdin, cmp::Ordering};
use rand::Rng;

fn main() {

    // Welcome message
    println!("Guss a a number!");

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    /* add rand = "0.8.5" to Cargo.toml dependencies!
    "0.8.5" = ^0.8.5'*/

    /* Loop until the user guesses the correct number
    loop works like while without and condition --> while(True)*/
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
            /*mut :: make variable editable;
            guess = "HI  ".to_string();
            slice string and object string; */

        // io::stdin().read_line(&mut guess).expect("Failed!");<in other way>
        stdin().read_line(&mut guess).expect("msg");
            /* add to the string without overriding it;
            & == refrence
            refrence arr also immutable by default
            exept handels result (output of readline);*/

        // println!("You gussed: {} " , guess); <in other way>
        println!("You gussed : {guess}");

        //shadowing  --> didnt need a new variable #explain later
        let guess: u32 = match guess.trim().parse() {
            Ok(num) =>num,
            Err(_) =>{
                println!("Not a number!");
                continue;
            },
            /*Error handeling --> dont crash if the input isnt number;
            Err(_) --> any error */
        };
        /* trim()' removes leading/trailing whitespaces and '\n'
        prease --> converting type (string -> :u32); return a result [Err , ok]*/


        /* Use 'cmp()' to compare the guess with the secret number.
           Break the loop if the guess matches the secret number. */        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>
            {
                println!("Congrats!");
                break;
            }
        }
        
    }

    // Display the secret number at the end
    println!("The secret number is {secret_number}");


}
