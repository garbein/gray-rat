
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file_name = &args[2];
    println!("{:?}", file_name);
    let path = Path::new(file_name);
    if path.exists() {
        println!("file exists");
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf);

    }
    //File::create(file_name).unwrap();
}