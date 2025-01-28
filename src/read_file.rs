pub mod read_file {
    pub const WORDS: &str = "sanasto.txt";
    pub const ASCII_ART: &str = "ascii_art.txt";

    use rand::{rng, Rng};
    use std::fs::{self};
    pub fn read_file_content(file_path: &str) -> Result<String, std::io::Error> {
        let file_content = fs::read_to_string(file_path)?;
        Ok(file_content)
    }

    pub fn get_random_line(file_content: &str) -> &str {
        let line_count = file_content.lines().count();
        let random_line_number = rng().random_range(0..line_count);
        let mut random_line: &str = "";
        for (i, line) in file_content.lines().enumerate() {
            if i == random_line_number {
                random_line = line;
            }
        }
        return random_line;
    }
}
