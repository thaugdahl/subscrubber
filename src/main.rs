#[path = "setup_utils.rs"] mod setup_utils;
#[path = "scrubber.rs"] mod scrubber;
#[path = "config.rs"] mod config;
#[path = "timestamp.rs"] mod timestamp;

use config::get_configuration;

fn main() {
    let configuration = get_configuration();

    println!();
    println!("======================= SUBSCRUBBER ========================");
    println!("Chosen Time Shift: {:.3}", configuration.offset);
    println!("Input File: {}", configuration.input_file);
    println!("Output File: {}", configuration.output_file);
    println!("------------------------------------------------------------");
    println!("Please wait while your file is being processed...");
    
    scrubber::scrub(&configuration);

    println!("Done! Results written to {}", configuration.output_file);
     
    
}
