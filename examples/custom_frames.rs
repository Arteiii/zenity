use std::thread::sleep;
use std::time::Duration;

use zenity::spinner::{Frames, MultiSpinner};

fn main() {
    // custom animations
    let custom_frames: Frames = Frames {
        frames: vec!["⚫", "⚪", "⚫", "⚪"], // custom frames for animation
        speed_ms: 150,                        // custom speed for animation in milliseconds
    };

    // create a MultiSpinner instance using the new custom animation
    let spinner = MultiSpinner::new(custom_frames);
    spinner.run_all();

    // wait for 5 seconds to showcase the loading animation with the custom animation
    sleep(Duration::from_secs(5));
    // end!
}
