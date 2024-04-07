use std::thread::sleep;
use std::time::Duration;
use unicode_icons::icons::symbols;
use zenity::{
    combine_attributes,
    spinner::{Frames, PreDefined},
    style::{Attribute, Color, ContentStyle},
    LoadingAnimation,
};

fn main() {
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
    change_colors_during_animation(&mut spinner, Duration::from_secs(2));

    let content_style = ContentStyle {
        foreground_color: Some(Color::Green),
        background_color: None,
        underline_color: Some(Color::Green),
        attributes: Attribute::Bold.into(),
    };

    spinner.stop_and_persist(
        Some(&symbols::check_mark_button().to_string()),
        Some("Successfully"),
        Some(content_style),
    );

    sleep(Duration::from_secs(5));
}

fn change_colors_during_animation(spinner: &mut LoadingAnimation, text_color_duration: Duration) {
    spinner.set_text("MOREEEEEEE Loading... (but with color)"); // overwrite current text

    // custom rgb color (50, 60, 70)
    spinner.set_text_color(Color::Rgb { r: 255, g: 0, b: 0 });
    sleep(text_color_duration);

    spinner.set_text("Styled Text and loading");

    // custom style
    let content_style = ContentStyle {
        foreground_color: Some(Color::Yellow),
        background_color: Some(Color::Blue),
        underline_color: Some(Color::Green),
        attributes: combine_attributes(&[
            &Attribute::Bold,
            &Attribute::Underlined,
            &Attribute::Italic, // add another attribute here
        ]),
    };

    spinner.set_animation_style(content_style);
    sleep(text_color_duration);

    // for more information, refer to the cross-term documentation (https://docs.rs/crossterm/latest/crossterm/style)
}
