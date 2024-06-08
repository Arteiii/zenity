use zenity::menu::input::Confirm;

// init and start directly
fn main() {
    let confirm = Confirm::new("Do you want to proceed?", false).start();
    println!("\n\n {:?}", confirm)
}
