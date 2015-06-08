extern crate getopts;
extern crate rustc_serialize;

mod compilation_db;
mod cli_args;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn read_compilation_db() -> Option<Vec<compilation_db::ClangCompUnit>> {
    let file_name_maybe: Option<String> = cli_args::parse();
    match file_name_maybe {
        Some(file_name) => {
            println!("1 Opening {:?}!", file_name);
            let path = Path::new(&file_name);
            let mut db_file = match File::open(&path) {
                Ok(file) => {
                    file
                }
                Err(err) => {
                    panic!("Couldn't open {:?} because {:?}", path, err);
                }
            };
            let mut db_file_contents = String::new();
            println!("3 {:?}!", file_name);
            db_file.read_to_string(&mut db_file_contents).ok().expect("Failed to read compilation db.");
            println!("4 {:?}!", file_name);

            Some(compilation_db::parse(db_file_contents))
        }
        None => None
    }
}

fn main() {
    let db = read_compilation_db();
    println!("{0:?}", db.unwrap());
}
