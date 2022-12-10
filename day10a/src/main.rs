use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut c = 0;
    let mut x = 1;
    let mut s = 0;

    let mut check_c = |d: i32, x: i32| {
        if d % 40 == 20 {
            s += d * x;
        }
    };

    for text in input.split(&[' ', '\n']) {
        match text {
            "addx" => {
                c += 1;
                check_c(c, x);
                c += 1;
                check_c(c, x);
            },
            "noop" => {
                c += 1;
                check_c(c, x);
            },
            _ => x += text.parse::<i32>().unwrap(),
        }
    }
    println!("{s}");
    
    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
