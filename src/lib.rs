//! # Yet Another Spinner Lib
//!
//!
//! ## How to Use?
//!
//! It's as easy as pie (or maybe even easier, depending on your pie-making skills)! Just follow these simple steps:
//!
//! ```rust
//! use animations_rs::{spinner::PreDefined, LoadingAnimation};
//!
//! fn scope_example() {
//!     // create a LoadingAnimation instance using one of the predefined animations
//!     let spinner = LoadingAnimation::new(PreDefined::dot_spinner1(false)); // invert frames bool (false)
//!
//!     spinner.set_text("Loading..."); // sets the text to "Loading..."
//!
//!     // `loading_animation` will run out of scope now and get dropped,
//!     // thus the animation will stop and remove itself from the console
//! }
//! ```
//!
//! check out the examples for more
//!

use std::sync::{Arc, Mutex};
use std::thread;

use crate::animations::animation;
pub use crate::animations::frames::spinner;

mod animations;

/// a simple loading animation that can be easily used with basic configurations
///
/// this struct provides a straightforward interface for creating and managing a loading animation
/// it is suitable for common use cases where a basic loading animation is sufficient
pub struct LoadingAnimation {
    should_stop: Arc<Mutex<bool>>,
    handle: Option<thread::JoinHandle<()>>,
    text: Arc<Mutex<Option<String>>>,
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
        let should_stop = Arc::new(Mutex::new(false));
        let text = Arc::new(Mutex::new(None));
        let should_stop_clone = Arc::clone(&should_stop);
        let text_clone = Arc::clone(&text);
        let handle = thread::spawn(move || {
            animation::spinner_animation(&frames, should_stop_clone, text_clone);
        });
        Self {
            should_stop,
            handle: Some(handle),
            text,
        }
    }

    /// sets the text content for the loading animation
    ///
    /// this function updates the text content displayed alongside the loading animation
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice (`&str`) representing the new text content to be displayed.
    ///
    /// # Example
    ///
    /// ```
    /// use animations_rs::{spinner::PreDefined, LoadingAnimation};
    /// let spinner = LoadingAnimation::new(PreDefined::dot_spinner1(false));
    /// // update the text content of the spinner animation
    /// spinner.set_text("Loading..."); // sets the text to "Loading..."
    /// ```
    pub fn set_text(&self, text: &str) {
        let mut guard = self.text.lock().unwrap();
        *guard = Some(text.to_string()); // update the text value
    }

    /// stops the loading animation
    ///
    /// this method sets the flag to stop the animation and waits for the animation thread to join
    pub fn finish(&mut self) {
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
