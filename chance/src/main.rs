use clap::Parser;

use eikthyrnir::*;
use anyhow::Result;

/// Calculate different probabilities for success
#[derive(Parser, Debug)]
pub struct Args {
    /// The chance between 0.0 and 1.0
    chance: f64,
}

pub fn main() -> Result<()> {
    let opt = Args::parse();
    let fail_chance = 1.0 - opt.chance;
    padded_message(" Chance for Success:".bright_green().bold(), format!("{}%", opt.chance * 100.0));
    padded_message(" Chance for Failure:".bright_red().bold(), format!("{}%", fail_chance * 100.0));

    let mut percent_5000 = false;
    let mut percent_9000 = false;
    let mut percent_9900 = false;
    let mut percent_9990 = false;
    let mut percent_9999 = false;
    let mut percent_9999999 = false;
    let mut counter: i32 = 0;
    loop {
        let percent = 1.0 - (fail_chance.powi(counter));
        if !percent_5000 & (percent > 0.5) {
            padded_message("Attempts for   50.00%:".bright_blue().bold(), format!("{}", counter));
            percent_5000 = true;
        }
        if !percent_9000 & (percent > 0.9) {
            padded_message("Attempts for   90.00%:".bright_blue().bold(), format!("{}", counter));
            percent_9000 = true;
        }
        if !percent_9900 & (percent > 0.99) {
            padded_message("Attempts for   99.00%:".bright_blue().bold(), format!("{}", counter));
            percent_9900 = true;
        }
        if !percent_9990 & (percent > 0.999) {
            padded_message("Attempts for   99.90%:".bright_blue().bold(), format!("{}", counter));
            percent_9990 = true;
        }
        if !percent_9999 & (percent > 0.9999) {
            padded_message("Attempts for   99.99%:".bright_blue().bold(), format!("{}", counter));
            percent_9999 = true;
        }
        if !percent_9999999 & (percent > 0.999999) {
            padded_message("Attempts for 99.9999%:".bright_blue().bold(), format!("{}", counter));
            percent_9999999 = true;
        }
        if percent_9999999 {
            break;
        }
        counter += 1;
    }
    Ok(())
}
