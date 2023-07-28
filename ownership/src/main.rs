fn main() {
    scope_fun();
    variables_and_data_interacting_with_move();
    data_cloning();

    let s = String::from("hello");
    takes_ownership(s);

    // s in no longer valid - it's been moved

    let x = 5;
    makes_copy(x);

    // x is still valid because it implements Copy

    let s1 = gives_ownership();
    let s2 = String::from("mine");

    let s3 = takes_and_gives_back(s2);

    // s2 is no longer valid here because it was moved to s3
    // It is moved into 'takes_and_gives_back', which returns the value
    // which is then moved to s3
    println!("{s1}, {s3}")
}

fn scope_fun() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    //different scope
    {
        let x = "foo";
        println!("{x}")
    }

    println!("{s}");
}

fn variables_and_data_interacting_with_move() {
    let x = String::from("hello");
    // moves the pointer from x to y, like a shallow copy + invalidating the variable
    let y = x;

    // x has now been moved to y, so we can only use y
    println!("{y}")
}

fn data_cloning() {
    let x = String::from("Hello");

    // Copies the data in the heap, so x is still valid
    let y = x.clone();

    println!("{x}, {y}");
}


fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_int: i32) {
    println!("{some_int}")
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    // Return and move ownership to the calling function
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // Return and moves ownership to calling function
    a_string
}
