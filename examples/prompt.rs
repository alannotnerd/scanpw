use scanpw::scanpw;

fn main() {
    println!("This example has a prompt and echos keystrokes with '*'s.");

    let password = scanpw!("Password: ");

    println!("\"{}\" was entered", password);
}
