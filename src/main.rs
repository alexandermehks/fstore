pub mod store;

use std::env;
use std::process;

fn help() {
    let string = "
Usage: store [OPTION]...

    
    new 
        creates a new store object where you can store secrets
        - store new vault

    get
        fetches the store and sends secrets to stdout
        - store get vault
        - store get vault key

    delete
        delete a store object permanently
        - store delete vault
    set
        add or update a secret to a store
        - store set vault key=value

    list
        show all stores present on your filesystem
        - store list

        ";

    println!("{}", string);



}

#[derive(Debug)]
enum Procedure {
    New,
    Get,
    Set,
    Delete,
    List,
}

fn main() {
    let user_using_process = current_user();
    store::init::init_store(&user_using_process);
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help();
        process::exit(0);
    }

    let argv = match args[1].to_lowercase().as_str() {
        "new" => Procedure::New,
        "get" => Procedure::Get,
        "delete" => Procedure::Delete,
        "list" => Procedure::List,
        "set" => Procedure::Set,
        _ => {
            println!("Unknown argument.");
            process::exit(1)
        }
    };


    match argv {
        Procedure::New => {
            store::create::create_new_store_object(&args, user_using_process);
        }
        Procedure::Get => {
            store::get::get_item(&args, user_using_process);
        }
        Procedure::Delete => {
            store::delete::delete_store_object(&args, user_using_process);
        }
        Procedure::List => {
            store::list::list_all_store(user_using_process);
        }
        Procedure::Set => {
            store::set::set(&args, user_using_process);
        }
    }
}


fn current_user() -> String{
    let out = process::Command::new("whoami")
        .output()
        .expect("somehow user couldnt be fetch from posix shell");

    let user = std::str::from_utf8(&out.stdout).expect("failed converting bytes");
    user.trim().to_string()
}

