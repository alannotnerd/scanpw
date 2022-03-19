use scanpw::scanpw;

fn main() {
    println!("This example has a prompt and doesn't echo keystrokes.");

    let echo_settings = None;

    let password = scanpw!(echo_settings, "Username: ");
    let password = scanpw!(echo_settings, "Password: ");

    println!("\"{}\" was entered", password);
}
