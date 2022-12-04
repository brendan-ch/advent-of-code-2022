use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    // In how many assignment pairs does one range overlap the other
    let mut t = 0;

    let mut n = [0, 0, 0, 0];
    let mut c = 0;
    
    input.split(&[',', '-', '\n'])
        .for_each(|x| {
            n[c] = x.parse::<i32>().unwrap();
            
            c += 1;
            if c >= 4 {
                // Do some counting
                if (n[1] - n[2] >= 0 && n[0] - n[3] < 0)
                || (n[1] - n[2] > 0 && n[0] - n[3] <= 0) {
                    t += 1;
                }

                c = 0;
            }
        });
    println!("{t}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
