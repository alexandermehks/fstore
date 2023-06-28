use std::fs;
use std::io;
use std::process;

use crate::Action;

enum Error {
    DontExistError,
}

pub fn delete_store_object(action: Action) {
    match action.args.len() {
        3 => match delete(None, action) {
            Ok(_) => println!("deleted"),
            Err(_) => eprintln!("store dont exists"),
        },
        _ => {
            let mut name_buffer = String::new();
            println!("What store to delete?");

            io::stdin().read_line(&mut name_buffer).unwrap();
            name_buffer = name_buffer.trim().to_string();

            match delete(Some(&name_buffer), action) {
                Ok(_) => println!("{} deleted", name_buffer),
                Err(_) => eprintln!("store dont exists"),
            }
        }
    }
    process::exit(0)
}

fn delete(filename: Option<&String>, action: Action) -> Result<(), Error> {
    let filename = match filename {
        Some(name) => name,
        None => &action.args[2],
    };
    let path = format!("{}/{}.json", action.store_path, filename);
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::DontExistError),
    }
}
