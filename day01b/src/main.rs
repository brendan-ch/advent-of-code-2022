use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt").to_owned() + "\n";
    let now = Instant::now();

    println!("Hello, world!");

    let mut most = 0;
    let mut second_most = 0;
    let mut third_most = 0;
    let mut current = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            if current > most {
                third_most = second_most;
                second_most = most;
                most = current;
            } else if current > second_most {
                third_most = second_most;
                second_most = current;
            } else if current > third_most {
                third_most = current;
            }

            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    let result = most + second_most + third_most;
    println!("{result}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
