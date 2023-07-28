fn main() {
    references();
    mutable_references();
    reference_scopes();
}

fn references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_references() {
    let mut s1 = String::from("hello");

    change_string(&mut s1);

    println!("{s1}")
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world")
}


fn reference_scopes() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} amd {r2}");

    // This is okay because r1 and r2 will not be used after the print above
    {
        let r3 = &mut s;
        println!("{r3}")
    }

    // This is okay because we're in a different scope
    let r4 = &mut s;
    println!("{r4}")
}
