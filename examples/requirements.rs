use zenity::menu::requirements;

fn main() {
    match requirements::verify_requirements(vec!["uidmap", "bridge-utils"]) {
        Ok(_) => println!("All required packages are installed."),
        Err(err) => eprintln!("Error verifying requirements: {}", err),
    }
}
