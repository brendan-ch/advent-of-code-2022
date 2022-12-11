use std::{time::Instant, collections::HashSet};

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]); // initialize with starting coord
    let mut head = (0, 0);
    let mut tail = (0, 0);

    // true if tail is adjacent
    let check_tail_adjacent = |h: (i32, i32), t: (i32, i32)| (h.0 - t.0).abs() <= 1 && (h.1 - t.1).abs() <= 1;

    for line in input.lines() {
        match line.get(0..1) { // to-do: fix ugly code
            Some("R") => head = (head.0 + line.get(2..).unwrap().parse::<i32>().unwrap(), head.1),
            Some("L") => head = (head.0 - line.get(2..).unwrap().parse::<i32>().unwrap(), head.1),
            Some("U") => head = (head.0, head.1 + line.get(2..).unwrap().parse::<i32>().unwrap()),
            Some("D") => head = (head.0, head.1 - line.get(2..).unwrap().parse::<i32>().unwrap()),
            Some(&_) | None => panic!("Double check your input"),
        }

        // while tail not adjacent to head
        while !check_tail_adjacent(head, tail) {
            // change the tail's coordinates by one in the correct direction
            if head.0 - tail.0 >= 1 {  // change x-coord
                tail.0 = tail.0 + 1;
            } else if head.0 - tail.0 <= -1 {
                tail.0 = tail.0 - 1;
            }

            if head.1 - tail.1 >= 1 { // change y-coord
                tail.1 = tail.1 + 1;
            } else if head.1 - tail.1 <= -1 {
                tail.1 = tail.1 - 1;
            }

            // add to visited
            visited.insert(tail);
        }
    }

    println!("{}", visited.len());

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
