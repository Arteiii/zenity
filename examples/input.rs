use regex::Regex;
use zenity::menu::Input;

fn main() {
    println!("Input Preview:");

    println!(
        "\n\nReturn:  {}",
        valid_regex(
            "Enter string:",
            Regex::new(r"^\d{3}$").unwrap(),
            Some("369"),
            false
        )
    );
    println!(
        "\n\nPath:  {:?}",
        valid_path("Enter A Valid Path:", None, true)
    );
}
