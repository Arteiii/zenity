//! mod for progress bars

use std::collections::HashMap;
use std::io::stdout;
use std::sync::{Arc, Mutex};

use crossterm::{cursor, execute, terminal};
use rand::Rng;

use crate::animations::frames::progress::ProgressBarFrames;

/// bar struct encapsulating the loading bar data animation
pub struct Bar {
    /// frames to use for animation
    frames: Arc<Mutex<ProgressBarFrames>>,

    /// size of progress bar
    size: u16,

    /// goal value
    goal: usize,

    /// current value
    current: usize,
}

/// struct holding multiple bars
pub struct Progress {
    bar: Arc<Mutex<HashMap<usize, Bar>>>,
}

impl Default for Progress {
    fn default() -> Self {
        let progress = Self::new();

        progress.add(Bar {
            frames: Arc::new(Mutex::new(ProgressBarFrames::rect())),
            size: 15,
            goal: 100,
            current: 0,
        });

        progress.run_all();

        progress
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
        Progress {
            bar: Arc::new(Mutex::new(HashMap::new())),
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

    /// helper function to clean-up after animation stop
    fn cleanup(&mut self) {
        execute!(
            stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Show,
        )
        .unwrap();
    }

    /// start each queued progressbar
    pub fn run_all(&self) {}
}

impl Drop for Progress {
    /// stops the loading animation thread when the `LoadingAnimation` object is dropped
    fn drop(&mut self) {
        self.cleanup();
    }
}
