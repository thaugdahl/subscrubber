use crate::config::*;
use crate::timestamp::*;

use std::fs::{File};
use std::io::{self, BufRead};
use std::io::prelude::*;

pub fn scrub(cfg : &Configuration) {

    let offset_time = TimeStamp::from_float(cfg.offset);


    let file_in : File = File::open(&cfg.input_file).expect("Failed to open input file for reading");
    let mut file_out : File = File::create(&cfg.output_file).unwrap();

    let lines = io::BufReader::new(file_in).lines();
    let mut current_line : String;

    for line in lines {
        current_line = line.unwrap();

        // SRT files delimit timestamps with an arrow " --> "
        let timestamps : Vec<&str> = current_line.trim().split(" --> ").collect();

        if timestamps.len() == 2 {
            // Current line contains timestamp info

            let timestamps : Vec<&str> = current_line.trim().split(" --> ").collect();

            let mut first_timestamp = TimeStamp::from_string(timestamps.get(0).unwrap());
            first_timestamp.subtract(&offset_time);

            let mut second_timestamp = TimeStamp::from_string(timestamps.get(1).unwrap());
            second_timestamp.subtract(&offset_time);
            
            let to_write = format!("{} --> {}\n", first_timestamp, second_timestamp);

            file_out.write_all(to_write.as_bytes())
                .expect(&format!("Could not write timestamp to {}", cfg.output_file));

        } else {
            let to_write = format!("{}\n", &current_line);

            file_out.write_all(to_write.as_bytes())
                .expect(&format!("Could not write to {}", cfg.output_file));
        }
    }
}
