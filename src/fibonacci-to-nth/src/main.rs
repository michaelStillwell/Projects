use std::io::BufRead;

fn main() {
    let to: usize = match std::env::args().nth(1) {
        Some(v) => v.parse().unwrap(),
        None => {
            println!("Input to what digit:");
            std::io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse()
                .unwrap()
        }
    };

    let result = fib(to - 1);
    println!("Result: {:?}", result);
}

fn fib(to: usize) -> u64 {
    match to {
        0 => 1,
        1 => 1,
        _ => fib(to - 1) + fib(to - 2),
    }
}
