#[allow(non_camel_case_types)]
pub type OFFSET_TYPE = f64;

use std::fs;
use std::io::{self, Write};

pub fn file_exists(filename : &str) -> bool {
    let metadata = fs::metadata(filename);

    let mut is_file = false;
    if metadata.is_ok() {
         is_file = metadata.unwrap().is_file();
    }

    is_file
}



#[allow(dead_code)]
pub fn get_offset() -> OFFSET_TYPE {
    let mut line : String = String::new();
    let offset : OFFSET_TYPE;

    print!("Specify the offset: ");
    io::stdout().flush().unwrap();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => 0,
            Err(_) => { println!("Invalid offset specified"); continue; }
        };

        line = line.trim().to_string();
        offset = line.parse::<OFFSET_TYPE>().unwrap();

        break;
    }

    offset
}

#[allow(dead_code)]
pub fn get_filename_checked() -> String {
    let mut filename : String;

    loop {
        filename = match get_filename() {
            Ok(fname) => fname,
            Err(_) => {
                println!("File does not exist");
                continue;
            }
        };

        if file_exists(&filename) {
            break;
        }
    }

    filename
}

#[allow(dead_code)]
pub fn get_filename() -> Result<String, bool> {
    let mut line : String = String::new(); 

    print!("Enter filename: ");
    std::io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut line) {
        Ok(n) => n,
        Err(_) => {return Err(false);}
    };

    line = line.trim().to_string();

    Ok(line)
}
