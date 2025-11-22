// use std::collections::HashSet;
use std::collections::HashSet;
use colored::Colorize;

fn main() {
	// Get binary and store the contents from the text file into a string
	let	words: &'static str = include_str!("wordlists/words.txt");

	// Split into words and collect them into a HashSet.
	let dict: HashSet<&str> = words.lines().collect();

    let mut buffer: [[char; 5]; 6] = [['_'; 5]; 6];

    // TODO: Get word of the day aka replace "harsh" with random word of dict
    let word_to_find: &str = "harsh";

    // Create a counter for each char in the word_to_find
    let mut char_counter_wtf: [u8; 26] = [0; 26];
    for char in word_to_find.chars() {
        char_counter_wtf[char as usize - 'a' as usize] += 1;
    }

    buffer[1][0] = 'a';
    buffer[1][1] = 'w';
    buffer[1][2] = 'a';
    buffer[1][3] = 'r';
    buffer[1][4] = 'd';

    let mut char_nb: usize;
    for elem in buffer {
        // Check if elem is part of the wordlist
        let current_word: String = elem.iter().collect();
        let curr: &str = current_word.as_str();
        if !dict.contains(curr) {
            println!("Word not in wordlist: {current_word}");
            continue;
        }

        // Reveal last word
        char_nb = 0;
        let mut char_counter_curr = char_counter_wtf;
        for character in elem {
            // Character in right position
            if word_to_find.chars().nth(char_nb) == Some(character) {
                print!("{}", character
                                .to_string()
                                .bold()
                                .black()
                                .on_truecolor(128, 239, 128));
            }
            // Character in word but wrong position
            // TODO: Make this work with char_coutner_curr
            else if char_counter_curr[character as usize - 'a' as usize] != 0 {
                print!("{}", character
                                .to_string()
                                .bold()
                                .black()
                                .on_truecolor(255, 206, 27));
                char_counter_curr[character as usize - 'a' as usize] -= 1;
            }
            else {
                print!("{character}");
            }
            print!(" ");
            char_nb += 1;
        }
        println!("");
    }
    println!("The word was \"{}\"", word_to_find);
}
