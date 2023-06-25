fn reduce_ranges(s: &str) -> Option<()> {
    let (first_range, second_range) = s.split_once(",").unwrap();

    let (fs, fe) = first_range.split_once("-").unwrap();
    let (ss, se) = second_range.split_once("-").unwrap();
    
    let (fs, fe, ss, se) = (
        fs.parse::<u32>().unwrap(),
        fe.parse::<u32>().unwrap(),
        ss.parse::<u32>().unwrap(),
        se.parse::<u32>().unwrap(),
    );
    
    if fs >= ss && fe <= se {
        return Some(());
    }
    if ss >= fs && se <= fe {
        return Some(());
    }
    return None;
}

fn main() {
    let raw_data = std::fs::read_to_string("input.txt").expect("input file to be absent");

    let special_pairs_count = raw_data
        .lines()
        .filter_map(|entry| reduce_ranges(entry))
        .count();

    println!("{}", special_pairs_count);
}
