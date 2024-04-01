// example/basic.rs

use std::thread::sleep;
use std::time::Duration;
use zenity::{style::Color, LoadingAnimation};

fn main() {
    println!("println test");
    scope_example();
}

fn scope_example() {
    // create a LoadingAnimation instance using one of the predefined animations
    let spinner = LoadingAnimation::default(); // invert frames bool (false)

    spinner.set_text("Loading..."); // sets the text to "Loading..."
    spinner.set_text_color(Color::DarkBlue);

    sleep(Duration::from_secs(5));
    // `loading_animation` will run out of scope now and get dropped,
    // thus the animation will stop and remove itself from the console
}
