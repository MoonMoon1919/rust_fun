use std::io;

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Call a bunch of random type specific funcs
    maths();
    bools();
    chars();
    tuples();
    arrays();
    ask_for_array_index();
}

fn maths() {
    let sum = 5 + 10;
    println!("{sum}");

    let difference = 95.5 - 4.3;
    println!("{difference}");

    let product = 4 * 30;
    println!("{product}");

    let quotient = 56.7 / 32.2;
    let truncated = -5 /3;
    println!("{quotient}, {truncated}");

    let remainder = 43 % 5;
    println!("{remainder}");
}

fn bools() {
    let t = true;

    let f: bool = false;
    println!("{t}, {f}")
}

fn chars() {
    let c = 'z';
    let z: char = 'Z';

    println!("{} {}", c, z);
}


fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("The value of y is {y}");

    let five_hundred = tup.0;
    let one = tup.2;

    println!("{five_hundred}, {one}");
}

fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let last = a[4];

    println!("{a:?}, {last}");

    // A bunch of the same numbers
    let b: [i32; 5] = [3; 5];
    let first = b[0];

    println!("{b:?}, {first}")
}


fn ask_for_array_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array number");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}")
}
