use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::Action;

pub fn list_all_store(action: Action) {
    let paths = fs::read_dir(action.store_path).unwrap();
    for path in paths {
        let free_path = path.unwrap().path().display().to_string();
        let striped: Vec<&str> = free_path.split('/').collect();
        println!(
            "{} {}",
            striped.last().unwrap().trim_end_matches(".json"),
            read_file_content(&free_path)
        );
    }
}

fn read_file_content(path: &String) -> i32 {
    let mut count = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(_) = line {
                count += 1;
            }
        }
    }
    match count {
        0 => count,
        _ => count - 2,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
