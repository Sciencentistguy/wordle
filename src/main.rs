use std::io::Write;

const GREEN_SQUARE: char = '🟩';
const YELLOW_SQUARE: char = '🟨';
const BLACK_SQUARE: char = '⬛';

fn main() {
    print!("Please enter a word: ");
    std::io::stdout().lock().flush().unwrap();
    let word: String = text_io::read!();
    let mut w = Wordle::new(&word);
    for _ in 0..6 {
        print!("Please enter a guess: ");
        std::io::stdout().lock().flush().unwrap();
        let guess: String = text_io::read!();
        if w.guess(&guess) {
            println!("Success!");
            return;
        }
    }
    println!("The word was {}.", word);
}

struct Wordle<'a> {
    word: &'a str,
}

impl<'a> Wordle<'a> {
    fn new(word: &'a str) -> Self {
        Self { word }
    }

    fn guess(&mut self, guess: &str) -> bool {
        for (guess_char, word_char) in guess.chars().zip(self.word.chars()) {
            print!(
                "{}",
                if guess_char == word_char {
                    GREEN_SQUARE
                } else if self.word.contains(guess_char) {
                    YELLOW_SQUARE
                } else {
                    BLACK_SQUARE
                }
            )
        }
        println!();

        guess == self.word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut w = Wordle::new("hello");
        assert!(!w.guess("roate"));
        assert!(!w.guess("helps"));
        assert!(w.guess("hello"));
    }
}
