use animations_rs::{
    spinner::{Frames, PreDefined},
    LoadingAnimation,
};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    scope_example();

    // custom animations
    let custom_frames: Frames = Frames {
        frames: vec!["⚫", "⚪", "⚫", "⚪"], // custom frames for animation
        speed_ms: 150,                        // custom speed for animation in milliseconds
    };

    // create a LoadingAnimation instance using the new custom animation
    let _custom_loading_animation = LoadingAnimation::new(custom_frames);

    // wait for 5 seconds to showcase the loading animation with the custom animation
    sleep(Duration::from_secs(5));
}

fn scope_example() {
    // create a LoadingAnimation instance using one of the predefined animations
    let _loading_animation = LoadingAnimation::new(PreDefined::kaomoji(false));

    // wait for 5 seconds to showcase the loading animation
    sleep(Duration::from_secs(5));

    // `loading_animation` will run out of scope now and get dropped,
    // thus the animation will stop and remove itself from the console
    // There might be issues with cleanup if there is any output during the animation
    // (will try to fix this issue, tho I don't know how yet)
}
