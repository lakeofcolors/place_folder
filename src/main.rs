mod cli;
mod core;
mod config;

use anyhow::Result;
use env_logger;
use clap::Parser;

pub const LOGO: &str = r#"
     ____
    {o,o}      place_folder
    /)  )
     " "
"#;

pub fn print_logo() {
    use colored::*;
    println!("{}", LOGO.bright_yellow().bold());
    println!("{}", "For community, beauty, everyone!".cyan().bold());
}

fn main() -> Result<()> {
    env_logger::init();
    if std::env::args().len() == 1 || std::env::args().any(|a| a == "--help" || a == "-h") {
        print_logo();
    }
    let cli = cli::Cli::parse();
    core::handle_command(cli)
}
