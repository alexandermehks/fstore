use std::env;
use std::process;

mod actions;
mod init;

#[derive(Debug)]
pub struct Action {
    user: String,
    store_path: String,
    args: Vec<String>,


}

#[derive(Debug)]
enum Procedure {
    New,
    Get,
    Set,
    Delete,
    List,
}

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

fn main() {
    let action = Action {
        user: current_user(),
        store_path: format!("/home/{}/.store", current_user()),
        args: env::args().collect(),
    };


    init::init_store(&action.user);
    if action.args.len() == 1 {
        help();
        process::exit(0);
    }

    let argv = match action.args[1].to_lowercase().as_str() {
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
            actions::create::create_new_store_object(action);
        },
        Procedure::Get => {
            //store::get::get_item(&args, user_using_process);
            actions::get::get_item(action);
        },
        Procedure::List => {
            actions::list::list_all_store(action);
        },
        Procedure::Delete => {
            actions::delete::delete_store_object(action);
        }
        Procedure::Set => {
            actions::set::set(&action.args, action.user);
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

