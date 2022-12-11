use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let coords = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut best = 0;
    for r in 1..(coords.len() - 1) {
        for c in 1..(coords[r].len() - 1) {
            let value = coords[r][c];
            // look left
            let mut dist = (0, 0, 0, 0);
            for c_temp in (0..c).rev() {
                dist.0 += 1;
                if value <= coords[r][c_temp] {
                    break;
                }
            }
            // look right
            for c_temp in (c + 1)..coords.len() {
                dist.1 += 1;
                if value <= coords[r][c_temp] {
                    break;
                }
            }
            // look up
            for r_temp in (0..r).rev() {
                dist.2 += 1;
                if value <= coords[r_temp][c] {
                    break;
                }
            }
            // look down
            for r_temp in (r + 1)..coords.len() {
                dist.3 += 1;
                if value <= coords[r_temp][c] {
                    break;
                }
            }

            let score = dist.0 * dist.1 * dist.2 * dist.3;
            if score > best {
                best = score;
            }
        }
    }
    println!("{best}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
