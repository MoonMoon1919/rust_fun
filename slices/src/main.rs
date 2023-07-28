fn main() {
    let string = String::from("Hello world");

    let hello = &string[..5];
    let world = &string[6..];
    let hello_world = &string[..]; // Takes the whole value

    println!("{hello} {world}");
    println!("{hello_world}");

    let fw = first_word(&string);
    let sw = second_word(&string);

    println!("{fw} {sw}");

    let word = first_word(&string[0..6]);

    println!("{word}");

    let string_lit = "hello word";

    let word = first_word(&string_lit);

    // string literals are string slices already
    // So we can do this too without the reference annotation
    let word2 = first_word(string_lit);

    println!("{word}, {word2}");

    slice_array()
}

// Preferring &str over &String makes this more generic
// so it can be used on string literals and Strings
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }

    &s[..]
}

fn slice_array() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3])
}
