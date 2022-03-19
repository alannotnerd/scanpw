use scanln::scanln;
use scanpw::scanpw;

fn main() {
    println!(
        "This example has a formatted prompt and echos keystrokes with '*'s."
    );

    let username = scanln!("Username: ");

    let password = scanpw!("Password for {}: ", username);

    println!("\"{}\" was entered", password);
}
