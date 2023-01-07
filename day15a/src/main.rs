use std::{time::Instant, collections::HashSet};

const Y: i32 = 2000000;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut o: HashSet<(i32, i32)> = HashSet::new();
    let mut b: HashSet<(i32, i32)> = HashSet::new();

    // do some light reading
    let mut n = [0, 0, 0, 0];
    let mut c = 0;
    let i = input.split(&['\n', '=', ',', ':']);
    for s in i {
        let parsed = s.parse::<i32>();
        if parsed.is_ok() {
            n[c] = parsed.unwrap();
            c += 1;
        }

        if c == 4 {
            b.insert((n[2], n[3]));
            o.remove(&(n[2], n[3]));
            // do some maths
            let x_s = n[0];
            let y_s = n[1];
            let m_dist = (n[0] - n[2]).abs() + (n[1] - n[3]).abs();

            let y_l = y_s - m_dist;
            let y_u = y_s + m_dist;
            if y_l <= Y && y_u >= Y {
                let y_c = if y_s > Y { y_l } else { y_u };
                let r = (y_c - Y).abs();
    
                for x in (x_s - r)..=(x_s + r) {
                    let pt = (x, Y);
                    if !b.contains(&pt) {
                        // add to set
                        o.insert(pt);
                    } // otherwise skip if equal to beacon
                }
            }
            
            c = 0;
        }
    }

    println!("{}", o.len());
    
    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
