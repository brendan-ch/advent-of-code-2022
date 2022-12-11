use std::{collections::HashSet, time::Instant};

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let coords = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for r in 1..(coords.len() - 1) {
        // get largest in row
        let mut most = (r, 0);
        for c in 1..(coords[r].len() - 1) {
            // get the largest
            if coords[r][c] > coords[most.0][most.1] {
                most = (r, c);
            }
            if most.1 > 0 {  // if not on edge
                visible.insert(most);
            }
        }
        
        most = (r, coords[r].len() - 1);
        for c in (1..coords[r].len() - 1).rev() {
            // get the largest
            if coords[r][c] > coords[most.0][most.1] {
                most = (r, c);
            }
            if most.1 < coords[r].len() - 1 {  // if not on edge
                visible.insert(most);
            }
        }
    }
    
    for c in 1..(coords[0].len() - 1) {
        // get largest in row
        let mut most = (0, c);
        for r in 1..(coords.len() - 1) {
            // get the largest
            if coords[r][c] > coords[most.0][most.1] {
                most = (r, c);
            }
            if most.0 > 0 {  // if not on edge
                visible.insert(most);
            }
        }
        
        most = (coords.len() - 1, c);
        for r in (1..coords.len() - 1).rev() {
            // get the largest
            if coords[r][c] > coords[most.0][most.1] {
                most = (r, c);
            }
            if most.0 < coords.len() - 1 {  // if not on edge
                visible.insert(most);
            }
        }
    }

    println!("{}", visible.len() + (coords.len() * 2) + ((coords[0].len() - 2) * 2));

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
