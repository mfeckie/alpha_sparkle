use std::env;

static ALPHABET: [&'static str; 26]  = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

fn main() {
    let args: Vec<_> = env::args().collect();

    // If we don't get enough arguments or they are for the wrong type or length, exit
    if args.len() == 1 || args[1].len() != 1 || not_alphabetic(args[1].chars().nth(0)) {
        println!("Invalid input");
        std::process::exit(1);
    }

    let input = args[1].to_uppercase();

    if input == "A".to_owned() {
        a();
    } else {
        build_pyramid(input);
    }

}

fn a() {
    println!("A");
}

fn build_pyramid(letter: String) {
    let iterations = letter_of_alphabet(letter);
    let total_width = total_width(iterations);
    let mut pyramid: Vec<String> = vec![pad_a(total_width)];

    for i in 1..iterations {
        let fixed_width = pad(&ALPHABET[i], total_width, i);
        pyramid.push(fixed_width);
    }
    for val in pyramid.iter() {
        println!("{}", val);
    }

    let (reversed, _) = pyramid.split_at(pyramid.len() - 1);
    for val in reversed.iter().rev() {
        println!("{}", val); 
    }
    println!("");
}

fn pad_a(total_width: usize) -> String {
    format!("{:^width$}", "A", width = total_width)
}

fn pad(letter: &str, total_width: usize, iteration: usize) -> String {
    let letters = format!("{letter}{padding:width$}{letter}", letter = letter, padding = "", width = internal_padding(iteration));
    format!("{letters: ^width$}", letters = letters, width = total_width)
}

fn internal_padding(n: usize) -> usize {
    let adjusted = n + 1;
    (adjusted + (adjusted - 1)) - 2
}

fn total_width(n: usize) -> usize {
    n + (n - 1)
}

fn letter_of_alphabet(input: String) -> usize {
    ALPHABET.iter().position(|letter| letter == &input).unwrap() + 1
}

fn not_alphabetic(input: Option<char>) -> bool {
    match input {
        Some(letter) => !letter.is_alphabetic(),
        None => false
    }
}

#[test]
fn b_is_two_test() {
    assert_eq!(letter_of_alphabet("B".to_owned()), 2);
}

#[test]
fn z_is_twenty_six() {
    assert_eq!(letter_of_alphabet("Z".to_owned()), 26);
}

#[test]
fn alphabetic_test() {
    assert!(!not_alphabetic(Some('a')));
}

#[test]
fn non_alphabetic_test() {
    assert!(not_alphabetic(Some('^')));
}

#[test]
fn padding_for_b_test() {
    assert_eq!(internal_padding(1), 1);
}

#[test]
fn padding_for_z_test() {
    assert_eq!(internal_padding(26), 51);
}
