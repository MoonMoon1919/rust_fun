fn main() {
    fib(1);
    fib(2);
    fib(3);
    fib(4);
    fib(5);
    fib(6);
}

fn fib(n: u32) {
    let mut prev = 0;
    let mut curr = 1;

    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
        println!("Current is {curr}")
    }

    println!("{n}th fibonacci number is {curr}");
}
