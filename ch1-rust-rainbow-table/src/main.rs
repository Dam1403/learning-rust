use std::fs;
use std::collections::HashMap;
use std::fs::File;

fn main() {

    let args:Vec<String> = std::env::args().collect();

    if args.len() != 2{
        println!("Usage: rust-rainbow-table  <dictionary_folder_path>");
        println!("Dictionary folder contains newline separated starting names");
        std::process::exit(0)
    }

    let folder_path= &args[1];
    let dir_iter = match fs::read_dir(folder_path){
        Ok(dir_iter) => dir_iter,
        Err(e) => return eprintln!("Error: {} {}", e,folder_path),
    };


    for entry in dir_iter{
        let entry = entry.unwrap();
        println!("Dir entry {}",entry.path().as_path().to_str().unwrap()); // Better way to do this? 
    };



}
