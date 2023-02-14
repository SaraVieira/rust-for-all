fn main() {
    the_cow();
}

fn the_cow() {
    let text = "sup";
    let dashes = "-".repeat(text.len() + 2);
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
