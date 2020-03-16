use colored::ColoredString;
pub use colored::Colorize;

pub const PADDING: usize = 15;

pub fn padded_message<T: Into<String>>(title: ColoredString, msg: T) -> () {
    println!("{:>padding$} {}", title, msg.into(), padding = PADDING)
}

pub fn print_error<T: Into<String>>(msg: T) -> () {
    eprintln!("{:>padding$} {}", "Error".bright_red().bold(), msg.into(), padding = PADDING)
}
