mod hangman;
mod read_file;
use hangman::game::start_hangman;
use read_file::read_file::{get_random_line, read_file_content, ASCII_ART, WORDS};

fn main() {
    match read_file_content(WORDS) {
        Ok(file_content) => {
            let random_line = get_random_line(file_content.as_str());
            if let Err(e) = start_hangman(random_line) {
                eprintln!("Error occurred when reading {}: {}", ASCII_ART, e)
            }
        }
        Err(e) => eprintln!("Error occured when reading {}: {}", WORDS, e),
    }
}
