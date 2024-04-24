// example/basic.rs

use std::thread::sleep;
use std::time::Duration;
use zenity::spinner::MultiSpinner;

fn main() {
    // create a LoadingAnimation instance using one of the predefined animations
    let spinner = MultiSpinner::default();
    sleep(Duration::from_secs(5));

    // optional:
    spinner.set_text(&spinner.get_last(), "Loading...".to_string()); // sets the text to "Loading..."

    sleep(Duration::from_secs(500));
    // `loading_animation` will run out of scope now and get dropped,
    // thus the animation will stop and remove itself from the console
}
