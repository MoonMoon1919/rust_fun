use rand;

use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}", add_one::add_one(num));
    println!("Hello, again! {num} plus two is {}", add_two::add_two(num));
}
