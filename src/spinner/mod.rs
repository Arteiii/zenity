//! Module for multiline spinners
//!
//! This module provides functionality for creating and managing multiline spinners,
//! which consist of multiple spinners running simultaneously, each with its own text
//!
//! ```
//! use zenity::spinner::MultiSpinner;
//!
//! // create a LoadingAnimation instance using one of the predefined animations
//! let spinner = MultiSpinner::default();
//!
//! // optional:
//! spinner.set_text(&spinner.get_last(), "Loading...".to_string()); // sets the text to "Loading..."
//!
//! // here you might have the time intensive task
//!
//! // `loading_animation` will run out of scope now and get dropped,
//! // thus the animation will stop and remove itself from the console
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub use frames::*;

use crate::iterators::balanced_iterator;
use crate::style;
use crate::style::{Attribute, Color, ContentStyle, StyledString};
use crate::terminal::{console_cursor, console_render};

pub mod frames;

/// struct holding multiple spinners
///
///
/// # Single Spinner Example
///
/// ```
/// use std::thread::sleep;
/// use std::time::Duration;
/// use zenity::spinner::{Frames, MultiSpinner};
///
/// let spinner = MultiSpinner::new(Frames::dot_spinner11());
/// spinner.run_all();
///
/// sleep(Duration::from_secs(4));
/// // do work here...
///
/// // get last created uid
/// spinner.set_text(&spinner.get_last(), "spinner1".to_string());
///
/// // no need to stop the spinners they will run out of scope and get dropped
/// ```
#[derive(Clone)]
pub struct MultiSpinner {
    spinner: Arc<Mutex<HashMap<usize, Frames>>>,
    show_line_number: Arc<Mutex<bool>>,
    stop: Arc<Mutex<bool>>,
}

impl Default for MultiSpinner {
    /// creates a new Progress instance
    ///
    /// ## Example
    /// ```
    /// use zenity::spinner::MultiSpinner;
    /// let spinner = MultiSpinner::default();
    /// ```
    fn default() -> Self {
        let spinner = Self::new();
        spinner.add(Frames::default());
        spinner.run_all();

        spinner
    }
}
/// ```
/// use std::thread::sleep;
/// use std::time::Duration;
/// use zenity::spinner::{Frames, MultiSpinner};
///
/// let spinner = MultiSpinner::default();
/// let spinner1 = spinner.get_last();
/// let spinner2 = spinner.add(Frames::default()); // this already returns the uid
///
/// spinner.run_all();
/// sleep(Duration::from_secs(4));
///
/// spinner.set_text(&spinner1, "spinner1".to_string());
///
/// spinner.stop(&spinner1); // stop the spinner1
///
/// spinner.set_text(&spinner2, "spinner2 :3".to_string());
///
/// // no need to stop spinner2 #
/// ```
impl MultiSpinner {
    /// creates a new MultiSpinner instance
    ///
    /// ## Example
    /// ```
    /// use zenity::spinner::MultiSpinner;
    /// let spinner = MultiSpinner::new();
    /// ```
    pub fn new() -> Self {
        MultiSpinner {
            spinner: Arc::new(Mutex::new(HashMap::new())),
            stop: Arc::new(Mutex::new(false)),
            show_line_number: Arc::new(Mutex::new(false)),
        }
    }

    /// create a new spinner
    ///
    /// # Returns
    ///
    /// unique identifier
    ///
    /// ## Example
    /// ```
    /// use zenity::spinner::MultiSpinner;
    /// use zenity::spinner::Frames;
    ///
    /// let spinner = MultiSpinner::new();
    ///
    /// spinner.add(Frames::aesthetic_load());
    /// ```
    pub fn add(&self, frames: Frames) -> usize {
        let mut spinner_map = self.spinner.lock().unwrap();
        let uid = spinner_map.len() + 1;

        spinner_map.insert(uid, frames);

        uid
    }

    /// get the last create uid
    ///
    /// # Returns
    ///
    /// unique identifier of the last created spinner
    ///
    /// ## Example
    /// ```
    /// use zenity::spinner::MultiSpinner;
    /// use zenity::spinner::Frames;
    ///
    /// let spinner = MultiSpinner::new();
    /// spinner.add(Frames::default());
    ///
    /// // the return values is an id you will need to edit the spinner later on
    /// let spinner1_uid = spinner.get_last();
    /// ```
    pub fn get_last(&self) -> usize {
        let spinner_map = self.spinner.lock().unwrap();

        // get the maximum key value (uid) from the spinner map
        spinner_map.keys().copied().max().unwrap()
    }

