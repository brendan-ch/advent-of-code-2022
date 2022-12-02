use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt").to_owned();

    let mut score = 0;
    for line in input.split('\n') {
        let mut chars = line.chars();
        let first = chars.next().unwrap();
        let last = chars.last().unwrap();

        match first {
            'A' => {
                if last == 'X' {
                    score += 3;
                } else if last == 'Y' {
                    score += 4;
                } else if last == 'Z' {
                    score += 8;
                }
            },
            'B' => {
                if last == 'X' {
                    score += 1;
                } else if last == 'Y' {
                    score += 5;
                } else if last == 'Z' {
                    score += 9;
                }
            },
            'C' => {
                if last == 'X' {
                    score += 2;
                } else if last == 'Y' {
                    score += 6;
                } else if last == 'Z' {
                    score += 7;
                }
            },
            _ => {}
        }
    }

    println!("{score}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
