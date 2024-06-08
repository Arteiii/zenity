use std::thread::sleep;
use std::time::Duration;

use zenity::spinner::{Frames, MultiSpinner};
use zenity::style::StyledString;
use zenity::styled_string;

fn main() {
    // custom animations
    let custom_frames: Frames = Frames {
        frames: styled_string!["⚫", "⚪", "⚫", "⚪"], // custom frames for animation
        speed_ms: 150, // custom speed for animation in milliseconds
        text: StyledString::default(),
        stop: false,
    };

    // create a MultiSpinner instance using the new custom animation
    let spinner = MultiSpinner::new();
    spinner.add(custom_frames);
    spinner.run_all();

    // wait for 5 seconds to showcase the loading animation with the custom animation
    sleep(Duration::from_secs(5));
    // end!
}
