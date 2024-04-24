//! mod for progress bars
//!
//! ```
//! use zenity::progress::{Frames, ProgressBar};
//! use std::thread;
//! use std::time::Duration;
//!
//! // create a new ProgressBar instance
//! let progress = ProgressBar::default();
//!
//! // wait for the background thread to finish
//! // in a real-world scenario, you might have other tasks to perform here
//! thread::sleep(Duration::from_secs(5));
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub use frames::*;

use crate::iterators::balanced_single;
use crate::style::StyledString;
use crate::terminal::{console_cursor, console_render};

pub mod frames;

/// struct holding multiple ProgressBars / Frames and the uid
///
/// # Example
///
/// ```
/// use zenity::progress::{Frames, ProgressBar};
/// use std::thread;
/// use std::time::Duration;
///
///  let progress = ProgressBar::new(Frames::rect().set_goal(253));
///  let progress1 = progress.get_last();
///
///  let progress2 = progress.add(Frames::equal().set_goal(253).set_size(7));
///  let progress3 = progress.add(Frames::hash().set_goal(253).set_size(60));
///
///  progress.run_all();
///
///  let loading = 1_usize;
///
///  for loading in loading..=253 {
///     progress.set(&progress1, &loading);
///     progress.set(&progress2, &loading);
///     progress.set(&progress3, &loading);
///
///     thread::sleep(Duration::from_millis(70));
///  }
///
/// ```
pub struct ProgressBar {
    bar: Arc<Mutex<HashMap<usize, Frames>>>,
    stop: Arc<Mutex<bool>>,
}

impl Default for ProgressBar {
    /// creates a new Progress instance
    ///
    /// ## Example
    /// ```
    /// use zenity::progress::ProgressBar;
    /// let spinner = ProgressBar::default();
    /// ```
    fn default() -> Self {
        let progress = Self::new(Frames::default());

        progress.run_all();

        progress
    }
}

/// ```
/// use zenity::progress::ProgressBar;
/// let spinner = ProgressBar::default();
/// ```
impl ProgressBar {
    /// creates a new Progress instance
    ///
    /// ## Example
    /// ```
    /// use zenity::progress::{Frames, ProgressBar};
    ///
    /// let spinner = ProgressBar::new(Frames::default());
    /// let uid1 = spinner.get_last(); // the last created uid
    /// # assert_eq!(uid1, 1);
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
    ///
    /// ## Example
    /// ```
    /// use zenity::progress::{Frames, ProgressBar};
    ///
    /// # let spinner = ProgressBar::new(Frames::default());
    /// # let uid1 = spinner.get_last();
    /// let uid2 = spinner.add(Frames::default());
    /// # assert_eq!(uid2, 2);
    /// ```
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
    /// * `Uid` - the unique identifier of the progress bar
    /// * `new_current` - the new value to set as the current progress
    ///
    /// ### NOTE
    ///
    /// - If the UID is invalid, this function does nothing
    /// -
    /// This function locks the progress bar associated with the provided UID
    /// and updates its current value incrementally
    ///
    /// ## Example
    /// ```
    /// use zenity::progress::{Frames, ProgressBar};
    ///
    /// let spinner = ProgressBar::default();
    /// let uid = spinner.add(Frames::default());
    ///
    /// # assert_eq!(spinner.get(&uid), Some(0));
    /// #
    /// spinner.set(&uid, &50);
    /// # assert_eq!(spinner.get(&uid), Some(50));
    /// ```
    pub fn set(&self, uid: &usize, new_current: &usize) {
        if let Some(bar) = self.bar.lock().unwrap().get_mut(uid) {
            let current = bar.current;
            let diff = new_current.saturating_sub(current);
            bar.inc(&diff);
        }
    }

    /// Get the current value of a progress bar
    ///
    /// # Arguments
    ///
    /// * `Uid` - the unique identifier of the progress bar
    ///
    /// # Returns
    ///
    /// The current value of the progressbar if it exists, otherwise `None`.
    ///
    /// ## Example
    /// ```
    /// use zenity::progress::{Frames, ProgressBar};
    /// #
    /// # let spinner = ProgressBar::default();
    /// # let uid = spinner.add(Frames::default());
    ///
    /// spinner.set(&uid, &50);
    /// # assert_eq!(spinner.get(&uid), Some(50));
    ///
    /// if let Some(current) = spinner.get(&uid) {
    ///     spinner.set(&uid, &(current + 10));
    /// }
    /// # assert_eq!(spinner.get(&uid), Some(60));
    /// ```
    pub fn get(&self, uid: &usize) -> Option<usize> {
        if let Some(bar) = self.bar.lock().unwrap().get(uid) {
            let current = bar.current;
            Some(current)
        } else {
            None
        }
    }

    /// start each queued progressbar
    ///
    /// # Returns
    ///
    /// the UID of the last created progress bar
    ///
    /// ## Example
    /// ```
    /// use zenity::progress::{Frames, ProgressBar};
    ///
    /// let spinner = ProgressBar::new(Frames::default());
    /// let uid1 = spinner.run_all(); // starts all created bars
    /// ```
    pub fn run_all(&self) {
        let bars = Arc::clone(&self.bar);
        let stop = Arc::clone(&self.stop);

        let mut last_render_time = Instant::now();

        thread::spawn(move || {
            let mut frame_index = 0_usize;
            while !*stop.lock().unwrap() {
                for (index, frames) in bars.lock().unwrap().iter() {
                    let mut line = Vec::new();

                    let size: usize = frames.size;
                    let goal = frames.goal;
                    let current: usize = frames.current;

                    // calculate percentage completion
                    let completion_percentage = (current as f64 / goal as f64) * 100.0;

                    // calculate the number of characters to represent the completion percentage
                    let complete_size = ((completion_percentage / 100.0) * size as f64) as usize;
                    let incomplete_size = size - complete_size;

                    // check if it's time to update the frame index
                    if Instant::now() - last_render_time >= Duration::from_millis(100) {
                        frame_index += 1; // go to the next frame for animated progress bars
                        last_render_time = Instant::now(); // update last render time
                    }

                    line.push(balanced_single(frame_index, &frames.begin.clone()).clone());
                    line.push(
                        balanced_single(frame_index, &frames.bar_complete_char.clone())
                            .repeat(complete_size),
                    );
                    line.push(balanced_single(frame_index, &frames.limiter.clone()).clone());
                    line.push(
                        balanced_single(frame_index, &frames.bar_incomplete_char.clone())
                            .repeat(incomplete_size),
                    );
                    line.push(balanced_single(frame_index, &frames.end.clone()).clone());
                    line.push(StyledString::new(&format!(" {:.2}", completion_percentage)));
                    line.push(StyledString::new("%"));
                    line.push(StyledString::new(" | "));
                    line.push(StyledString::new(&format!("{}", current)));
                    line.push(StyledString::new("/"));
                    line.push(StyledString::new(&format!("{}", goal)));

                    console_render::render_styled_line(*index as u16, &line);
                }
            }
        });
    }

    /// retrieves the UID of the last created progress bar
    ///
    /// # Returns
    ///
    /// the UID of the last created progress bar
    ///
    /// ## Example
    /// ```
    /// use zenity::progress::{Frames, ProgressBar};
    ///
    /// let spinner = ProgressBar::new(Frames::default());
    /// let uid1 = spinner.get_last(); // the last created uid
    /// # assert_eq!(uid1, 1);
    /// ```
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
