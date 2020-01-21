
use super::{
    config::Config
};
use std::path::*;
use std::fs;
use std::io::prelude::*;
use std::env;

pub(super) fn load_config() -> Config {

    let args: Vec<String> = env::args().collect();

    let path = match args.len() {
        2 => {
            println!("Path to config: {}", args.get(1).unwrap());
            args.get(1).expect("Get config path")
        },

        x => {
            println!("Wrong number of arguments:{}\nDefault config will be applied", x);
            ""
        }
    };

    let path = Path::new(path);

    if path.exists() && path.is_file() {
        let mut file = fs::File::open(path).expect(&format!("Failed to open file: {:?}", path));
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).expect(&format!("Failed to read file content: {:?}", path));
        let config: Config = serde_json::from_str(&file_content).expect("Failed to map config");
        Config::from(config)         
    } else {
        Config::default()
    }
}

