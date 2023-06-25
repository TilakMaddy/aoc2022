use std::fs;
use std::collections::HashSet;

// not the most efficient solution

fn main() {
    
    let data_stream = fs::read_to_string("input.txt")
        .expect("the file's absence");
    
    
    for i in 13..data_stream.len() {

        let mut prev_chars = HashSet::new();
        for j in 0..14 {
            prev_chars.insert(data_stream.chars().nth(i - j).unwrap());
        }

        if prev_chars.len() == 14 {
            println!("{}", i + 1);
            
        }

    }

}