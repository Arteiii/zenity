use std::thread::sleep;
use std::time::Duration;
use zenity::{
    spinner::{Frames, PreDefined},
    Color, LoadingAnimation,
};

fn main() {
    scope_example();

    {
        // custom animations
        let custom_frames: Frames = Frames {
            frames: vec!["⚫", "⚪", "⚫", "⚪"], // custom frames for animation
            speed_ms: 150,                        // custom speed for animation in milliseconds
        };

        // create a LoadingAnimation instance using the new custom animation
        let _custom_loading_animation = LoadingAnimation::new(custom_frames);

        // wait for 5 seconds to showcase the loading animation with the custom animation
        sleep(Duration::from_secs(5));
        // end!
    }

    let mut spinner = LoadingAnimation::new(PreDefined::dot_spinner3(false));

    spinner.set_text("Loading..."); // sets the text to "Loading..."

    sleep(Duration::from_secs(5));

    // change colors during the animation
    change_colors_during_animation(&mut spinner, Duration::from_secs(2), Duration::from_secs(2));

    sleep(Duration::from_secs(5));
}

fn change_colors_during_animation(
    spinner: &mut LoadingAnimation,
    animation_color_duration: Duration,
    text_color_duration: Duration,
) {
    spinner.set_text("MOREEEEEEE Loading... (but with color)"); // overwrite current text

    // default color Red
    spinner.set_animation_color(Color::Red);
    sleep(text_color_duration);

    //  default color Blue
    spinner.set_text_color(Color::Blue);
    sleep(animation_color_duration);

    // custom RGB color (50, 60, 70)
    spinner.set_animation_color(Color::Rgb {
        r: 50,
        g: 60,
        b: 70,
    });
    sleep(text_color_duration);

    // custom RGB color (100, 200, 255)
    spinner.set_text_color(Color::Rgb {
        r: 100,
        g: 200,
        b: 255,
    });
    sleep(text_color_duration);

    // for more information, refer to the Crossterm documentation (https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html)
}

fn scope_example() {
    // create a LoadingAnimation instance using one of the predefined animations
    let _loading_animation = LoadingAnimation::new(PreDefined::dot_spinner1(false));

    // wait for 5 seconds to showcase the loading animation
    sleep(Duration::from_secs(5));

    // `loading_animation` will run out of scope now and get dropped,
    // thus the animation will stop and remove itself from the console
    // There might be issues with cleanup if there is any output during the animation
    // (will try to fix this issue, tho I don't know how yet)
}
