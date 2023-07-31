use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 2, 3, 3, 1];

    let avg = average(&v);
    let med = median(&mut v);
    let modez = mode(&v);

    println!("Median of {:?} is {}", v, med);
    println!("Average of {:?} is {}", v, avg);
    println!("Mode of {:?} is {}", v, modez)
}

fn median(v: &mut Vec<i32>) -> i32 {
    let l = v.len();
    v.sort();

    v[l / 2]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for val in v {
        map.entry(*val).and_modify(|x| {*x += 1}).or_insert(1);
    }

    let mut key = 0;
    let mut value = 0;

    for (k , v) in map {
        if v > value {
            key = k;
            value = v;
        }
    }

    key
}

fn average(v: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    let l = v.len() as i32;

    for val in v {
        sum += val;
    }
    sum / l
}