    /// set text of a specific spinner
    ///
    /// if the uid is invalid, this does nothing
    ///
    /// ## Example
    /// ```
    /// use zenity::spinner::MultiSpinner;
    /// use zenity::spinner::Frames;
    /// use zenity::style::StyledString;
    ///
    /// let spinner = MultiSpinner::new();
    ///
    /// spinner.set_text(&spinner.get_last(),"example".to_string());
    /// ```
    pub fn set_text(&self, uid: &usize, new_text: String) {
        if let Some(spinner) = self.spinner.lock().unwrap().get_mut(uid) {
            spinner.text = StyledString::new(&new_text);
        }
    }

    /// set a styled text of a specific spinner
    ///
    /// if the uid is invalid, this does nothing
    ///
    /// ## Example
    /// ```
    /// use crossterm::style::Color;
    /// use zenity::spinner::MultiSpinner;
    /// use zenity::spinner::Frames;
    /// use zenity::style::StyledString;
    ///
    /// let spinner = MultiSpinner::new();
    ///
    /// spinner.set_styled_text(&spinner.get_last(),
    ///     StyledString::simple("test string", Some(Color::Red), Some(Color::Black), None));
    /// ```
    pub fn set_styled_text(&self, uid: &usize, new_text: StyledString) {
        if let Some(spinner) = self.spinner.lock().unwrap().get_mut(uid) {
            spinner.text = new_text;
        }
    }

    /// stops a spinner if the uid is invalid this does nothing
    ///
    /// ## Example
    ///
    /// ```
    /// use zenity::spinner::MultiSpinner;
    /// use zenity::spinner::Frames;
    ///
    /// let spinner = MultiSpinner::new(Frames::default());
    ///
    /// spinner.stop(&spinner.get_last());
    /// ```
    pub fn stop(&self, uid: &usize) {
        if let Some(spinner) = self.spinner.lock().unwrap().get_mut(uid) {
            spinner.stop = true; // Set spinner.stop to true
        }
    }

    /// shows the line number of the running spinners
    ///
    /// [1/4]  .¸¸¸¸¸¸¸¸
    ///
    /// ## Example
    ///
    /// ```
    /// use zenity::spinner::MultiSpinner;
    /// use zenity::spinner::Frames;
    ///
    /// let spinner = MultiSpinner::new(Frames::default());
    ///
    /// spinner.show_line_number();
    /// ```
    pub fn show_line_number(&self) {
        *self.show_line_number.lock().unwrap() = true;
    }

    /// execute all created spinners
    /// ## Example
    ///
    /// ```
    /// use zenity::spinner::MultiSpinner;
    /// use zenity::spinner::Frames;
    ///
    /// // make spinner mutable
    /// let mut spinner = MultiSpinner::new(Frames::dots_simple_big1());
    ///
    /// // queue spinners for execution
    /// let spinner_num1 = spinner.get_last();
    /// let spinner_num2 = spinner.add(Frames::dots_simple_big1());
    ///
    /// //start the spinners
    /// spinner.run_all();
    /// ```
    pub fn run_all(&self) {
        let spinners = Arc::clone(&self.spinner);
        let stop = Arc::clone(&self.stop);
        let show_line_number = Arc::clone(&self.show_line_number);

        thread::spawn(move || {
            console_cursor::save_hide_cursor();

            let mut index = 1_u16;
            let mut max_line_number: usize = 0;

            while !*stop.lock().unwrap() {
                // collect frames and texts from all spinners
                for (line_number, spinner) in spinners.lock().unwrap().iter() {
                    let mut combined_vec = Vec::new();

                    if *show_line_number.lock().unwrap() {
                        // update the max_line_number if a larger line number is encountered
                        if *line_number > max_line_number {
                            max_line_number = *line_number;
                        }

                        console_render::push_styled_string!(
                            combined_vec,
                            format!("[{}/{}]", &line_number, &max_line_number),
                            Some(Color::Grey),
                            None,
                            None,
                            style::combine_attributes(&[&Attribute::Italic])
                        );

                        // to prevent style to apply to the spacing
                        console_render::push_unstyled_spaces!(combined_vec, 1);
                    }

                    // if the spinner is not stopped, include new frames and update text
                    if !spinner.stop {
                        let frames = vec![spinner.frames.clone()];

                        let current_frame = balanced_iterator(index as usize, &frames)
                            .iter()
                            .map(|opt| opt.cloned().unwrap_or_default())
                            .collect::<Vec<_>>();

                        if let Some(first_frame) = current_frame.first() {
                            combined_vec.push(first_frame.clone());
                        }
                        console_render::push_unstyled_spaces!(combined_vec, 1);
                    }

                    // always include spinner text
                    combined_vec.push(spinner.text.clone());

                    console_render::render_styled_line(*line_number as u16, &combined_vec);
                }

                index += 1;

                thread::sleep(Duration::from_millis(80));
            }
        });
    }
}

impl Drop for MultiSpinner {
    /// stops the loading animation thread when the `LoadingAnimation` object is dropped
    fn drop(&mut self) {
        *self.stop.lock().unwrap() = true;

        console_render::cleanup();
        console_cursor::reset_cursor();
    }
}
