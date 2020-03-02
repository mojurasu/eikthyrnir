use colored::{Colorize, ColoredString};

pub fn padded_message<T: Into<String>>(title: ColoredString, msg: T) -> () {
    println!("{:>padding$} {}", title, msg.into(), padding = crate::PADDING)
}

pub fn print_error<T: Into<String>>(msg: T) -> () {
    eprintln!("{:>padding$} {}", "Error".bright_red().bold(), msg.into(), padding = crate::PADDING)
}
