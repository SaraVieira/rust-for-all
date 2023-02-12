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
    println!("{:?}", cli);
    let mut text = String::new();
    let mut dashes = String::from("-");

    if let Some(msg) = cli.msg {
        let len = msg.len();
        text = msg;
        dashes = "-".repeat(&len + 2);
    }

    if cli.fact.is_some() {
        let body =
            reqwest::blocking::get("https://useless-api.pages.dev/api/random")?.json::<Fact>()?;

        let len = body.fact.len();
        text = body.fact;
        dashes = "-".repeat(&len + 2);
    }

    the_cow(dashes, text);

    Ok(())
}

fn the_cow(dashes: String, text: String) -> () {
    println!("");
    println!("");
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
    println!("");
    println!("");
    println!("");
}
