use regex::Regex;
use zenity::menu::input::{valid_regex, valid_path};

fn main() {
    println!("\n\nReturn:  {}", valid_regex(Regex::new(r"^\d{3}$").unwrap()));
    println!("\n\nPath:  {:?}", valid_path());

}
