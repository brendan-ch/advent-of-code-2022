use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt").to_owned() + "\n";
    let now = Instant::now();

    let mut most = 0;
    let mut current = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            if current > most {
                most = current;
            }

            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("{most}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
