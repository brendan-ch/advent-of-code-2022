use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut c = input[0..4].chars().collect::<Vec<char>>();
    let mut n = 0;
    let mut t = 5;

    for x in input[4..].chars() {
        c[n] = x;

        // Check that each char doesn't exist in the rest of the array
        let mut is_unique = true;
        for i in 0..c.len() {
            for j in (i + 1)..c.len() {
                if c[i] == c[j] {
                    is_unique = false;
                }
            }
        }

        if is_unique {
            println!("{t}");
            break;
        }

        n += 1;
        t += 1;
        if n >= 4 {
            n = 0;
        }
    }

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
