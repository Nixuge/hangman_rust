use std::io;

fn get_input_reuse(string: &mut String) {
    io::stdin().read_line(string).unwrap();
}

pub fn get_input() -> String {
    let mut string = String::new();
    get_input_reuse(&mut string);
    return string.trim().to_string();
}

pub fn get_letter_input() -> char {
    let mut input_string = String::new();

    while input_string.trim().len() != 1 || input_string.trim() == "_" {
        println!("please enter a letter:");
        input_string.clear();
        get_input_reuse(&mut input_string);
    }
    return input_string.chars().next().unwrap();
}