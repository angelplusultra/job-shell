use std::io::Write;
pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap()
}
