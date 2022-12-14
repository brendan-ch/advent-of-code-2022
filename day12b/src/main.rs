use std::{collections::{HashMap, HashSet, VecDeque}, time::Instant};

fn shortest_distance(
    starting_points: &HashSet<(usize, usize)>,
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
) -> i32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();

    // shortest distance is updated when queueing each new vertex
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    distances.insert(start, 0);

    if visited.is_empty() {
        // start with a vertex
        stack.push_back(start);
    }

    // bfs + distance tracking
    while !stack.is_empty() {
        let first = *stack.get(0).unwrap();
        visited.insert(first); // add to visited
        // append adjacent vertices to end of stack
        let adjacent = graph.get(&first).unwrap();
        // println!("Adjacent: {:?}", adjacent);
        for i in adjacent.iter() {
            if !visited.contains(i) && !stack.contains(i) {
                stack.push_back(*i);

                let distance = distances.get(i);

                match distance {
                    Some(x) => {
                        // Compare
                        if *x > distances.get(&first).unwrap() + 1 {
                            distances.insert(*i, distances.get(&first).unwrap() + 1);
                        }
                    }
                    None => {
                        distances.insert(*i, distances.get(&first).unwrap() + 1);
                    }
                }
            }
        }

        // remove first element
        stack.pop_front();
        // println!("Stack: {:?}", stack);
    }
    
    // get coordinates of all a's
    let mut min = i32::MAX;
    for pt in starting_points.iter() {
        let dist = distances.get(pt);

        match dist {
            Some(x) => {
                if (*x as i32) < min {
                    min = *x as i32;
                }
            },
            None => {}
        }

    }    

    min
}

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let v: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut starting_points: HashSet<(usize, usize)> = HashSet::new();  // collect starting points during first pass through chars

    let mut end = (0, 0);

    for r in 0..v.len() {
        for c in 0..v[r].len() {
            // do some stuff
            if v[r][c] == 'S' || v[r][c] == 'a' {
                starting_points.insert((r, c));
            } else if v[r][c] == 'E' {
                end = (r, c);
            }
            
            // do some more stuff
            let mut vec: Vec<(usize, usize)> = Vec::new();
            
            let mut curr = v[r][c] as i32;
            if v[r][c] == 'S' {
                curr = 'a' as i32;
            } else if v[r][c] == 'E' {
                curr = 'z' as i32;
            }
            
            // to-do: fix ugly code
            if r > 0 && curr - if v[r - 1][c] == 'S' { 'a' as i32 } else { v[r - 1][c] as i32 } <= 1 {
                vec.push((r - 1, c));
            }
            if r < v.len() - 1 && curr - if v[r + 1][c] == 'S' { 'a' as i32 } else { v[r + 1][c] as i32 } <= 1 {
                vec.push((r + 1, c));
            }
            if c > 0 && curr - if v[r][c - 1] == 'S' { 'a' as i32 } else { v[r][c - 1] as i32 } <= 1 {
                vec.push((r, c - 1));
            }
            if c < v[r].len() - 1 && curr - if v[r][c + 1] == 'S' { 'a' as i32 } else { v[r][c + 1] as i32 } <= 1 {
                vec.push((r, c + 1));
            }
                
            // after we're done with vec, we can move it into the hashmap
            map.insert((r, c), vec);
        }
    }

    // println!("{:?}", starting_points);
    // println!("{:?}", map);
    println!("{}", shortest_distance(&starting_points, &map, end));
    
    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
