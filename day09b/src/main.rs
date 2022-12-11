use std::{time::Instant, collections::HashSet};

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]); // initialize with starting coord

    let mut rope = [(0, 0); 10];

    // true if tail is adjacent
    let check_tail_adjacent = |h: (i32, i32), t: (i32, i32)| (h.0 - t.0).abs() <= 1 && (h.1 - t.1).abs() <= 1;

    for line in input.lines() {
        let moves = line.get(2..).unwrap().parse::<i32>().unwrap();

        // do it backwards
        for _ in 0..moves {
            // change position of every knot
            // first, update the head once
            match line.get(0..1) { // to-do: fix ugly code
                Some("R") => rope[0] = (rope[0].0 + 1, rope[0].1),
                Some("L") => rope[0] = (rope[0].0 - 1, rope[0].1),
                Some("U") => rope[0] = (rope[0].0, rope[0].1 + 1),
                Some("D") => rope[0] = (rope[0].0, rope[0].1 - 1),
                Some(&_) | None => panic!("Double check your input"),
            }

            // then, update positions of other knots
            for i in 0..(rope.len() - 1) {
                // while tail not adjacent to 
                while !check_tail_adjacent(rope[i], rope[i + 1]) {
                    // change the tail's coordinates by one in the correct direction
                    if rope[i].0 - rope[i + 1].0 >= 1 {  // change x-coord
                        rope[i + 1].0 = rope[i + 1].0 + 1;
                    } else if rope[i].0 - rope[i + 1].0 <= -1 {
                        rope[i + 1].0 = rope[i + 1].0 - 1;
                    }
        
                    if rope[i].1 - rope[i + 1].1 >= 1 { // change y-coord
                        rope[i + 1].1 = rope[i + 1].1 + 1;
                    } else if rope[i].1 - rope[i + 1].1 <= -1 {
                        rope[i + 1].1 = rope[i + 1].1 - 1;
                    }
        
                    // add to visited if last in rope
                    if i + 1 == rope.len() - 1 {
                        visited.insert(rope[i + 1]);
                    }
                }
            }

        }
    }

    println!("{}", visited.len());

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
