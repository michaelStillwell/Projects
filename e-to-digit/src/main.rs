use std::io::BufRead;

fn main() {
    println!("What digit:");
    let to = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("Result: {:.1$}", std::f64::consts::E, to);
}
