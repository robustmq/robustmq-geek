use std::{fs, path};

pub mod placement_center;

fn read_file(config_path: &String) -> String {
    if !path::Path::new(config_path).exists() {
        panic!("The configuration file does not exist.");
    }

    let content: String = fs::read_to_string(&config_path).expect(&format!(
        "Failed to read the configuration file. File path:{}.",
        config_path
    ));
    return content;
}
