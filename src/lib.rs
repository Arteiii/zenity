#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! # Yet Another Spinner Lib
//!
//!
//! ## How to Use?
//!
//! It's as easy as pie (or maybe even easier, depending on your pie-making skills)! Just follow these simple steps:
//!
//! ```rust
//! // example/basic.rs
//! use std::thread::sleep;
//! use std::time::Duration;
//! use zenity::{style::Color, LoadingAnimation};
//!
//! fn main() {
//!     println!("println test");
//!     scope_example();
//! }
//!
//! fn scope_example() {
//!     // create a LoadingAnimation instance using one of the predefined animations
//!     let spinner = LoadingAnimation::default();
//!
//!     // optional:
//!     spinner.set_text("Loading..."); // sets the text to "Loading..."
//!     spinner.set_text_color(Color::DarkBlue); // set the color
//!
//!     sleep(Duration::from_secs(5));
//!     // `loading_animation` will run out of scope now and get dropped,
//!     // thus the animation will stop and remove itself from the console
//! }
//! ```
//!
//! # Color Configuration
//!
//! To configure the color output, you can use the `--color` option with one of the following values:
//!
//! - `always`: Enable color output regardless of the terminal type and capabilities
//! - `auto`: Automatically determine whether to enable color output based on the terminal type and capabilities. If stdout is a pipe or if the terminal doesn't support colors, colors will be disabled
//! - `never`: Disable color output
//!
//! **Note**: If the stdout is a pipe or if the terminal doesn't support colors, colors will be automatically disabled
//!
//! Check out the examples for more.

use std::sync::{Arc, Mutex};
use std::thread;

pub use crossterm::style;

pub use animations::spinner as multi_spinner;

use crate::animations::animation::Spinner;
pub use crate::animations::frames::spinner;
pub use crate::helper::colors::{combine_attributes, CliColorConfig};

pub mod animations;
pub(crate) mod helper;
pub(crate) mod terminal;

/// `LoadingAnimation` is a struct that provides a straightforward interface for creating and managing customizable loading animations.
///
/// # Examples
///
/// ```
/// use zenity::{spinner::PreDefined, LoadingAnimation};
///
/// // Create a loading animation instance using one of the predefined animations
/// let spinner = LoadingAnimation::new(PreDefined::dot_spinner1(false));
/// spinner.set_text("Loading..."); // Sets the text to "Loading..."
/// ```
///
/// The `LoadingAnimation` struct offers various methods for customizing the loading animation:
///
/// - `new(frames: spinner::Frames) -> Self`: Creates a new `LoadingAnimation` instance with the specified frames for the animation
/// - `set_text(text: &str)`: Sets the text content for the loading animation
/// - `finish()`: Stops the loading animation
/// - `set_animation_color(color: style::Color)`: Sets the color of the animation
/// - `set_text_color(color: style::Color)`: Sets the color of the text
/// - `set_animation_style(color: style::ContentStyle)`: Sets the style for the animation
/// - `set_text_style(color: style::ContentStyle)`: Sets the style for the text
///
/// # Notes
///
/// The `LoadingAnimation` struct automatically stops the loading animation when it goes out of scope. However, you can explicitly call the `finish()` method to stop the animation at any time
///
/// # Thread Safety
///
/// `LoadingAnimation` is thread-safe and can be safely shared across multiple threads. It uses `Arc` and `Mutex` internally to ensure safe concurrent access to its fields
///
/// # Error Handling
///
/// Error handling is minimal in this struct. However, methods that may encounter errors (e.g., `set_text()`) return a `Result` type to handle potential errors gracefully
///
/// # Performance Considerations
///
/// While `LoadingAnimation` strives for efficiency, creating complex animations or frequently updating the animation's appearance may impact performance. Consider optimizing your usage based on performance requirements
pub struct LoadingAnimation {
    should_stop: Arc<Mutex<bool>>,
    handle: Option<thread::JoinHandle<()>>,
    text: Arc<Mutex<Option<String>>>,
    end_sequence: Arc<Mutex<Option<String>>>,
    animation_style: Arc<Mutex<style::ContentStyle>>,
    text_style: Arc<Mutex<style::ContentStyle>>,
    cleanup_on_exit: Arc<Mutex<bool>>,
    color_support: Arc<Mutex<CliColorConfig>>,
}

