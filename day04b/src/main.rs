use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    // In how many assignment pairs does one range fully contain the other?
    let mut count = 0;

    for line in input.split('\n') {
        let mut split_line = line.split(',');
        let mut first_pair_iterator = split_line.next().unwrap()
            .split('-');
        let mut second_pair_iterator = split_line.next().unwrap()
            .split('-');

        let first_pair = (
            first_pair_iterator.next().unwrap().parse::<i32>().unwrap(),
            first_pair_iterator.next().unwrap().parse::<i32>().unwrap(),
        );
        
        let second_pair = (
            second_pair_iterator.next().unwrap().parse::<i32>().unwrap(),
            second_pair_iterator.next().unwrap().parse::<i32>().unwrap(),
        );

        if (first_pair.1 - second_pair.0 >= 0 && first_pair.0 - second_pair.1 < 0)
        || (first_pair.1 - second_pair.0 > 0 && first_pair.0 - second_pair.1 <= 0)
        {
            count += 1;
        }

    }
    println!("{count}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
