use std::io;
use wordle::Game;

fn main() {
    let mut game = Game::new();

    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => game.guess_word(guess.trim().to_string()),
            Err(_) => print!("Error reading input"),
        }
    }
}
