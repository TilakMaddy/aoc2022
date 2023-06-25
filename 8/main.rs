
fn visible_from_left(forest: &Vec<Vec<isize>>) -> Vec<Vec<bool>> {

    let m = forest.len();
    let n = forest[0].len();

    let mut left_visibility = vec![vec![false; n]; m];

    let mut r = 0;
    while r < m {
        let mut c = 0;
        let mut biggest_on_left = -1;
        while c < n {
            if forest[r][c] > biggest_on_left {
                left_visibility[r][c] = true;
            }
            biggest_on_left = std::cmp::max(biggest_on_left, forest[r][c]);
            c += 1;
        }
        r += 1;
    }

    return left_visibility;

}


fn visible_from_right(forest: &Vec<Vec<isize>>) -> Vec<Vec<bool>> {

    let m = forest.len();
    let n = forest[0].len();

    let mut right_visibility = vec![vec![false; n]; m];

    let mut r = 0;
    while r < m {
        let mut biggest_on_right = -1;
        let mut c = n - 1;
        while c >= 0 {
            if forest[r][c] > biggest_on_right {
                right_visibility[r][c] = true;
            }
            biggest_on_right = std::cmp::max(biggest_on_right, forest[r][c]);
            if c == 0 {
                break;
            } else {
                c -= 1;
            }
        }
        r += 1;
    }

    return right_visibility;

}


fn visible_from_top(forest: &Vec<Vec<isize>>) -> Vec<Vec<bool>> {

    let m = forest.len();
    let n = forest[0].len();

    let mut top_visibility = vec![vec![false; n]; m];

    let mut c = 0;
    while c < n {
        let mut r = 0;
        let mut biggest_on_top = -1;
        while r < m {
            if forest[r][c] > biggest_on_top {
                top_visibility[r][c] = true;
            }
            biggest_on_top = std::cmp::max(biggest_on_top, forest[r][c]);
            r += 1;
        }
        c += 1;
    }

    return top_visibility;

}

fn visible_from_bottom(forest: &Vec<Vec<isize>>) -> Vec<Vec<bool>> {

    let m = forest.len();
    let n = forest[0].len();

    let mut bottom_visibility = vec![vec![false; n]; m];
    
    let mut c = 0;
    while c < n {
        let mut biggest_on_bottom = -1;
        let mut r = m - 1;
        while r >= 0 {
            if forest[r][c] > biggest_on_bottom {
                bottom_visibility[r][c] = true;
            }
            biggest_on_bottom = std::cmp::max(biggest_on_bottom, forest[r][c]);
            if r == 0 {
                break;
            } else {
                r -= 1;
            }
        }
        c += 1;
    }

    return bottom_visibility;

}


fn main() {
    
    let data = std::fs::read_to_string("input.txt")
        .expect("the file's absence!");

    let mut forest: Vec<Vec<isize>> = vec![];

    for line in data.lines() {
        let mut row: Vec<isize> = vec![];
        for letter in line.chars() {
            let digit = letter.to_digit(10).unwrap() as isize;
            row.push(digit);
        }
        forest.push(row);
    }

    let left = visible_from_left(&forest);
    let bottom = visible_from_bottom(&forest);
    let top = visible_from_top(&forest);
    let right = visible_from_right(&forest);

    let mut total = 0;

    let m = forest.len();
    let n = forest[0].len();
    let mut r = 0;
    while r < m {
        let mut c = 0;
        while c < n {
            if left[r][c] || right[r][c] || bottom[r][c] || top[r][c] {
                total += 1;
            }
            c += 1;
        }
        r += 1;
    }

    println!("{}", total);


}