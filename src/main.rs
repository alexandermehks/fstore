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

fn main() {
    let action = Action {
        user: current_user(),
        store_path: format!("/home/{}/.store", current_user()),
        args: env::args().collect(),
    };

    init::init_store(&action.user);
    if action.args.len() == 1 {
        init::help();
        process::exit(0);
    }

    match action.args[1].to_lowercase().as_str() {
        "new" => {
            actions::create::create_new_store_object(action);
        }
        "get" => {
            actions::get::get_item(action);
        }
        "list" => {
            actions::list::list_all_store(action);
        }
        "delete" => {
            actions::delete::delete_store_object(action);
        }
        "set" => {
            actions::set::set(&action.args, action.user);
        }
        "export" => {
            actions::export::export()
        }
        _ => {
            println!("Unknown argument.");
            process::exit(1)
        }
    }
}

fn current_user() -> String {
    let out = process::Command::new("whoami")
        .output()
        .expect("somehow user couldnt be fetch from posix shell");

    let user = std::str::from_utf8(&out.stdout).expect("failed converting bytes");
    user.trim().to_string()
}
