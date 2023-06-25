use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_common(first: String, second: String, third: String) -> char {
    for fc in first.chars() {
        for sc in second.chars() {
            for tc in third.chars() {
                if fc == sc && sc == tc {
                    return fc;
                }
            }
        }
    }
    panic!("matching character not found !");
}

fn score(ch: char) -> u32 {
    if ch.is_ascii_uppercase() {
        let temp = ch as u32 - b'A' as u32;
        return temp + 27 as u32;
    }
    else {
        let temp = ch as u32 - b'a' as u32;
        return temp + 1 as u32;
    }

}

fn main() -> io::Result<()> {
    
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut line_reader = reader.lines();
    let mut total = 0;

    while let Some(Ok(f)) = line_reader.next() {
        let s = line_reader.next().unwrap().unwrap();
        let t = line_reader.next().unwrap().unwrap();
        total += score(find_common(f, s, t));
    }

    println!("{}", total);

    Ok(())
}
