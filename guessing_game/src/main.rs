use std::io;
use rand::Rng;
use std::process;
fn main() {
    loop
    {
        println!("Play game? [Y/N]");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap(); // Get the stdin from the user, and put it in read_string
        if user_input.trim() == "Y" {
            play_game();
        }
        else if user_input.trim() == "N" {
            process::exit(1);
        }
        else {
            println!("Invalid Input :(");
            println!();
        }
    }
}

fn play_game()
{
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen_range(0..10);
    let mut guesses = 0;
    println!("Guess number!");
    loop
    {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        let guess: u8 = match user_input.trim().parse::<u8>() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {e}");
                break;
            },
          };
        guesses += 1;
          if guess == number {
            println!("Correct! You got it in {guesses} guesses.");
            println!();
             break;
          }
          else {
            println!("Incorrect! Guess again.");
          }
    }
}
