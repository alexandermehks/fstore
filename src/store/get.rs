use std::fs;
use std::path::Path;
use std::process;

pub fn get_item(args: &Vec<String>, user: String) {
    if args.len() == 2 {
        println!("No store object provided");
        process::exit(0)
    }

    let store_object = &args[2];
    let store_path = format!("/home/{}/.store/{}.json", user, store_object);
    if !Path::new(&store_path).exists() {
        process::exit(0)
    }

    let content = fs::read_to_string(store_path).expect("Error reading store, check permissions.");

    //Return whole store object.
    if args.len() == 3 {
        println!("{}", content.trim().replace(['{', '}'], ""));
        process::exit(0)
    }

    let k = &args[3];
    for line in content.lines() {
        let modified = &line.replace('"', "");
        let split: Vec<&str> = modified.split(':').collect();
        if split[0] == k {
            println!("{}", split[1].trim());
            process::exit(0)
        }
    }
}
