use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn score(s: String) -> (char, u32) {
    let mid = s.len() / 2;
    for left_char in (&s[0..mid]).chars() {
        for right_char in (&s[mid..]).chars() {
            if left_char == right_char {
                if left_char.is_ascii_uppercase() {
                    let temp = left_char as u32 - b'A' as u32;
                    return (left_char, temp + 27 as u32);
                }
                else {
                    let temp = left_char as u32 - b'a' as u32;
                    return (left_char, temp + 1 as u32);
                }
            }
        }
    }
    panic!("matching character not found !");
}

fn main() -> io::Result<()> {
    
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut total_score = 0;

    for line in reader.lines() {
        let res = score(line?);
        total_score += res.1;
    }

    println!("{}", total_score);

    Ok(())
}
