use std::fs;
use std::path::Path;
use std::process;

use crate::Action;

pub fn get_item(action: Action) {
    if action.args.len() == 2 {
        println!("No store object provided");
        process::exit(0)
    }

    let store_object = &action.args[2];
    let store_path = format!("{}/{}.json", action.store_path, store_object);
    if !Path::new(&store_path).exists() {
        process::exit(0)
    }

    let content = fs::read_to_string(store_path).expect("Error reading store, check permissions.");

    //Return whole store object.
    if action.args.len() == 3 {
        println!("{}", content.trim().replace(['{', '}'], ""));
        process::exit(0)
    }

    let k = &action.args[3];
    for line in content.lines() {
        let modified = &line.replace('"', "");
        let split: Vec<&str> = modified.split(':').collect();
        if split[0] == k {
            println!("{}", split[1].trim());
            process::exit(0)
        }
    }
}
