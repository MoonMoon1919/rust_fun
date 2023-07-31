use std::io;
use std::collections::HashMap;
use std::io::BufRead;

enum Command {
    Add {dept: String, name: String},
    List(String),
    Quit,
    All,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.split_whitespace().collect();

        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Add", name, "to", dept] => Some(Command::Add { dept: dept.to_string(), name: name.to_string() }),
            _ => None,
        }
    }
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    println!("Type 'All' to list all employees and their departments");
    println!("Type 'Add' <name> to <department> to add an employee");
    println!("Type 'List <department> to list employees of a department");
    println!("Type 'Quit' to exit");

    for line in io::stdin().lock().lines() {
        let input = line.expect("Failed to read user input");

        match Command::from_input(&input) {
            Some(Command::All) => {
                println!("{:?}", employees);
            }
            Some(Command::Quit) => break,
            Some(Command::List(dept)) => {
                let dept_employees = employees.get(&dept);
                println!("Employees for {}: {:?}", dept, dept_employees);
            }
            Some(Command::Add { dept, name }) => {
                employees.entry(String::from(&dept)).and_modify(|x| {
                    let idx = x.binary_search(&name).unwrap_or_else(|e| e);
                    x.insert(idx, name.to_string())
                }).or_insert(vec![name.to_string()]);
                println!("Added {} to department {}", name, dept);
            }
            None => println!("Input error"),
        }
    }
}
