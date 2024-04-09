//! mod for progress bars
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub use frames::*;

use crate::terminal::{console_cursor, console_render};

pub mod frames;


/// struct holding multiple bars
#[derive(Clone)]
pub struct ProgressBar {
    // TODO: instead of random ids go after creation and increment by one
    // this would allow to render them line for line based on this and order them correctly
    bar: Arc<Mutex<HashMap<usize, Frames>>>,
    stop: Arc<Mutex<bool>>,
}

impl Default for ProgressBar {
    fn default() -> Self {
        let progress = Self::new(Frames::default());

        progress.run_all();

        progress
    }
}

impl ProgressBar {
    /// creates a new Progress instance
    ///
    /// ## Example
    /// ```
    /// # use zenity::progress::{Frames, ProgressBar};
    /// let _spinner = ProgressBar::new(Frames::default());
    /// ```
    pub fn new(bar: Frames) -> Self {
        // console_cursor::reset_cursor();

        console_cursor::save_hide_cursor();

        let progress = ProgressBar {
            bar: Arc::new(Mutex::new(HashMap::new())),
            stop: Arc::new(Mutex::new(false)),
        };

        progress.add(bar);

        progress
    }

    /// adds a new progress bar with an incremental UID starting from 1
    ///
    /// # Arguments
    ///
    /// * `bar` - the progress bar to add
    ///
    /// # Returns
    ///
    /// the UID assigned to the added progress bar
    pub fn add(&self, bar: Frames) -> usize {
        let mut bar_map = self.bar.lock().unwrap();
        let uid: usize = bar_map.len() + 1_usize; // Incremental UID starting from 1

        bar_map.insert(uid, bar);

        uid
    }

    /// Set the current value
    ///
    /// # Arguments
    ///
    /// * `uid` - the unique identifier of the progress bar
    /// * `new_current` - the new value to set as the current progress
    ///
    /// **NOTE:**
    /// - if the UID is invalid, this function does nothing
    /// - this function locks the progress bar associated with the provided UID and updates its current value incrementally
    pub fn set(&self, uid: &usize, new_current: &usize) {
        if let Some(bar) = self.bar.lock().unwrap().get(uid) {
            let current = bar.current.lock().unwrap();
            let diff = new_current.saturating_sub(*current);
            drop(current);
            bar.inc(&diff);
        }
    }

    /// start each queued progressbar
    pub fn run_all(&self) {
        let bars = Arc::clone(&self.bar);
        let stop = Arc::clone(&self.stop);

        thread::spawn(move || {
            while !*stop.lock().unwrap() {
                let mut rendered_frames = Vec::new();

                for (_, frames) in bars.lock().unwrap().iter() {
                    let begin: &str = frames.begin[0];
                    let end: &str = frames.end[0];
                    let current_incomplete: &str = frames.bar_incomplete_char[0];
                    let current_complete: &str = frames.bar_complete_char[0];

                    let size: usize = *frames.size.lock().unwrap();
                    let goal = *frames.goal.lock().unwrap();
                    let current: usize = *frames.current.lock().unwrap();

                    // calculate percentage completion
                    let completion_percentage = (current as f64 / goal as f64) * 100.0;

                    // calculate number of characters to represent the completion percentage
                    let complete_size = ((completion_percentage / 100.0) * size as f64) as usize;
                    let incomplete_size = size - complete_size;

                    // Render the frame with the updated incomplete string and add it to the vector
                    let rendered_frame = format!(
                        "{begin}{}{}{end}  {:.2}% | {}/{}",
                        current_complete.repeat(complete_size),
                        current_incomplete.repeat(incomplete_size),
                        completion_percentage,
                        current,
                        goal,
                    );
                    rendered_frames.push(rendered_frame);
                }

                // Join all the rendered frames from the vector
                let combined_output = rendered_frames.join("\n");

                // render the frame with the updated incomplete string
                console_render::render_frame(&combined_output);
            }
        });
    }

    /// retrieves the UID of the last created progress bar
    ///
    /// # Returns
    ///
    /// the UID of the last created progress bar
    pub fn get_last(&self) -> usize {
        let bar_map = self.bar.lock().unwrap();
        bar_map.len()
    }
}

impl Drop for ProgressBar {
    /// stops the thread when the object is dropped
    fn drop(&mut self) {
        *self.stop.lock().unwrap() = true;
        // cleanup methods
        console_cursor::reset_cursor();
        console_cursor::next_line(self.bar.lock().unwrap().len() as u16);
    }
}
