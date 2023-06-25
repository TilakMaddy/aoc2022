#![allow(unused_variables)]
#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;
use std::borrow::Borrow;

#[derive(Debug)]
struct Dir {
    parent: Option<Rc<RefCell<Dir>>>,
    name: String,
    subdirs: Vec<Rc<RefCell<Dir>>>,
    subfiles_size: usize,
    total_size: usize,
}


enum Action {
    Navigate(usize),
    InsertAndNavigate(usize),
}

fn populate_total_size(dir: &Rc<RefCell<Dir>>, result: &mut usize, j: &mut usize) -> usize {
    let is_leaf = RefCell::borrow(&dir.borrow()).subdirs.len() == 0;
    let subfiles_size = RefCell::borrow(&dir.borrow()).subfiles_size;
    
    if is_leaf {
        RefCell::borrow_mut(&dir.borrow()).total_size = subfiles_size;
        if subfiles_size <= 100000 {
            *result += subfiles_size;
            // println!("LEAF {}", subfiles_size);
        }
        if subfiles_size >= 6090134 {
            *j = min(*j, subfiles_size);
        }
        return subfiles_size;
    }

    let mut total_size_subdirs = 0;
    for subdir in &RefCell::borrow(&dir.borrow()).subdirs {
        total_size_subdirs += populate_total_size(&subdir, result, j);
    }

    RefCell::borrow_mut(&dir.borrow()).total_size = subfiles_size + total_size_subdirs;
    let total_dir_size =  total_size_subdirs + subfiles_size;
    if total_dir_size <= 100000 {
        *result += total_dir_size;
        // println!("TOTAL {}", total_dir_size);
    }
    if total_dir_size >= 6090134 {
        *j = min(*j, total_dir_size);
    }
    return total_dir_size;
}

fn main() {

    let data = std::fs::read_to_string("input.txt")
        .expect("the file's absence ");

    let mut browser = data.lines().skip(1); // skip the root directory while reading input.txt

    let root = Rc::new(RefCell::new(Dir {
        parent: None,
        name: "/".to_string(),
        subdirs: vec![],
        subfiles_size: 0,
        total_size: 0,
    }));

    let mut current = Rc::clone(&root);

    while let Some(line) = browser.next() {

        if line.starts_with("$ ls") {
            continue;
        }
 

        let mut current_has_child: Option<String> = None; // if in this loop we get a hint !
        let mut gb = false;

        if line.starts_with("$ cd ") {
            let target = &line[5..];
            if target == ".." {
                gb = true;
            }
            else {
                current_has_child = Some(target.to_string());
            }
        }
        else if !line.starts_with("$ ls") {
            
            if line.starts_with("dir") {
                let subdir = &line[4..];
               // current_has_child = Some(subdir.to_string());
               // i dont wanna navigate to this child ! also, its useless until we cd into it !!
            } else {
                let subfile = line.clone();
                let (filesize, _) =  subfile.split_once(" ").unwrap();
                let filesize: usize = filesize.parse::<usize>().unwrap();
                // dbg!(filesize);
                RefCell::borrow_mut(&current.borrow()).subfiles_size += filesize;
            }
        }  
        
        // dbg!(&pwd, &current_has_child);
        if gb {
            let parent = Rc::clone(&RefCell::borrow(&current.borrow()).parent.as_ref().unwrap());
            current = parent;
        } else if let Some(target) = current_has_child {
            let action = match RefCell::borrow(&current.borrow())
                .subdirs
                .binary_search_by_key(&(target), |s| {
                    RefCell::borrow(&s.borrow()).name.clone()
                }) {  
                Ok(index) => { 
                    Action::Navigate(index)
                },
                Err(index) => {
                    Action::InsertAndNavigate(index)
                }
            };
            match action {
                Action::Navigate(idx) => {
                    let nextc = Rc::clone(&RefCell::borrow(&current.borrow()).subdirs[idx]);
                    current = nextc;
                }
                Action::InsertAndNavigate(idx) => {
                    let new_dir = Rc::new(RefCell::new(Dir {
                        parent: Some(Rc::clone(&current)),
                        name: target,
                        subdirs: vec![],
                        subfiles_size: 0,
                        total_size: 0,
                    }));
                    RefCell::borrow_mut(&current.borrow()).subdirs.insert(idx, new_dir);
                    let nextc = Rc::clone(&RefCell::borrow(&current.borrow()).subdirs[idx]);
                    current = nextc;
                }
            }
        } 


    };

    // dbg!(&root);

    let mut ans = 0;
    let mut r = 70000000;
    let bt = populate_total_size(&Rc::clone(&root), &mut ans, &mut r);

    println!("{}", ans); // answer to first part

    let free_space_avail = 70000000 - bt;
    let desired = 30000000;

    let to_free = desired - free_space_avail;

    println!("{}, {}, {}", bt, r, to_free);


    //6090134
    

}