impl Default for LoadingAnimation {
    fn default() -> Self {
        let animation_style = Arc::new(Mutex::new(style::ContentStyle {
            foreground_color: Some(style::Color::White),
            background_color: None,
            underline_color: None,
            attributes: style::Attributes::default(),
        }));

        let text_style = Arc::new(Mutex::new(style::ContentStyle {
            foreground_color: Some(style::Color::White),
            background_color: None,
            underline_color: None,
            attributes: style::Attributes::default(),
        }));

        Self::with_colors(
            spinner::PreDefined::dot_spinner1(false),
            animation_style,
            text_style,
            CliColorConfig::default(),
        )
    }
}

impl LoadingAnimation {
    /// creates a new `LoadingAnimation` instance and starts the loading animation
    ///
    /// # Arguments
    ///
    /// * `frames` - The frames to be used for the loading animation
    /// (you can find predefined ones in PreDefined::)
    ///
    /// # Returns
    ///
    /// a new `LoadingAnimation` instance
    pub fn new(frames: spinner::Frames) -> Self {
        let default_animation = Self::default();
        Self::with_colors(
            frames,
            Arc::clone(&default_animation.animation_style),
            Arc::clone(&default_animation.text_style),
            CliColorConfig::default(),
        )
    }

    /// creates a new `LoadingAnimation` instance with specified colors and starts the loading animation
    ///
    /// # Arguments
    ///
    /// * `frames` - the frames to be used for the loading animation
    /// * `animation_color` - the color of the animation
    /// * `text_color` - the color of the text
    ///
    /// # Returns
    ///
    /// a new `LoadingAnimation` instance with specified colors
    pub fn with_colors(
        frames: spinner::Frames,
        animation_style_mutex: Arc<Mutex<style::ContentStyle>>,
        text_style_mutex: Arc<Mutex<style::ContentStyle>>,
        color_support: CliColorConfig,
    ) -> Self {
        let frames = Arc::new(Mutex::new(frames));
        let cleanup_on_exit = Arc::new(Mutex::new(true));
        let should_stop = Arc::new(Mutex::new(false));
        let text = Arc::new(Mutex::new(None));
        let end_sequence = Arc::new(Mutex::new(None));
        let color_support = Arc::new(Mutex::new(color_support));

        let should_stop_clone = Arc::clone(&should_stop);
        let text_clone = Arc::clone(&text);
        let end_sequence_clone = Arc::clone(&end_sequence);
        let animation_style_clone = Arc::clone(&animation_style_mutex);
        let text_style_clone = Arc::clone(&text_style_mutex);
        let cleanup_on_exit_clone = Arc::clone(&cleanup_on_exit);

        // TODO: implement
        // let color_support_clone = Arc::clone(&color_support);

        let spinner = Spinner::new(
            frames,
            should_stop_clone,
            text_clone,
            animation_style_clone,
            text_style_clone,
            end_sequence_clone,
            cleanup_on_exit_clone,
        );

        let handle = thread::spawn(move || spinner.run());

        Self {
            should_stop,
            handle: Some(handle),
            text,
            animation_style: animation_style_mutex,
            text_style: text_style_mutex,
            end_sequence,
            cleanup_on_exit,
            color_support,
        }
    }

    /// sets the text content for the loading animation
    ///
    /// this function updates the text content displayed alongside the loading animation
    ///
    /// # Arguments
    ///
    /// * `text` - a string slice (`&str`) representing the new text content to be displayed
    ///
    /// # Example
    ///
    /// ```
    /// use zenity::LoadingAnimation;
    /// let spinner = LoadingAnimation::default();
    /// // update the text content of the spinner animation.
    /// spinner.set_text("Loading..."); // ets the text to "Loading..."
    /// ```
    pub fn set_text(&self, text: &str) {
        let mut guard = self.text.lock().unwrap();
        *guard = Some(text.to_string()); // update the text value
    }

    /// sets the color of the animation
    ///
    /// # Arguments
    ///
    /// * `color` - the color to set for the animation
    ///
    /// # Example
    ///
    /// ```
    /// use zenity::{LoadingAnimation, style::Color};
    /// let spinner = LoadingAnimation::default();
    ///
    /// spinner.set_animation_color(Color::Red); // sets the animation color to Red
    /// ```
    pub fn set_animation_color(&self, color: style::Color) {
        if !self.color_support.lock().unwrap().should_enable_color() {
            *self.animation_style.lock().unwrap() = style::ContentStyle {
                foreground_color: Some(color),
                background_color: None,
                underline_color: None,
                attributes: style::Attributes::default(),
            };
        }
    }

