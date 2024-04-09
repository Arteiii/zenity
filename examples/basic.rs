// example/basic.rs

use std::thread::sleep;
use std::time::Duration;
use zenity::spinner::MultiSpinner;

fn main() {
    println!("println test");
    scope_example();
}

fn scope_example() {
    // create a LoadingAnimation instance using one of the predefined animations
    let spinner = MultiSpinner::default();

    // optional:
    spinner.set_text(&spinner.get_uid(), "Loading...".to_string()); // sets the text to "Loading..."

    sleep(Duration::from_secs(5));
    // `loading_animation` will run out of scope now and get dropped,
    // thus the animation will stop and remove itself from the console
}
