fn main() {
    // Create an empty string
    let mut _s = String::new();

    // Convert a string literal to a string
    let data = "initial content";
    // All of these result in the same outcome
    let _s = data.to_string();
    let _s = "initial content".to_string();
    let _s = String::from("initial content");

    appending_strings();
    concatenating_strings();
    formatting_strings();
    string_iteration();
}

fn appending_strings() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}")
}

fn concatenating_strings() {
    let s1 = String::from("Hello");
    let s2 = String::from(", World");
    let s3 = s1 + &s2;
    println!("{s3}")
}

fn formatting_strings() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}")
}

fn string_iteration() {
    let s = String::from("hello, world");

    for c in s.chars() {
        println!("{c}");
    }
}
