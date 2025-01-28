pub mod game {
    const ASCII_WIDTH: usize = 38;
    const ASCII_HEIGHT: usize = 17;

    use crate::read_file::read_file::{read_file_content, ASCII_ART};
    fn read_input() -> std::io::Result<char> {
        let result = loop {
            println!("Guess a character: ");
            let mut buffer = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut buffer) {
                break Err(e);
            }
            let mut character = buffer.as_str().chars();
            match character.next() {
                Some(c) => break Ok(c),
                None => {
                    println!("No characters provided");
                    continue;
                }
            };
        };
        result
    }

    fn display_hidden_word(correct_word: &str, correct_guesses: &Vec<char>) {
        let characters = correct_word.chars();
        let mut guess_is_correct = false;
        for correct in characters {
            for guess in correct_guesses {
                if *guess == correct {
                    guess_is_correct = true;
                }
            }
            if guess_is_correct {
                print!("{}", correct)
            } else {
                print!("_");
            }
            guess_is_correct = false;
        }
        println!();
    }

    fn check_win_status(correct_word: &str, correct_guesses: &Vec<char>) -> bool {
        let mut correct_count = 0;
        for correct in correct_word.chars() {
            for guess in correct_guesses {
                if *guess == correct {
                    correct_count += correct.len_utf8();
                }
            }
        }
        return correct_count == correct_word.len();
    }

    fn is_already_guessed(character: char, list: &Vec<char>) -> bool {
        for element in list {
            if character == *element {
                return true;
            }
        }
        return false;
    }

    fn compare_characters(
        correct_guesses: &mut Vec<char>,
        guess: char,
        correct_word: &str,
    ) -> bool {
        let mut found_equal = false;
        for correct in correct_word.chars() {
            if correct == guess && !is_already_guessed(guess, &correct_guesses) {
                correct_guesses.push(correct);
                found_equal = true;
            } else if is_already_guessed(guess, &correct_guesses) {
                found_equal = true;
            }
        }

        return found_equal;
    }

    fn reveal_ascii_art(count: usize, ascii_art: &[[char; ASCII_WIDTH]; ASCII_HEIGHT]) {
        if count == 5 {
            display_array(ascii_art, 11, 0, ASCII_HEIGHT, ASCII_WIDTH, true);
        }
        if count == 4 {
            display_array(ascii_art, 1, 0, 11, 22, true);
            display_array(ascii_art, 11, 0, ASCII_HEIGHT, ASCII_WIDTH, true);
        }
        if count == 3 {
            display_array(ascii_art, 0, 0, 2, ASCII_WIDTH, false);
            display_array(ascii_art, 1, 0, 11, 22, true);
            display_array(ascii_art, 11, 0, ASCII_HEIGHT, ASCII_WIDTH, true);
        }
        if count == 2 {
            display_array(ascii_art, 0, 0, 4, ASCII_WIDTH, false);
            display_array(ascii_art, 1, 0, 9, 22, true);
            display_array(ascii_art, 11, 0, ASCII_HEIGHT, ASCII_WIDTH, true);
        }
        if count == 1 {
            display_array(ascii_art, 0, 0, ASCII_HEIGHT, ASCII_WIDTH, false);
        }
    }
    fn write_ascii_string_to_array(ascii_art: &str) -> [[char; ASCII_WIDTH]; ASCII_HEIGHT] {
        let mut ascii_art_array = [[' '; ASCII_WIDTH]; ASCII_HEIGHT];
        let mut characters = ascii_art.chars();
        for column in 0..ASCII_HEIGHT {
            for row in 0..ASCII_WIDTH {
                let character = characters.next();
                if let Some(character) = character {
                    ascii_art_array[column][row] = character;
                }
            }
        }
        return ascii_art_array;
    }

    fn display_array(
        array: &[[char; ASCII_WIDTH]; ASCII_HEIGHT],
        column_min: usize,
        row_min: usize,
        column_max: usize,
        row_max: usize,
        enter_new_line: bool,
    ) {
        for column in 0..ASCII_HEIGHT {
            for row in 0..ASCII_WIDTH {
                let in_range =
                    column > column_min && row > row_min && column < column_max && row < row_max;
                if in_range {
                    print!("{}", array[column][row]);
                }
            }
            let in_range = column > column_min && column < column_max;
            if in_range && enter_new_line {
                println!();
            }
        }
    }

    fn display_used_words(all_guesses: &Vec<char>) {
        println!("Already guessed: ");
        print!("[");
        for guess in all_guesses {
            print!("{} ", guess);
        }
        println!("]");
    }

    pub fn start_hangman(correct_word: &str) -> std::io::Result<()> {
        let mut attempt_count = 6;
        let ascii_art_buffer = read_file_content(ASCII_ART)?;
        let ascii_art_array = write_ascii_string_to_array(&ascii_art_buffer);
        let mut correct_guesses: Vec<char> = Vec::new();
        let mut all_guesses: Vec<char> = Vec::new();
        while attempt_count > 1 {
            display_hidden_word(correct_word, &correct_guesses);
            let input = read_input();
            std::process::Command::new("clear").status().unwrap();
            if let Ok(input) = input {
                if !compare_characters(&mut correct_guesses, input, correct_word) {
                    attempt_count -= 1;
                }
                if !is_already_guessed(input, &all_guesses) {
                    all_guesses.push(input);
                }
                display_used_words(&all_guesses);

                if attempt_count < 6 {
                    reveal_ascii_art(attempt_count, &ascii_art_array);
                }

                if check_win_status(correct_word, &correct_guesses) {
                    println!("Winner");
                    break;
                }
            } else if let Err(e) = input {
                eprintln!("Error while reading input: {}", e);
                std::process::exit(1);
            }
        }
        if attempt_count <= 1 {
            println!("Game over");
        }
        println!("Correct word: {}", correct_word);
        Ok(())
    }
}
