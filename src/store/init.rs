use std::fs;
use std::path::Path;
use std::process;

pub fn init_store(user: &String) {
    let store_directory_path = format!("/home/{}/.store/", user);
    let store_directory = Path::new(&store_directory_path);

    if store_directory.exists() {
        return;
    } else {
        welcome();
        fs::create_dir_all(store_directory_path).expect("Couldnt create file in your home folder.");
    }
    process::exit(0);
}

fn welcome() {
    let ascii_art = r" 
      _________ __                        
     /   _____//  |_  ___________   ____  
     \_____  \\   __\/  _ \_  __ \_/ __ \ 
     /        \|  | (  <_> )  | \/\  ___/ 
    /_______  /|__|  \____/|__|    \___  >
            \/                         \/
        ";
    println!("{}", ascii_art);
    println!("You are running store for the first time.");
    println!("I will create a store directory in your home folder.");
}
