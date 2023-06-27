use std::fs;
use std::io;
use std::process;

pub fn delete_store_object(args: &Vec<String>, user: String) {
    if args.len() == 3 {
        match delete(&args[2], user) {
            Ok("ok") => {
                println!("{} deleted", args[2]);
                process::exit(1);
            }
            Err("error") => {
                eprintln!("store {} dont exists", args[2]);
                process::exit(0);
            }
            _ => (),
        }
        // Dont need this, but borrow checker wants to make sure i dont reuse user.
        process::exit(0);
    };

    let mut name_holder = String::new();
    println!("What store to delete?");
    io::stdin().read_line(&mut name_holder).unwrap();
    name_holder = name_holder.trim().to_string();
    match delete(&name_holder, user) {
        Ok("ok") => println!("{} deleted", name_holder),
        Err("error") => eprintln!("store {} dont exists", name_holder),
        _ => (),
    }
}

fn delete(filename: &String, user: String) -> Result<&str, &str> {
    let path = format!("/home/{}/.store/{}.json", user, filename);
    match fs::remove_file(path) {
        Ok(_) => Ok("ok"),
        Err(_) => Err("error"),
    }
}
