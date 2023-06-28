use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn exists(mut content: String, key: &str, value: &str) -> (String, bool) {
    for i in content.lines() {
        let kv: Vec<&str> = i.split(':').collect();
        if kv[0].replace('"', "") == key {
            let old = format!("{}:{}", kv[0], kv[1]);
            let new = format!("\"{}\": {}", key, value);
            content = content.replace(&old, &new);
            return (content, true);
        }
    }
    (content, false)
}

pub fn set(args: &Vec<String>, user: String) {
    // If all arguments that are needed are provided
    if args.len() > 4 {
        println!("malformed argument, aborting");
        process::exit(0);
    }
    let mut content = String::new();
    if args.len() == 4 {
        let split = args[3].split("=");
        let collection = split.collect::<Vec<&str>>();
        if collection.len() != 2{
            println!("malformed argument, aborting");
            process::exit(0);
        }
        let (store_object, key, value) = (&args[2], collection[0], collection[1]);
        let file_path = format!("/home/{}/.store/{}.json", user, store_object);
        if !Path::new(&file_path).exists() {
            eprintln!("Store object {} dont exists.", store_object);
            eprintln!("\nAdd it by using: store new {}", store_object);
            process::exit(0);
        }
        let mut file = File::options()
            .append(true)
            .read(true)
            .open(file_path)
            .expect("Couldnt read file");

        file.read_to_string(&mut content)
            .expect("Failed reading the data from the file.");

        content = content.replace("{ \n", "");
        content = content.replace('}', "");
        file.set_len(0).expect("someting went wrong");
        let (mut content, found) = exists(content, key, value);

        if !found {
            content += &format!("\"{}\": {} \n", &key.trim(), &value.trim());
        }
        file.write_all("{ \n".as_bytes())
            .expect("couldnt write to file");
        file.write_all(content.as_bytes())
            .expect("couldnt write to file");
        file.write_all("}".as_bytes())
            .expect("couldnt write to file");
    } else if args.len() == 2 {
        let mut store_object = String::new();
        let mut key = String::new();
        let mut value = String::new();

        println!("What store object to add to?");
        io::stdin().read_line(&mut store_object).unwrap();

        let file_path = format!(
            "/home/{}/.store/{}.json",
            user,
            store_object.trim()
        );
        if !Path::new(&file_path).exists() {
            eprintln!("Store object dont exists.");
            eprint!("Add it by using: store new {}", store_object);
            process::exit(0);
        }

        println!("What key to use?");
        io::stdin().read_line(&mut key).unwrap();

        print!("What value to assign to key:{}", key);
        io::stdin().read_line(&mut value).unwrap();
        let mut file = File::options()
            .append(true)
            .read(true)
            .open(file_path)
            .expect("Couldnt read file");
        file.read_to_string(&mut content)
            .expect("Failed reading the data from the file.");
        content = content.replace("{ \n", "");
        content = content.replace('}', "");
        file.set_len(0).expect("someting went wrong");
        content += &format!("\"{}\": {} \n", &key.trim(), &value.trim());
        file.write_all("{ \n".as_bytes())
            .expect("couldnt write to file");
        file.write_all(content.as_bytes())
            .expect("couldnt write to file");
        file.write_all("}".as_bytes())
            .expect("couldnt write to file");
    }
}
