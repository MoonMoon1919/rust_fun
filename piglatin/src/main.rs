use std::io;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_vowel(c: &char) -> bool {
    return VOWELS.contains(c)
}

fn pigify(word: &mut String) {
    let initial = word.remove(0);

    if !is_vowel(&initial) {
        word.push('-');
        word.push(initial);
        word.push_str("ay");
    } else {
        word.insert(0, initial);
        word.push_str("-hay")
    }
}

fn main() {
    loop {
        println!("Input a word to pigify");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let mut user_input = user_input.trim().to_string();

        pigify(&mut user_input);
        println!("Piglatin version is {user_input}")
    }
}
