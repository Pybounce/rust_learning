use std::io;

fn main() {
    println!("-----------------");
    println!("GAME START");
    println!("-----------------");
    println!();
    play_game();
}

fn play_game() {
    let (target_letters, target_word) = gen_target();
    let mut guesses_remaining: i8 = 10;
    let mut guessed_characters = Vec::<char>::new();
    let mut correct_guesses = 0;

    while guesses_remaining > 0 {
        println!("-----------------");
        println!("Guesses remaining: {}", guesses_remaining);
        println!("Letters guessed: {}", String::from_iter(&guessed_characters));
        println!("Word: {}", get_target_guessed_letters(&guessed_characters, &target_word));
        println!("-----------------");
        let guess = get_guess(&guessed_characters);
        guessed_characters.push(guess);
        if target_letters.contains(&guess) {
            println!("Correct character guessed!");
            correct_guesses += 1;
        }
        else {
            println!("Wrong, that character is not in the target :(");
        }
        println!();
        println!();
        println!();
        println!();

        if correct_guesses == target_letters.len() {
            break;
        }
        guesses_remaining -= 1;
    }
    if guesses_remaining <= 0 {
        print_game_over();
    }
    else {
        print_game_won(&target_word, guesses_remaining);
    }
}

fn print_game_won(target_word: &String, guesses_remaining: i8) {
    println!("-----------------");
    println!("GAME WON");
    println!("You guessed the word '{0}' in {1} guesses!", &target_word, 11 - guesses_remaining);
    println!("-----------------");
    println!();
}

fn print_game_over() {
    println!("-----------------");
    println!("GAME OVER");
    println!("You ran out of guesses!");
    println!("-----------------");
    println!();
}

fn get_target_guessed_letters(guessed_characters: &Vec::<char>, target_word: &String) -> String {
    let mut output = Vec::<char>::new();

    for c in target_word.chars() {
        if guessed_characters.contains(&c) {
            output.push(c);
        }
        else {
            output.push('_');
        }
    }
    return output.into_iter().collect();
}

fn get_guess(guessed_characters: &Vec::<char>) -> char {
    loop {
        let mut user_input = String::new();
        println!("Enter your guess: ");
        match io::stdin().read_line(&mut user_input) {
            Ok(_n) => {
                if user_input.trim().len() == 1 {
                    let character_guessed = user_input.trim().chars().nth(0).unwrap();
                    if guessed_characters.contains(&character_guessed) {
                        println!();
                        println!("This character was already guessed, you absolute moron.");
                        continue;
                    }
                    else {
                        return character_guessed;
                    }
                }
            }
            Err(_e) => continue,
        }
    }
}

fn gen_target() -> (Vec::<char>, String) {
    let mut target = Vec::<char>::new();
    target.push('a');
    target.push('p');
    target.push('l');
    target.push('e');
    return (target, String::from("apple"));
}