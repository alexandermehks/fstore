use std::fs;
use std::io;
use std::path::Path;
use std::process;

use crate::Action;

enum Error {
    FileExist,
}

pub fn create_new_store_object(action: Action) {
    match action.args.len() {
        3 => {
            match write_to_store_list(None, action) {
                Ok(_) => println!("created"),
                Err(Error::FileExist) => eprintln!("already exists"),
            }
            process::exit(0);
        }
        _ => {
            let mut name_holder = String::new();
            println!("What do you want the store object to be called?");

            io::stdin().read_line(&mut name_holder).unwrap();
            name_holder = name_holder.trim().to_string();

            match write_to_store_list(Some(name_holder.to_lowercase()), action) {
                Ok(_) => println!("created"),
                Err(Error::FileExist) => eprintln!("Store {} already exists", name_holder),
            }
            process::exit(0);
        }
    }
}

fn write_to_store_list(name: Option<String>, action: Action) -> Result<(), Error> {
    let name_val = match name {
        Some(name) => name,
        None => action.args[2].to_string(),
    };
    let store_path = format!("{}/{}.json", action.store_path, name_val);
    if Path::new(&store_path).exists() {
        return Err(Error::FileExist);
    }
    fs::File::create(&store_path).expect("Couldnt create new store object.");
    Ok(())
}
