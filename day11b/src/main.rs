use std::{collections::VecDeque, time::Instant};

#[derive(Debug)]
struct Operation {
    left: String,
    right: String,
    operator: char,
}

#[derive(Debug)]
struct Monke {
    items: VecDeque<i128>,
    operation: Operation,
    test: i128,
    if_true: usize,
    if_false: usize,
    times_inspected: i128,
}

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut monkeys = input
        .split("\n\n")
        .map(|monke_str| {
            let mut split = monke_str.split(&[':', '\n']);

            Monke {
                items: split
                    .nth(3)
                    .unwrap()
                    .split(", ")
                    .map(|x| x.trim().parse::<i128>().unwrap())
                    .collect(),
                operation: {
                    let mut iter = split.nth(1).unwrap().split(" ");

                    Operation {
                        left: iter.nth(3).unwrap().to_string(),
                        operator: iter.next().unwrap().chars().next().unwrap(),
                        right: iter.next().unwrap().to_string(),
                    }
                },
                test: {
                    let val = split.nth(1).unwrap().split(" ").nth(3).unwrap();
                    val.parse::<i128>().unwrap()
                },
                if_true: split
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .nth(4)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                if_false: split
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .nth(4)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                times_inspected: 0,
            }
        })
        .collect::<Vec<Monke>>();

    let mut mod_value = 1;
    for m in &monkeys {
        mod_value *= m.test;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let mut item = monkeys[i].items[0];
                monkeys[i].times_inspected += 1;

                let mut left = Ok(item);
                let mut right = Ok(item);
                if monkeys[i].operation.left != "old" {
                    left = monkeys[i].operation.left.parse::<i128>();
                }
                if monkeys[i].operation.right != "old" {
                    right = monkeys[i].operation.right.parse::<i128>();
                }

                if monkeys[i].operation.operator == '+' {
                    monkeys[i].items[0] = left.unwrap() + right.unwrap();
                } else {
                    monkeys[i].items[0] = left.unwrap() * right.unwrap();
                }

                // Test
                let new_index = if monkeys[i].items[0] % monkeys[i].test == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };

                // Move to new index
                item = monkeys[i].items[0];
                item %= mod_value;
                // if item < 0 {
                //     println!("{i}");
                // }

                // println!("Monkey {}  Worry level: {}  New index: {}  Test: {}", i, item, new_index, monkeys[i].test);
                monkeys[new_index].items.push_back(item);
                monkeys[i].items.pop_front();
            }
        }
    }

    let mut most = 0;
    let mut second_most = 0;

    for monke in &monkeys {
        if monke.times_inspected > most {
            second_most = most;
            most = monke.times_inspected;
        } else if monke.times_inspected > second_most {
            second_most = monke.times_inspected
        }
    }

    println!("{}", most * second_most);

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
