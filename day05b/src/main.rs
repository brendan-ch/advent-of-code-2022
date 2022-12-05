use std::{time::Instant, collections::VecDeque};

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut stacks: Vec<VecDeque<&str>> = vec![VecDeque::new(); 9];

    let mut split = input.split("\n\n");
    for line in split.next().unwrap().split('\n') {
        // read the original layout
        // find the index of each letter
        line.match_indices(char::is_alphabetic).for_each(|x| {
            let stack_index = (x.0 - 1) / 4;

            stacks[stack_index].push_back(x.1);
        });
    }

    // do some light reading
    let mut n: [usize; 3] = [0, 0, 0];
    let iterator = split.next().unwrap().split(&[' ', '\n']);
    let mut c = 0;
    iterator.for_each(|x| {
        if x.parse::<usize>().is_ok() {
            n[c] = x.parse::<usize>().unwrap();
            c += 1;
        }

        if c >= 3 {
            for x in (0..n[0]).rev() {
                // move some boxes
                let to_move = stacks[n[1] - 1].remove(x as usize).unwrap();
                stacks[n[2] - 1].push_front(to_move);
            }

            c = 0;
        }
    });

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
