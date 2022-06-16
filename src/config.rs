use std::env;
use crate::setup_utils::*;
use std::io::{stdout, Write};


#[derive(Default)]
pub struct Configuration {
    pub offset : OFFSET_TYPE,
    pub input_file : String,
    pub output_file : String
}


pub fn get_configuration() -> Configuration {
    let mut config = Configuration::default();

    let cli_arguments : Vec<String> = env::args().collect();

    config.offset = match cli_arguments.get(1) {
        Some(s) => {
            s.trim().parse().unwrap_or_else(|_| { get_offset() })
        },
        None => get_offset()
    };

    config.input_file = match cli_arguments.get(2) {
        Some(s) => 
            if file_exists(s.trim()) {
                s.trim().to_string() 
            }
            else { 
                println!("Input File: ");
                stdout().flush().unwrap();
                get_filename_checked() 
            },
        None => get_filename_checked()
    };

    config.output_file = match cli_arguments.get(3) {
        Some(s) => s.trim().to_string(),
        None => get_filename().unwrap()
    };
    

    config
}
