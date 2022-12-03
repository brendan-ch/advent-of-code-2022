use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut sum = 0;

    for line in input.split('\n') {
        let middle = line.len() / 2;
        let first = line.get(..middle)
            .expect("fuck");
        let last = line.get(middle..)
            .expect("fuck");

        let mut matching: char = 'a';
        
        for character in first.chars() {
            if last.contains(character) {
                matching = character;
                break;
            }
        }

        // Convert to priority by ASCII value
        if matching.is_lowercase() {
            sum += matching as u32 - 96;  // -97, and shift up 1
        } else {
            sum += matching as u32 - 38;  // -65, and shift up 27
        }
    }
    println!("{sum}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
