//! mod for progress bars

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use rand::Rng;

use crate::animations::frames::progress::ProgressBarFrames;
use crate::terminal::{console_cursor, console_render};

/// bar struct encapsulating the loading bar data animation
pub struct Bar {
    /// frames to use for animation
    pub frames: Arc<Mutex<ProgressBarFrames>>,

    /// size of progress bar
    pub size: Arc<Mutex<usize>>,

    /// goal value
    pub goal: Arc<Mutex<usize>>,

    /// current value
    pub current: Arc<Mutex<usize>>,
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            frames: Arc::new(Mutex::new(ProgressBarFrames::equal())),
            size: Arc::new(Mutex::new(31)),
            goal: Arc::new(Mutex::new(253)),
            current: Arc::new(Mutex::new(0)),
        }
    }
}

/// struct holding multiple bars
pub struct Progress {
    // TODO: instead of random ids go after creation and increment by one
    // this would allow to render them line for line based on this and order them correctly
    bar: Arc<Mutex<HashMap<usize, Bar>>>,
    stop: Arc<Mutex<bool>>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    /// creates a new Progress instance
    ///
    /// ## Example
    /// ```
    /// # use zenity::multi_spinner::MultiSpinner;
    /// let _spinner = MultiSpinner::new();
    /// ```
    pub fn new() -> Self {
        // console_cursor::reset_cursor();

        console_cursor::save_hide_cursor();

        Progress {
            bar: Arc::new(Mutex::new(HashMap::new())),
            stop: Arc::new(Mutex::new(false)),
        }
    }

    /// add a new progress bar
    pub fn add(&self, bar: Bar) -> usize {
        let mut rng = rand::thread_rng();
        let mut uid: usize;

        loop {
            uid = rng.gen();
            if !self.bar.lock().unwrap().contains_key(&uid) {
                break;
            }
        }

        self.bar.lock().unwrap().insert(uid, bar);

        uid
    }

    /// set the current
    ///
    /// # Arguments
    ///
    /// * `uid` - the unique identifier of the progress bar
    /// * `new_current` - the new value to set as the current progress
    ///
    /// **NOTE:**
    /// - if the UID is invalid, this function does nothing
    /// - this function locks the progress bar associated with the provided uid and updates its current value
    pub fn set(&self, uid: &usize, new_current: &usize) {
        if let Some(bar) = self.bar.lock().unwrap().get(uid) {
            let mut current = bar.current.lock().unwrap();
            *current = *new_current;
        }
    }

    /// start each queued progressbar
    pub fn run_all(&mut self) {
        let bars = Arc::clone(&self.bar);
        let stop = Arc::clone(&self.stop);

        thread::spawn(move || {
            while !*stop.lock().unwrap() {
                let mut rendered_frames = Vec::new();

                for (_, spinner) in bars.lock().unwrap().iter() {
                    let frames = spinner.frames.lock().unwrap();
                    let begin: &str = frames.begin[0];
                    let end: &str = frames.end[0];
                    let current_incomplete: &str = frames.bar_incomplete_char[0];
                    let current_complete: &str = frames.bar_complete_char[0];

                    let size: usize = *spinner.size.lock().unwrap();
                    let goal = *spinner.goal.lock().unwrap();
                    let current: usize = *spinner.current.lock().unwrap();

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
}

impl Drop for Progress {
    /// stops the thread when the object is dropped
    fn drop(&mut self) {
        // cleanup methods
        console_render::cleanup();
        console_cursor::reset_cursor();
    }
}
