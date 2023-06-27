use std::fs;
use std::io;
use std::path::Path;
use std::process;

pub fn create_new_store_object(args: &Vec<String>, user: String) {
    if args.len() == 3 {
        match write_to_store_list(&args[2].to_lowercase(), user) {
            Ok(_f) => {
                println!("{} created", &args[2]);
                process::exit(0);
            },
            Err("already_exist_error") => {
                eprintln!("Store {} already exists", args[2]);
            }
            Err(_) => {
                eprint!("Couldnt write to store object {}", args[2]);
                process::exit(1)
            }
        }
        process::exit(0);
    }

    let mut name_holder = String::new();
    println!("What do you want the store object to be called?");
    io::stdin().read_line(&mut name_holder).unwrap();
    name_holder = name_holder.trim().to_string();
    match write_to_store_list(&name_holder.to_lowercase(), user) {
        Ok(_f) => (),
        Err("already_exist_error") => eprintln!("Store {} already exists", name_holder),
        Err(_) => eprint!("Couldnt write to store object {}", name_holder),
    }
}
fn write_to_store_list(name: &String, user: String) -> Result<&str, &str> {
    let store_path = format!("/home/{}/.store/{}.json", user, name);
    if Path::new(&store_path).exists() {
        return Err("already_exist_error");
    }
    fs::File::create(&store_path).expect("Couldnt create new store object.");
    Ok("ok")
}
