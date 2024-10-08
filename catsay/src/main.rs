use std::io::{self, Read};

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

/// "catsay" is a small app showing a cat saying whatever you enter a the message.
#[derive(Parser)]
struct Options {
    /// What the cat says
    #[arg(default_value = "Meow!")]
    message: String,

    /// Make the cat appear dead
    #[arg(short = 'd', long = "dead")]
    dead: bool,

    /// Load the cat picture from the specific file
    #[arg(short = 'f', long = "file")]
    catfile: Option<std::path::PathBuf>,

    /// Read the message form STDIN instead of the argument
    #[arg(short = 'i', long = "stdin")]
    stdin: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let eye = if options.dead { "x" } else { "o" };

    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    match options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(&path)
                .with_context(|| format!("Could not read file {:?}", path))?;

            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", cat_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye = eye.red().bold());
            println!("    =( I )=");

            if message.to_lowercase() == "woof" {
                eprintln!("A cat shouldn't bark like a dog.")
            }
        }
    }

    Ok(())
}
