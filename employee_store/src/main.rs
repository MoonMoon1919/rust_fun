use std::io;
use std::collections::HashMap;

fn get_username_department(input: &str) -> (String, String) {
    let split_string = input.split_whitespace();

    let mut name = "";
    let mut department = "";

    for (i, word) in split_string.enumerate() {
        if i == 1 {
            name = word;
        } else if i == 3 {
            department = word;
        }
    }

    (name.to_string(), department.to_string())
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Input the a new user and their department");
        println!("e.g., Add Sally to Sales");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let user_input = user_input.trim().to_string();
        let (username, department) = get_username_department(&user_input);

        employees.entry(String::from(department)).and_modify(|x| {
            let idx = x.binary_search(&username).unwrap_or_else(|e| e);
            x.insert(idx, username.to_string())
        }).or_insert(vec![username]);

        println!("{:?}", employees)
    }
}
