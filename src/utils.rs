use std::{ thread::sleep, time::Duration};
use std::io::{self, Write};
pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap()
}

pub fn stall_program(seconds: u64) {
    sleep(Duration::from_secs(seconds));
}

pub fn stall_and_present_countdown(start: u32, message: Option<&'static str>) {

    for i in (1..=start).rev() {
        
        print!("\r{}: {}",message.unwrap_or_default(), i); // Print the number on the same line
        io::stdout().flush().unwrap(); // Flush to ensure immediate output
        sleep(Duration::from_secs(1)); // Pause for the specified duration
    }
    println!("\rGo!     "); // Clear the line and print "Go!"
}
