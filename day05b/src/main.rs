use std::{time::Instant, collections::VecDeque};

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut stacks: [VecDeque<&str>; 9] = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];
    let mut instructions = false;

    for line in input.split('\n') {
        if instructions {
            // do some light reading
            let mut numbers_iterator = line.split(' ');
            numbers_iterator.next();

            let num_to_move = numbers_iterator.next().unwrap().parse::<i32>().unwrap();
            numbers_iterator.next();
            
            let initial_stack = numbers_iterator.next().unwrap().parse::<usize>().unwrap();
            numbers_iterator.next();
        
            let target_stack = numbers_iterator.next().unwrap().parse::<usize>().unwrap();

            for x in (0..num_to_move).rev() {
                // move some boxes
                let to_move = stacks[initial_stack - 1].remove(x as usize).unwrap();
                stacks[target_stack - 1].push_front(to_move);
            }

        } else {
            // read the original layout
            // find the index of each letter
            line.match_indices(char::is_alphabetic)
                .for_each(|x| {
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
    stacks.iter()
        .for_each(|stack| {
            if !stack.get(0).is_none() {
                print!("{}", stack[0]);
            }
        });

    println!();
    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
