// Valid CSV file format are :
// Header1; Header2
// Cell1; Cell2
//
use chrono::prelude::*;
use std::fs;

use crate::Action;

pub fn export(action: Action) {
    let utc: DateTime<Utc> = Utc::now();
    let paths = fs::read_dir(&action.store_path).unwrap();
    let directory = format!("{}/exports/export-{}", action.store_path, utc);

    fs::create_dir_all(&directory).expect("Couldnt create file in your home folder.");

    for path in paths {
        //Path is the actual json file with key value pairs to be written to .CSV
        //following the correct format as named above.
        let path_to_store = path.unwrap().path().display().to_string();
        todo!();
    }
}
