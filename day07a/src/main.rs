// love me some spaghett
// https://images.unsplash.com/photo-1627286400579-027de47e9e73?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=3687&q=80

use std::{time::Instant};
use hashbrown::HashMap;

const FILE_DELIMITER: &str = ":";

#[derive(Debug)]
struct File {
    is_directory: bool,
    pointers: Vec<String>, // point to something in hashmap
    size: Option<i32>,
}

fn trace(file: &File, files: &HashMap<String, File>, sizes: &mut Vec<i32>) -> i32 {
    if file.is_directory {
        let mut size = 0;

        // Trace again
        for x in file.pointers.iter() {
            size += trace(&mut files.get(x).unwrap().to_owned(), files, sizes);
        }
        
        if size <= 100000 {
            sizes.push(size);
        }
        return size;
    } else {
        // Return the file size
        let size = file.size.unwrap();
        return size;
    }
}

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut files: HashMap<String, File> = HashMap::new();
    // The first line is always "cd /"
    let mut dir_stack = vec![];

    for line in input.lines() {
        let wd = files.get_mut(&dir_stack.join(FILE_DELIMITER));
        // find the index of the space
        let space_index = line.find(" ").unwrap();
        let name = &line[(space_index + 1)..];

        if line.starts_with("$ cd") {
            // change the current directory
            if &line[5..] == ".." {
                // Pop the last item
                dir_stack.pop();
            } else {
                // Add the an item
                dir_stack.push(&line[5..]);
            }
        } else if !line.starts_with("$ ls") {
            // add wd as a file
            if wd.is_none() {
                // create directory
                files.insert(dir_stack.join(FILE_DELIMITER), File {
                    is_directory: true,
                    pointers: vec![dir_stack.join(FILE_DELIMITER) + FILE_DELIMITER + name],
                    size: None,
                });
            } else {
                wd.unwrap().pointers.push(dir_stack.join(FILE_DELIMITER) + FILE_DELIMITER + name);
            }

            if line.starts_with(char::is_numeric) {
                // create file
                let size = line[..space_index].parse::<i32>().unwrap();
                files.insert(dir_stack.join(FILE_DELIMITER) + FILE_DELIMITER + name, File {
                    is_directory: false,
                    pointers: vec![],
                    size: Some(size),
                });
            }
        }
    }
    // Go through the tree
    let mut sizes: Vec<i32> = Vec::new();
    trace(files.get("/").unwrap(), &files, &mut sizes);

    let mut sum = 0;
    for size in sizes.iter() {
        sum += size;
    }
    println!("{:?}", sum);

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
