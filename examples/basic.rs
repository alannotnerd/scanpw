use scanpw::scanpw;

fn main() {
    println!("This example has no prompt and echos keystrokes with '*'s.");

    let password = scanpw!();

    println!("\"{}\" was entered", password);
}
