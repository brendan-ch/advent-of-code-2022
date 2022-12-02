use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt").to_owned();

    let mut score = 0;
    for line in input.split('\n') {
        let mut chars = line.chars();
        let first = chars.next().unwrap();
        let mut last = chars.last().unwrap();

        match last {
            'X' => {
                score += 1;
                last = 'A';
            }
            'Y' => {
                score += 2;
                last = 'B';
            }
            'Z' => {
                score += 3;
                last = 'C';
            }
            _ => {}
        };

        if first == last {
            // tie
            score += 3;
        } else if (first == 'A' && last == 'B')
            || (first == 'B' && last == 'C')
            || (first == 'C' && last == 'A')
        {
            // win
            score += 6;    
        }
    }

    println!("{score}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
