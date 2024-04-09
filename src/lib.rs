use bracket_random::prelude::RandomNumberGenerator;
use colored::*;
use std::collections::HashSet;

const WORDS: &str = include_str!("words.data");
const LENGTH: usize = 5; //the length of words can only be 5
const TRY: usize = 6; //player has 6 chances

//arranging some words
fn arrange_word(word: &str) -> String {
    word.trim() //trimming the spaces
    .to_uppercase() //making all letters uppercase
    .chars()
    .filter(|c| c.is_ascii_uppercase()) //wordlist could contain non-words such as emojis. so we filter the words
    .collect() //collecting the characters

}

fn word_list() -> Vec<String> {
    WORDS //contains the words
    .split('\n') //splitting all words
    .map(arrange_word) //arranging all the words
    .filter(|line| line.len() == LENGTH) //filtering the words with length 5
    .collect() //collecting the words

}

pub struct Manager {
    available_words: Vec<String>,
    chosen_word: String,
    guessed_letters: HashSet<char>,
    guesses: Vec<String>,
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Manager {
    pub fn new() -> Self {
        let mut random_generate = RandomNumberGenerator::new(); //random number generator
        let dictionary = word_list();
        //we choose a random word using the random number that we generated
        let chosen_word = random_generate.random_slice_entry(&dictionary).unwrap().clone();
        Self { 
            available_words: dictionary,
            chosen_word,
            guessed_letters: HashSet::new(),
            guesses: Vec::new(), 
        }
    }

    //The guessed letters vector will be added to the guessed letters vector via self. That's why self is taken as a mutable reference.
    pub fn draw_board(&mut self) {
        self.guesses.iter().enumerate().for_each(|(_, guess)| {
            guess.chars().enumerate().for_each(|(i,c)| {

                //if the ith character in chosen_word and the c character in guess is equal then the letter is true and the place is correct.
                let row = if self.chosen_word.chars().nth(i).unwrap() == c {
                    format!("{}", c).bright_green()
                } else if self.chosen_word.chars().any(|wc| wc == c){
                    //letter is true but the place is wrong
                    format!("{}", c).bright_yellow()
                } else {
                    //if the letter is not correct, add the letter in guessed_letters and paint it red 
                    self.guessed_letters.insert(c);
                    format!("{}", c).red()
                };
                print!("{}", row);
            });
            println!();
        })
    }

    //showing invalid letters in terminal
    pub fn show_invalid_letters(&self) {
        if !self.guessed_letters.is_empty() {
            self.guessed_letters.iter().for_each(|c| print!("{}", c));
            println!("{}", "\nThese letters are not in the word you are looking for\n".to_string().cyan());
            println!()
        }
    }

    //this function take the player's guess and checks it if its length is true or is the word in the dictionary
    pub fn take_guess(&mut self) -> String {
        println!("{}", format!("Give me a word that has {} characters",LENGTH).purple());
        self.show_invalid_letters();

        let mut player_guess = String::new();
        let mut is_guess_valid = false;

        while !is_guess_valid {
            player_guess = String::new();
            std::io::stdin().read_line(&mut player_guess).unwrap();
            player_guess = arrange_word(&player_guess);

            if player_guess.len() != LENGTH {
                println!("{}", format!("Length should be {}", LENGTH).red())
            } else if !self.available_words.iter().any(|word| word == &player_guess) {
                println!("{}", "This word is not in the dictionary".red())
            }else {
                self.guesses.push(player_guess.clone());
                is_guess_valid = true;
            }
        }

        player_guess
    }

    //checks if the game is over or not
    pub fn is_game_over(&self, player_guess: &str) -> bool {

        let try_count = self.guesses.len();
        //if the player's word and the chosen word is same then the game is over and the player wins.
        if player_guess == self.chosen_word {
            println!("{}", format!("You found the word in {} tries. Congrats!",try_count).blue());
            true
        } else if try_count >= TRY {
            //if the player's try count is over, game is over and the player lost.
            println!("{}", format!("You finished all your chances. The word is: {}", self.chosen_word).bright_green());
            true
        }else {
            false
        }
    }
    
}

#[cfg(test)]
mod test {
    use crate::{arrange_word,word_list,Manager};

    #[test]
    fn manager_crated_successfully() {
        let a = Manager::new();
        assert_eq!(a.chosen_word.chars().count(),5);
        assert!(a.available_words.len() > 0);
        assert!(a.guesses.len() == 0);
    }

    #[test]
    fn arrange_word_works() {
        let word = "gol Dy  ";
        let result = arrange_word(word);
        assert_eq!(result, "GOLDY");
    }

    #[test]
    fn word_list_works(){
        let words = word_list();
        assert!(words.len() > 1);
        let count = words.iter().filter(|w| w.chars().count() != 5).count();
        assert_eq!(count,0);

    }
}
