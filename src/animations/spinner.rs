//! mod for multiline spinners

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rand::Rng;

use crate::helper::iterators;
use crate::spinner::Frames;
use crate::terminal::{console_cursor, console_render};

/// spinner struct encapsulating the spinner animation
pub struct Spinner {
    frames: Arc<Mutex<Frames>>,
    text: Arc<Mutex<String>>,
    should_stop: Arc<Mutex<bool>>,
}

/// struct holding multiple spinners
pub struct MultiSpinner {
    spinner: Arc<Mutex<HashMap<usize, Spinner>>>,
    stop: Arc<Mutex<bool>>,
    // index: usize,
}

impl Default for MultiSpinner {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiSpinner {
    /// creates a new MultiSpinner instance
    ///
    /// ## Example
    /// ```
    /// # use zenity::multi_spinner::MultiSpinner;
    /// let _spinner = MultiSpinner::new();
    /// ```
    pub fn new() -> Self {
        MultiSpinner {
            spinner: Arc::new(Mutex::new(HashMap::new())),
            stop: Arc::new(Mutex::new(false)),
            // index: 1_usize,
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
    /// use zenity::multi_spinner::MultiSpinner;
    /// use zenity::spinner::PreDefined;
    ///
    /// let spinner = MultiSpinner::new();
    ///
    /// // the return values is an id you will need to edit the spinner later on
    /// let spinner_num1 = spinner.new(PreDefined::dots_simple_big1(false));
    /// ```
    pub fn add(&self, frames: Frames) -> usize {
        let mut rng = rand::thread_rng();
        let mut uid: usize;

        let new_spinner = Spinner {
            frames: Arc::new(Mutex::new(frames)),
            text: Arc::new(Mutex::new("".to_string())),
            should_stop: Arc::new(Mutex::new(false)),
        };

        loop {
            uid = rng.gen();
            if !self.spinner.lock().unwrap().contains_key(&uid) {
                break;
            }
        }

        self.spinner.lock().unwrap().insert(uid, new_spinner);
        uid
    }

    /// set text of a specific spinner
    ///
    /// if the uid is invalid this does nothing
    ///
    /// ## Example
    /// ```
    /// use zenity::multi_spinner::MultiSpinner;
    /// use zenity::spinner::PreDefined;
    ///
    /// let spinner = MultiSpinner::new();
    /// let spinner_num1 = spinner.new(PreDefined::dots_simple_big1(false));
    ///
    /// spinner.set_text(spinner_num1, "this is a text...".to_string());
    /// ```
    pub fn set_text(&self, uid: &usize, new_text: String) {
        if let Some(spinner) = self.spinner.lock().unwrap().get(uid) {
            let mut text = spinner.text.lock().unwrap();
            *text = new_text;
        }
    }

    /// stops a spinner if the uid is invalid this does nothing
    ///
    /// ## Example
    ///
    /// ```
    /// use zenity::multi_spinner::MultiSpinner;
    /// use zenity::spinner::PreDefined;
    ///
    /// let spinner = MultiSpinner::new();
    /// let spinner_num1 = spinner.new(PreDefined::dots_simple_big1(false));
    ///
    /// spinner.stop(spinner_num1);
    /// ```
    pub fn stop(&self, uid: &usize) {
        if let Some(spinner) = self.spinner.lock().unwrap().get(uid) {
            *spinner.should_stop.lock().unwrap() = true;
        }
    }

    /// execute all created spinners
    /// ## Example
    ///
    /// ```
    /// use zenity::multi_spinner::MultiSpinner;
    /// use zenity::spinner::PreDefined;
    ///
    /// // make spinner mutable
    /// let mut spinner = MultiSpinner::new();
    ///
    /// // queu spinners for execution
    /// let spinner_num1 = spinner.new(PreDefined::dots_simple_big1(false));
    /// let spinner_num2 = spinner.new(PreDefined::dots_simple_big1(false));
    ///
    /// //start the spinners
    /// spinner.run_all();
    /// ```
    pub fn run_all(&mut self) {
        let spinners = Arc::clone(&self.spinner);
        let stop = Arc::clone(&self.stop);

        thread::spawn(move || {
            console_cursor::save_hide_cursor();

            let mut index = 1_usize;

            while !*stop.lock().unwrap() {
                let mut all_frames = Vec::new();
                let mut all_texts = Vec::new();
                let mut all_should_stop = Vec::new();

                // collect frames and texts from all spinners
                for (_, spinner) in spinners.lock().unwrap().iter() {
                    let frames = spinner.frames.lock().unwrap().frames.clone();
                    let text = spinner.text.lock().unwrap().clone();
                    let should_stop = *spinner.should_stop.lock().unwrap();
                    all_should_stop.push(should_stop);
                    all_frames.push(frames);
                    all_texts.push(text);
                }

                // render frames with corresponding texts
                MultiSpinner::render_frames(&all_frames, index, &all_texts, &all_should_stop);

                index += 1;

                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    /// helper function to render frames to stdout
    fn render_frames(frames: &[Vec<&str>], index: usize, texts: &[String], should_stop: &[bool]) {
        let first_frame = iterators::balanced_iterator(index, frames);

        let combined_string: String = first_frame
            .iter()
            .zip(texts.iter())
            .zip(should_stop.iter())
            .filter_map(|((frame, text), should_stop)| {
                if *should_stop {
                    Some(text.to_string())
                } else {
                    frame.as_ref().map(|frame| format!("{}  {}", frame, text))
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        console_render::render_frame(&combined_string);
    }

    /// helper function to clean-up after animation stop
    fn cleanup(&mut self) {
        *self.stop.lock().unwrap() = true;

        console_render::cleanup();
        console_cursor::reset_cursor();
    }
}

impl Drop for MultiSpinner {
    /// stops the loading animation thread when the `LoadingAnimation` object is dropped
    fn drop(&mut self) {
        self.cleanup();
    }
}
