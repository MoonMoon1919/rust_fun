fn main() {
    println!("Hello, world!");

    another_function(23);
    print_labeled_measurement(5, 'h');
    expression_fun();
    let x = five();
    let plus = plus_one(x);

    println!("The value of x is: {x} plus 1 is {plus}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn expression_fun() {
    // Expressions don't get a semicolon at the end
    // Otherwise they become a statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
