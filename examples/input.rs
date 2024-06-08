use regex::Regex;

use zenity::menu::input::{Input, Requirements};

fn main() {
    println!("\n\nInput Preview:");

    let path = Input::new("Enter Valid Path:", Requirements::default()).start();
    let path_with_regex = Input::new(
        "Enter a Valid .toml file",
        Requirements::path()
            .set_regex(Regex::new(r"\.toml\z").unwrap())
            .set_note("", "Please Enter a Valid Path to a .toml file"),
    )
    .start();
    let regex = Input::new(
        "Enter Valid Regex (ABC):",
        Requirements::regex(Regex::new(r"^ABC$").unwrap()),
    )
    .allow_force()
    .set_default("ABC")
    .start();

    println!("Existing Path:  {:?}", path);
    println!("Path with Regex:  {:?}", path_with_regex);
    println!("Regex:  {:?}", regex);
}
