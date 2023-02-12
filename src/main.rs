use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    msg: Option<String>,
    #[arg(short, long)]
    fact: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Fact {
    fact: String,
    category: String,
}

fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();

    if let Some(msg) = cli.msg {
        the_cow(msg);
    }

    if cli.fact.is_some() {
        let body =
            reqwest::blocking::get("https://useless-api.pages.dev/api/random")?.json::<Fact>()?;

        the_cow(body.fact);
    }

    Ok(())
}

fn the_cow(text: String) {
    let dashes = "-".repeat(&text.len() + 2);
    println!();
    println!();
    println!("              +{}+", dashes);
    println!("              | {} |", text);
    println!("              +{}+", dashes);
    println!("              /");
    println!("          (__)");
    println!("          (oo)");
    println!(r"   /-------\/");
    println!("  / |     ||");
    println!(" *  ||----||");
    println!("    ~~    ~~");
    println!();
    println!();
    println!();
}
