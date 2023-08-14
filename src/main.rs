use utils::{get_letter_input, get_input};

use crate::word::{word_contains_letter, print_revealed_word_letters};

mod word;
mod utils;

fn main() {
    run_game_loop(8);
}

fn run_game_loop(max_fails: u8) {
    let mut used_letters: Vec<char> = vec![];
    let mut revealed_indexes: Vec<usize> = vec![];
    let mut fails: u8 = 0;
    
    println!("Enter your choosen word:");
    let word_to_find = get_input().to_lowercase();
    let word_to_find_len = word_to_find.len();

    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n"); //dirty but oh well

    loop {
        // Input checks
        let mut input = get_letter_input();
        while used_letters.contains(&input) {
            print!("Character already used! ");
            input = get_letter_input();
        }
        used_letters.push(input);

        // Word contains letter or not
        let mut containing = word_contains_letter(&word_to_find, input);
        match containing.len() == 0 {
            true => fails += 1,
            false => revealed_indexes.append(&mut containing)
        }

        // Game won or not
        if revealed_indexes.len() == word_to_find_len {
            println!("You won!");
            break;
        }

        // Too much fails or not
        if fails >= max_fails {
            println!("Game ended! you lost!");
            println!("Word to find: {}", word_to_find);
            break;
        }

        // Prints
        print_revealed_word_letters(&word_to_find, &revealed_indexes);
        println!("  ({}/{} fails)", fails, max_fails);        
    }
}