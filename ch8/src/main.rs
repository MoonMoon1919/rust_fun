enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    // inferred type
    let _nv = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading
    // indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get()
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // for loops
    let mut v  = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // using enums to allow multiple types in a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
