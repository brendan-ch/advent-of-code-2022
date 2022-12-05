use std::{collections::VecDeque, time::Instant};

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut stacks: Vec<VecDeque<&str>> = vec![VecDeque::new(); 9];
    let mut instructions = false;

    for line in input.split('\n') {
        if instructions {
            // do some light reading
            let mut n: [usize; 3] = [0, 0, 0];
            let iterator = line.split(' ').filter(|x| x.parse::<i32>().is_ok());
            let mut c = 0;
            iterator.for_each(|x| {
                n[c] = x.parse::<usize>().unwrap();
                c += 1;
            });

            for _ in 0..n[0] {
                let to_move = stacks[n[1] - 1].pop_front().unwrap();
                stacks[n[2] - 1].push_front(to_move);
            }
        } else {
            // read the original layout
            // find the index of each letter
            line.match_indices(char::is_alphabetic).for_each(|x| {
                let stack_index = (x.0 - 1) / 4;

                stacks[stack_index].push_back(x.1);
            });

            if line.is_empty() {
                // move on to reading the instructions
                instructions = true;
            }
        }
    }

    // Print some characters
    stacks.iter().for_each(|stack| {
        print!("{}", stack[0]);
    });

    println!();
    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
