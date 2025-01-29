//if attempt_count == 1 {
//display_array(ascii_art, 11, 0, ASCII_HEIGHT, ASCII_WIDTH, false);
//}
//if attempt_count == 2 {
//display_array(ascii_art, 1, 0, 11, 22, true);
//display_array(ascii_art, 11, 0, ASCII_HEIGHT, ASCII_WIDTH, false);
//}
//if attempt_count == 3 {
//display_array(ascii_art, 0, 0, 2, ASCII_WIDTH, false);
//display_array(ascii_art, 1, 0, 11, 22, true);
//display_array(ascii_art, 11, 0, ASCII_HEIGHT, ASCII_WIDTH, false);
//}
//if attempt_count == 4 {
//display_array(ascii_art, 0, 0, 4, ASCII_WIDTH, false);
//display_array(ascii_art, 1, 0, 9, 22, true);
//display_array(ascii_art, 11, 0, ASCII_HEIGHT, ASCII_WIDTH, false);
//}
//if attempt_count == 5 {
// Display the whole ascii art
//display_array(ascii_art, 0, 0, ASCII_HEIGHT, ASCII_WIDTH, false);
//}

pub mod game {
    const ASCII_WIDTH: usize = 38;
    const ASCII_HEIGHT: usize = 17;

    use std::collections::HashSet;

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

    fn display_hidden_word(correct_word: &str, correct_guesses: &HashSet<char>) {
        let characters = correct_word.chars();
        let mut guess_is_correct = false;
        for correct in characters {
            if correct_guesses.contains(&correct) {
                guess_is_correct = true;
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

    fn check_win_status(correct_word: &str, correct_guesses: &HashSet<char>) -> bool {
        let mut correct_count = 0;
        for correct in correct_word.chars() {
            if correct_guesses.contains(&correct) {
                correct_count += correct.len_utf8();
            }
        }
        return correct_count == correct_word.len();
    }

    fn compare_characters(
        correct_guesses: &mut HashSet<char>,
        guess: char,
        correct_word: &str,
    ) -> bool {
        let mut found_equal = false;
        for correct in correct_word.chars() {
            if correct == guess && !correct_guesses.contains(&guess) {
                correct_guesses.insert(guess);
                found_equal = true;
            } else if correct_guesses.contains(&guess) {
                found_equal = true;
            }
        }

        return found_equal;
    }

    fn reveal_ascii_art(attempt_count: usize, ascii_art: &[[char; ASCII_WIDTH]; ASCII_HEIGHT]) {
        let stages: [(usize, usize, usize, usize, bool); 11] = [
            (0, 0, ASCII_HEIGHT, ASCII_WIDTH, false),
            (11, 0, ASCII_HEIGHT, ASCII_WIDTH, false),
            (1, 0, 11, 22, true),
            (11, 0, ASCII_HEIGHT, ASCII_WIDTH, false),
            (0, 0, 2, ASCII_WIDTH, false),
            (1, 0, 11, 22, true),
            (11, 0, ASCII_HEIGHT, ASCII_WIDTH, false),
            (0, 0, 4, ASCII_WIDTH, false),
            (1, 0, 9, 22, true),
            (11, 0, ASCII_HEIGHT, ASCII_WIDTH, false),
            (0, 0, ASCII_HEIGHT, ASCII_WIDTH, false),
        ];
        let call_counts: [usize; 6] = [0, 1, 2, 3, 3, 1];
        let call_count = call_counts[attempt_count];

        let iterations: [usize; 6] = [0, 1, 2, 4, 7, 10];
        let iterator = iterations[attempt_count];
        for i in 0..call_count {
            let (column_min, row_min, column_max, row_max, enter_new_line) = stages[iterator + i];
            display_array(
                ascii_art,
                column_min,
                row_min,
                column_max,
                row_max,
                enter_new_line,
            );
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

    fn display_used_words(all_guesses: &HashSet<char>) {
        println!("Already guessed: ");
        print!("[");
        for guess in all_guesses.iter() {
            print!("{} ", guess);
        }
        println!("]");
    }

    fn clear_screen() -> std::io::Result<()> {
        let os = std::env::consts::OS;
        let mut clear_command = "clear";
        if os == "windows" {
            clear_command = "cls";
        }
        std::process::Command::new(clear_command).status()?;
        Ok(())
    }

    pub fn start_hangman(correct_word: &str) -> std::io::Result<()> {
        let mut attempt_count = 0;
        let ascii_art_buffer = read_file_content(ASCII_ART)?;
        let ascii_art_array = write_ascii_string_to_array(&ascii_art_buffer);
        let mut correct_guesses: HashSet<char> = HashSet::new();
        let mut all_guesses: HashSet<char> = HashSet::new();
        while attempt_count < 5 {
            display_hidden_word(correct_word, &correct_guesses);
            let input = read_input();
            clear_screen()?;
            if let Ok(input) = input {
                if !compare_characters(&mut correct_guesses, input, correct_word) {
                    attempt_count += 1;
                }
                if !all_guesses.contains(&input) {
                    all_guesses.insert(input);
                }
                display_used_words(&all_guesses);

                if attempt_count > 0 {
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
    #[cfg(test)]
    mod tests {
        use std::collections::HashSet;

        fn insert_word_as_characters(word: &str) -> HashSet<char> {
            let mut guesses = HashSet::new();
            for character in word.chars() {
                guesses.insert(character);
            }
            return guesses;
        }

        use super::{check_win_status, compare_characters};
        #[test]
        fn test_check_winning_status_should_succeed() {
            let guesses = insert_word_as_characters("makkaraperunat");
            let status = check_win_status("makkaraperunat", &guesses);
            assert_eq!(true, status);
        }

        #[test]
        fn check_win_status_should_fail() {
            let guesses = insert_word_as_characters("koira");
            let status = check_win_status("saukko", &guesses);
            assert_eq!(false, status);
        }

        #[test]
        fn inserts_correct_unique_word_to_set() {
            let guess = 'm';
            let mut correct_guesses = insert_word_as_characters("sieni");

            let result = compare_characters(&mut correct_guesses, guess, "sienimetsä");
            assert_eq!(result, true);
            assert!(correct_guesses.contains(&guess));
        }

        #[test]
        fn does_not_insert_if_answer_is_wrong() {
            let guess = 'y';
            let mut correct_guesses = insert_word_as_characters("sieni");

            let result = compare_characters(&mut correct_guesses, guess, "sienimetsä");
            assert_eq!(result, false);
            assert_eq!(correct_guesses.contains(&guess), false);
        }
    }
}
