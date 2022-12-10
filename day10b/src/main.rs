use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut c = 0;
    let mut x = 1;

    let mut stuff_to_draw = [""; 240];

    for text in input.split(&[' ', '\n']) {
        let r = (x - 1)..=(x + 1); // draw the position
        stuff_to_draw[c] = if r.contains(&(c % 40)) {"#"} else {"."};
        
        match text {
            "addx" | "noop" => {
                c += 1; // next cycle
            },
            _ => {
                x += text.parse::<i32>().unwrap() as usize;
                c += 1;
            }
        }
    }
    for i in 0..6 {
        let r = (i * 40)..((i * 40) + 40);
        println!("{}", stuff_to_draw.get(r).unwrap().join(""));
    }
    
    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
