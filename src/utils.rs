use std::io::{self, Write};
use std::{thread::sleep, time::Duration};
pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap()
}

pub fn stall_program(seconds: u64) {
    sleep(Duration::from_secs(seconds));
}

pub fn stall_and_present_countdown<S>(start: u32, message: Option<S>)
where
    S: Into<String> + Clone,
{
    let message = message.map(|s| s.into());
    for i in (1..=start).rev() {
        print!("\r{}: {}", message.clone().unwrap_or_default(), i); // Print the number on the same line
        io::stdout().flush().unwrap(); // Flush to ensure immediate output
        sleep(Duration::from_secs(1)); // Pause for the specified duration
    }
}
