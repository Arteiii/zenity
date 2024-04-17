use regex::Regex;
use zenity::menu::input::{valid_path, valid_regex};

fn main() {
    println!("Input Preview:");

    println!(
        "\n\nReturn:  {}",
        valid_regex(Regex::new(r"^\d{3}$").unwrap(), Some("369"), false)
    );
    println!("\n\nPath:  {:?}", valid_path(None, true));
}
