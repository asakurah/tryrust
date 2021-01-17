use std::fs;

pub fn begin(src_file: &str) {
    println!("#### read_to_string: {}", src_file);
    match fs::read_to_string(src_file) {
        Ok(data) => {
            println!("**success**");
            println!("{}", data);
        }
        Err(err) => {
            println!("**failure**");
            println!("{}", err);
        }
    }
}
