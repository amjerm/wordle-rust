use ez_colorize::ColorizeDisplay;
use random_word;
use std::{io, process};

enum GuessCharStatus {
    Correct,
    InWord,
    Incorrect,
}

struct GuessChar {
    char: char,
    status: GuessCharStatus,
}

impl GuessChar {
    pub fn new(char: char, status: GuessCharStatus) -> Self {
        Self { char, status }
    }
}

struct Guess {
    chars: Vec<GuessChar>,
    string: String,
}

impl Guess {
    pub fn new(chars: Vec<GuessChar>, string: String) -> Self {
        Self { chars, string }
    }
}

struct Game {
    guesses: Vec<Guess>,
    word: String,
}

impl Game {
    pub fn new() -> Self {
        println!("Welcome to the game! Enter your first guess.");
        let word = random_word::gen_len(5).unwrap().to_string();
        Self {
            guesses: vec![],
            word,
        }
    }

    pub fn print_status(&self) {
        for g in &self.guesses {
            for c in &g.chars {
                match c.status {
                    GuessCharStatus::Correct => print!("[{}]", c.char.green()),
                    GuessCharStatus::InWord => print!("[{}]", c.char.yellow()),
                    GuessCharStatus::Incorrect => print!("[{}]", c.char),
                }
            }
            println!("")
        }

        println!("Guesses Left: {}/6", 6 - self.guesses.len());
    }

    pub fn guess_word(&mut self, guess: String) {
        // reject guesses that are not exactly 5 chars
        if guess.chars().count() != 5 {
            println!("Guess must be exactly 5 characters");
            self.print_status();
            return;
        }

        // reject guesses that are not real words
        self.check_word(&guess);

        self.print_status();

        // exit game after 6 guesses
        if self.guesses.len() == 6 {
            println!("You lose. The word was {}.", self.word);
            process::exit(0);
        }
    }

    fn guess_exists(&self, guess: &String) -> bool {
        for g in &self.guesses {
            if &g.string == guess {
                return true;
            }
        }
        return false;
    }

    fn check_word(&mut self, guess: &String) {
        // do not consider same word as a new guess
        if !self.guess_exists(&guess) {
            let mut new_guess_chars = vec![];
            for (i, c) in guess.chars().enumerate() {
                let guess_char: GuessChar;
                match self.word.find(c) {
                    Some(pos) => {
                        if pos == i {
                            guess_char = GuessChar::new(c, GuessCharStatus::Correct);
                        } else {
                            guess_char = GuessChar::new(c, GuessCharStatus::InWord);
                        }
                    }
                    None => guess_char = GuessChar::new(c, GuessCharStatus::Incorrect),
                };
                new_guess_chars.push(guess_char);
            }

            let new_guess = Guess::new(new_guess_chars, String::from(guess));
            self.guesses.push(new_guess);

            if guess == &self.word {
                println!("You win!!");
                self.print_status();
                process::exit(0);
            }
        }
    }
}

pub fn play_game() {
    let mut game = Game::new();

    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => game.guess_word(guess.trim().to_string()),
            Err(_) => print!("Error reading input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_and_invalid_guesses() {
        let mut game = Game::new();

        // Guess under 5 chars should be rejected
        game.guess_word(String::from("foo"));
        assert_eq!(game.guesses.len(), 0);

        // Guess over 5 chars should be rejected
        game.guess_word(String::from("potato"));
        assert_eq!(game.guesses.len(), 0);

        // Valid guess should add to guesses
        game.guess_word(String::from("spare"));
        assert_eq!(game.guesses.len(), 1);

        // Should not add to guesses if already guessed
        game.guess_word(String::from("spare"));
        assert_eq!(game.guesses.len(), 1);
    }
}
