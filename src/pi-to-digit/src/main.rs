use std::io::BufRead;

fn main() {
    println!("To what digit do you want your PI:");
    let to = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("Result: {:.1$}", std::f64::consts::PI, to);
}
