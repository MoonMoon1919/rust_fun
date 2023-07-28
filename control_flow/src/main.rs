fn main() {
    basic_ifs();
    bunch_of_ifs(6);
    if_in_let();
    loops();
    labeled_loop();
    while_loops();
    while_loops_indexer();
    for_loops();
    rev_it();
}

fn basic_ifs() {
    let number = 7;

    if number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    if number != 0 {
        println!("number was something other than zero...")
    }
}

fn bunch_of_ifs(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}


fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}


fn labeled_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!")
}

fn while_loops_indexer() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}


fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}")
    }
}

fn rev_it() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!")
}
