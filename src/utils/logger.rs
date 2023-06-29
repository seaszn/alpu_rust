
pub fn clear_console() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn log_info(message: &str) {
    println!("{}", message);
}
