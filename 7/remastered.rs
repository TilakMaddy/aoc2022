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
    NavigateUpstream(),
    NavigateDownstream(usize),
    InsertAndNavigate(usize),
}

impl From<(&Rc<RefCell<Dir>>, String)> for Action {
    fn from(param: (&Rc<RefCell<Dir>>, String)) -> Action {
        let current = param.0;
        let target = param.1;

        if target == ".." {
            return Action::NavigateUpstream();
        } 

        match RefCell::borrow(current.borrow())
            .subdirs
            .binary_search_by_key(&(target), |s| {
                RefCell::borrow(&s.borrow()).name.clone()
            }
        ){  
            Ok(index) => { 
                return Action::NavigateDownstream(index);
            },
            Err(index) => {
                return Action::InsertAndNavigate(index);
            }
        }
    }
}

fn fetchsert_dest(current: &Rc<RefCell<Dir>>, target: String) -> Rc<RefCell<Dir>> {

    let action: Action = ((&Rc::clone(current), target.clone())).into();

    let new_destination = match action {
        Action::NavigateUpstream() => {
            Rc::clone(&RefCell::borrow(current.borrow()).parent.as_ref().unwrap())
        }
        Action::NavigateDownstream(idx) => {
            Rc::clone(&RefCell::borrow(current.borrow()).subdirs[idx])
        }
        Action::InsertAndNavigate(idx) => {
            let new_dir = Rc::new(RefCell::new(Dir {
                parent: Some(Rc::clone(current)),
                name: target,
                subdirs: vec![],
                subfiles_size: 0,
                total_size: 0,
            }));
            RefCell::borrow_mut(current.borrow()).subdirs.insert(idx, new_dir);
            Rc::clone(&RefCell::borrow(current.borrow()).subdirs[idx])
        }
    };

    return new_destination;

}

fn populate_total_size(dir: &Rc<RefCell<Dir>>, result: &mut usize, j: &mut usize) -> usize {
    let is_leaf = RefCell::borrow(&dir.borrow()).subdirs.len() == 0;
    let subfiles_size = RefCell::borrow(&dir.borrow()).subfiles_size;
    
    if is_leaf {
        RefCell::borrow_mut(&dir.borrow()).total_size = subfiles_size;
        if subfiles_size <= 100000 {
            *result += subfiles_size;
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

        if line.starts_with("$ ls") || line.starts_with("dir") {
            // we don't care about 'dir's listed in ls unless we cd into them
            continue;
        }

        if !line.starts_with("$ cd ") {
            //subfile being listed here Ex: 12314 file_name.txt
            let subfile = line.clone();
            let (filesize, _) =  subfile.split_once(" ").unwrap();
            let filesize: usize = filesize.parse::<usize>().unwrap();
            RefCell::borrow_mut(&current.borrow()).subfiles_size += filesize;
            continue;
        }

        // here, it is confirmed that we are changing directory ...
        let target = (&line[5..]).to_string(); // extract '<???>' from line "$ cd <???>" 
        let new_current = fetchsert_dest(&current, target.clone());
        current = new_current;

    };

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

