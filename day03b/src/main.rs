use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut sum = 0;

    let mut line_iterator = input.split('\n').peekable();
    while !line_iterator.peek().is_none() {
        // Iterate every 3 lines
        let first = line_iterator.next()
            .expect("fuck");
        let second = line_iterator.next()
            .expect("fuck");
        let third = line_iterator.next()
            .expect("fuck");
        let mut matching: char = 'a';

        for character in first.chars() {
            if second.contains(character) && third.contains(character) {
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