    /// sets the color of the text
    ///
    /// # Arguments
    ///
    /// * `color` - the color to set for the text
    /// # Example
    ///
    /// ```
    /// use zenity::{LoadingAnimation, style::Color};
    /// let spinner = LoadingAnimation::default();
    ///
    /// spinner.set_text_color(Color::Red); // sets the text color to Red
    /// ```
    pub fn set_text_color(&self, color: style::Color) {
        if !self.color_support.lock().unwrap().should_enable_color() {
            *self.text_style.lock().unwrap() = style::ContentStyle {
                foreground_color: Some(color),
                background_color: None,
                underline_color: None,
                attributes: style::Attributes::default(),
            };
        }
    }

    /// sets the style of the animation
    ///
    /// not all styles/color can be applied to all animations
    ///
    /// # Example
    ///
    /// ```
    /// use zenity::{LoadingAnimation, style::{Attribute, ContentStyle, Color}, combine_attributes};
    /// let spinner = LoadingAnimation::default();
    ///
    /// let style = ContentStyle {
    ///     background_color: None,
    ///     underline_color: Some(Color::Red),
    ///     foreground_color: Some(Color::Rgb {r: 255, g:255, b:255}),
    ///
    ///     // combine attributes is a helper function to combine multiple style elements and apply them
    ///     attributes: combine_attributes(&[
    ///         &Attribute::Bold,
    ///         &Attribute::Underlined,
    ///     ]),
    /// };
    /// spinner.set_animation_style(style); // sets the aniamtion style
    /// ```
    pub fn set_animation_style(&self, color: style::ContentStyle) {
        if !self.color_support.lock().unwrap().should_enable_color() {
            *self.animation_style.lock().unwrap() = color;
        }
    }

    /// sets the style of the text
    ///
    /// # Example
    ///
    /// ```
    /// use zenity::{LoadingAnimation, style::{Attribute, ContentStyle, Color}, combine_attributes};
    /// let spinner = LoadingAnimation::default();
    ///
    /// let style = ContentStyle {
    ///     background_color: None,
    ///     underline_color: Some(Color::Red),
    ///     foreground_color: Some(Color::Rgb {r: 255, g:255, b:255}),
    ///
    ///     // combine attributes is a helper function to combine multiple style elements and apply them
    ///     attributes: combine_attributes(&[
    ///         &Attribute::Bold,
    ///         &Attribute::Underlined,
    ///     ]),
    /// };
    /// spinner.set_text_style(style); // sets the text style
    /// ```
    pub fn set_text_style(&self, color: style::ContentStyle) {
        if !self.color_support.lock().unwrap().should_enable_color() {
            *self.text_style.lock().unwrap() = color;
        }
    }

    /// stop and delete the animation from the console
    pub fn stop_and_discard(&mut self) {
        self.finish();
    }

    /// stops the loading animation without clearing it from the console
    ///
    /// # Arguments
    ///
    /// * `spinner` - optional string slice (`&str`) representing the new spinner frame
    /// * `text` - optional string slice (`&str`) representing the new text content to be displayed
    /// * `text_color` - optional `ContentStyle` representing the color of the text
    ///
    /// if `spinner` is provided, it will replace the last spinner frame
    /// if `text` is provided, it will replace the text field
    /// if `text_color` is provided, it will set the color of the text
    ///
    /// the loading animation will be stopped after this method is called, but it won't be cleared from the console
    pub fn stop_and_persist(
        &mut self,
        spinner: Option<&str>,
        text: Option<&str>,
        text_color: Option<style::ContentStyle>,
    ) {
        // set cleanup_on_exit to false
        *self.cleanup_on_exit.lock().unwrap() = false;

        if let Some(spinner_seq) = spinner {
            *self.end_sequence.lock().unwrap() = Some(spinner_seq.to_string());
        }

        if let Some(text_seq) = text {
            let mut guard = self.text.lock().unwrap();
            *guard = Some(text_seq.to_string());
        }

        if let Some(color) = text_color {
            if !self.color_support.lock().unwrap().should_enable_color() {
                *self.text_style.lock().unwrap() = color;
            }
        }

        // stop the animation
        self.finish();
    }

    /// stops the loading animation
    ///
    /// this method sets the flag to stop the animation and waits for the animation thread to join
    fn finish(&mut self) {
        *self.should_stop.lock().unwrap() = true;
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl Drop for LoadingAnimation {
    /// stops the loading animation thread when the `LoadingAnimation` object is dropped
    fn drop(&mut self) {
        self.finish();
    }
}
