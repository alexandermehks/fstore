// Valid CSV file format are :
// Header1; Header2
// Cell1; Cell2
//
use chrono::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::Action;

pub fn export(action: Action) {
    let utc: DateTime<Utc> = Utc::now();
    let paths = fs::read_dir(&action.store_path).unwrap();
    let directory = format!("{}/exports/export-{:?}", action.store_path, utc);

    fs::create_dir_all(&directory).expect("Couldnt create file in your home folder.");

    for path in paths {
        //Path is the actual json file with key value pairs to be written to .CSV
        //following the correct format as named above.
        let path_to_store = path.unwrap().path().display().to_string();

        // Exclude exports folder
        let mut buffer: Vec<String> = Vec::new();
        match path_to_store.ends_with(".json") {
            true => {
                for line in fs::read_to_string(&path_to_store)
                    .expect("Unable to read file")
                    .lines()
                {
                    match line {
                        "}" => continue,
                        "{" => continue,
                        _ => {
                            buffer.push(format!("{}{}", line.replace(":", ";"), "\n"));
                        }
                    }
                }
            }
            false => continue,
        }

        // Create the file to be written to.
        let export_name: &str = path_to_store
            .split("/")
            .last()
            .expect("couldnt unwrap result");
        let s = format!("{}/{}", directory, export_name.replace(".json", ".csv"));
        fs::File::create(&s).expect("Couldnt create csv file during export.");

        let mut file = File::options()
            .append(true)
            .read(true)
            .open(s)
            .expect("Couldnt read file");

        for line in buffer {
            file.write_all(line.replace(":", ";").as_bytes()).unwrap();
        }
    }
}
