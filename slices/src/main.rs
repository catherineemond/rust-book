// input: a string of words separated by spaces
// output: the first word of the sequence
// logic:
// 1. split the string on non-alpha characters
// 2. return the first word or empty string if no word is found

fn first_word(s: &str) -> &str {
    s.split(|c: char| !c.is_alphabetic())
        .find(|s| !s.is_empty())
        .unwrap_or("")
}

fn main() {
    let sentence = String::from("Hello, world!");
    let word = first_word(&sentence);
    println!("The first word is: {}", word);

    let empty_string = String::from("");
    let word = first_word(&empty_string);
    println!("The first word is: {}", word);
}