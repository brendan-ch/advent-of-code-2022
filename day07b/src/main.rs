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
        
        sizes.push(size);
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
        if line.starts_with("$ cd") {
            // change the current directory
            if &line[5..] == ".." {
                // Pop the last item
                dir_stack.pop();
            } else {
                // Add the an item
                dir_stack.push(&line[5..]);
            }
        } else if line.starts_with("dir") {
            // Read the next line with respect to the current directory
            let mut_file = files.get_mut(&dir_stack.join(FILE_DELIMITER));
            if mut_file.is_none() {
                // create a new file
                files.insert(dir_stack.join(FILE_DELIMITER), File {
                    is_directory: true,
                    pointers: vec![dir_stack.join(FILE_DELIMITER) + FILE_DELIMITER + &line[4..].to_string()],
                    size: None,
                });
            } else {
                mut_file.unwrap().pointers.push(dir_stack.join(FILE_DELIMITER) + FILE_DELIMITER + &line[4..].to_string());
            }
        } else if line.starts_with(char::is_numeric) {
            // line starts with a number
            // find the index of the space
            let space_index = line.find(" ").unwrap();
            // get the number
            let size = line[..space_index].parse::<i32>().unwrap();
            let name = &line[(space_index + 1)..];

            // mutable file from the current working directory
            let mut_file = files.get_mut(&dir_stack.join(FILE_DELIMITER));

            // add as a file
            if mut_file.is_none() {
                // create a new file
                files.insert(dir_stack.join(FILE_DELIMITER), File {
                    is_directory: true,
                    pointers: vec![dir_stack.join(FILE_DELIMITER) + FILE_DELIMITER + name],
                    size: None,
                });
            } else {
                mut_file.unwrap().pointers.push(dir_stack.join(FILE_DELIMITER) + FILE_DELIMITER + name);
            }

            // either way, create a new file
            files.insert(dir_stack.join(FILE_DELIMITER) + FILE_DELIMITER + name, File {
                is_directory: false,
                pointers: vec![],
                size: Some(size),
            });
        }
    }

    // Go through the tree
    let mut sizes: Vec<i32> = Vec::new();
    let space_taken = trace(files.get("/").unwrap(), &files, &mut sizes);

    let total_on_disk = 70000000;
    let required_for_update = 30000000;
    let available = total_on_disk - space_taken;

    let mut least = i32::MAX;
    for size in sizes.iter() {
        if &least > size && &available + size >= required_for_update {
            least = size.to_owned();
        }
    }
    println!("{:?}", least);

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
