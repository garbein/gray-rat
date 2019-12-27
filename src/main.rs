extern crate clap;

use clap::App;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

struct Config {
    tpl_path: String,
}

fn main() {
    let matches = App::new("gray-rat")
        .version("0.1")
        .args_from_usage(
            "-f, --file=[FILE] 'file name'
            <action> 'action'",
        )
        .get_matches();

    let config = Config {
        tpl_path: String::from("/data/rsworkspace/gray-rat/tpl"),
    };
    let mut file_name = "";
    if let Some(f) = matches.value_of("file") {
        file_name = f;
    }
    if let Some(action) = matches.value_of("action") {
        match action {
            "c" => create_controller(file_name, config),
            "s" => create_service(file_name, config),
            "m" => create_model(file_name, config),
            _ => println!("unknow action"),
        }
    }
}

fn create_controller(name: &str, config: Config) {
    let path = String::from("KJZ/SchoolBiz/");
    let class_name = name[0..1].to_uppercase() + &name[1..];
    let tpl_path = config.tpl_path + "/controller";
    let mut tpl_file = File::open(tpl_path).unwrap();
    let mut tpl = String::new();
    tpl_file.read_to_string(&mut tpl).unwrap();
    let content = tpl.replace("{name}", &class_name);
    let file_name = path + &class_name + ".php";
    let mut file = create_file(&file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn create_service(name: &str, config: Config) {
    let path = String::from("KJZ/SchoolBiz/apps/classes/services/");
    let class_name = name[0..1].to_uppercase() + &name[1..] + "Service";
    let tpl_path = config.tpl_path + "/service";
    let mut tpl_file = File::open(tpl_path).unwrap();
    let mut tpl = String::new();
    tpl_file.read_to_string(&mut tpl).unwrap();
    let content = tpl.replace("{name}", &class_name);
    let file_name = path + &class_name + ".php";
    let mut file = create_file(&file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn create_model(table_name: &str, config: Config) {
    let model_path = String::from("KJZ/SchoolBiz/apps/classes/models/");
    let names: Vec<&str> = table_name.split("_").collect();
    let _db_name = names[0];
    let mut model_name = String::new();
    for i in 1..names.len() {
        let first = &names[i][0..1].to_uppercase();
        model_name = model_name + first + &names[i][1..];
    }
    model_name = model_name + "Model";
    let name = model_path + &model_name + ".php";
    let mut file = create_file(&name).unwrap();
    let tpl_path = config.tpl_path + "/model";
    let mut tpl_file = File::open(tpl_path).unwrap();
    let mut tpl = String::new();
    tpl_file.read_to_string(&mut tpl).unwrap();
    let model_tpl = tpl.replace("{model}", &model_name);
    let content = model_tpl.replace("{table}", table_name);
    file.write_all(content.as_bytes()).unwrap();
}

fn create_file(file_name: &str) -> io::Result<File> {
    let base_path = String::from("/data/www/schoolbiz-service/src/");
    let s = base_path + file_name;
    let path = Path::new(&s);
    if path.exists() {
        print!("file is exists, cover it? (y/n)?:");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        if buf == "n" {
            process::exit(0);
        }
    }
    File::create(path)
}
