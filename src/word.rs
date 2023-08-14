pub fn word_contains_letter(word: &str, letter: char) -> Vec<usize> {
    let mut finds: Vec<usize> = vec![];
    for (index, current_letter) in word.chars().into_iter().enumerate() {
        if current_letter == letter {
            finds.push(index);
        }
    }

    return finds;
}

pub fn print_revealed_word_letters(word: &str, indexes: &Vec<usize>) {
    let mut print_str = String::new();

    for (current_index, current_letter) in word.chars().into_iter().enumerate() {
        match indexes.contains(&current_index) {
            true => print_str.push(current_letter),
            false => print_str.push('_')
        }
    }
    print!("\n{}", print_str);
}