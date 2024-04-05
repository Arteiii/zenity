use std::collections::HashMap;
use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crossterm::style::Print;
use crossterm::{cursor, execute, terminal};
use rand::Rng;
use utils_arteii_rs::vector_operations::iterators;

use crate::spinner::Frames;

/// spinner struct encapsulating the spinner animation
pub struct Spinner {
    frames: Arc<Mutex<Frames>>,
    text: Arc<Mutex<String>>,
    should_stop: Arc<Mutex<bool>>,
}

/// struct holding multiple spinners
pub struct MultiSpinner {
    spinner: Arc<Mutex<HashMap<usize, Spinner>>>,
    // index: usize,
}

impl MultiSpinner {
    pub fn new() -> Self {
        MultiSpinner {
            spinner: Arc::new(Mutex::new(HashMap::new())),
            // index: 1_usize,
        }
    }

    /// create a new spinner
    ///
    /// # Returns
    /// unique identifier
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

    pub fn set_text(&self, uid: &usize, new_text: String) {
        if let Some(spinner) = self.spinner.lock().unwrap().get(uid) {
            let mut text = spinner.text.lock().unwrap();
            *text = new_text;
        }
    }

    /// stops a spinner if the uid is invalid this does nothing
    pub fn stop(&self, uid: &usize) {
        if let Some(spinner) = self.spinner.lock().unwrap().get(&uid) {
            *spinner.should_stop.lock().unwrap() = true;
        }
    }

    pub fn run_all(&mut self) {
        let spinners = Arc::clone(&self.spinner);

        thread::spawn(move || {
            let mut index = 1_usize;

            loop {
                let mut all_frames = Vec::new();
                let mut all_texts = Vec::new();
                let mut all_should_stop = Vec::new();

                // collect frames and texts from all spinners
                for (_, spinner) in spinners.lock().unwrap().iter() {
                    let frames = spinner.frames.lock().unwrap().frames.clone();
                    let text = spinner.text.lock().unwrap().clone();
                    let should_stop = spinner.should_stop.lock().unwrap().clone();
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

    /// output frame using crossterm
    fn render_frame(frame: &str) {
        execute!(
            stdout(),
            cursor::SavePosition,
            cursor::MoveTo(0, 1),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Hide,
            Print(frame),
            cursor::RestorePosition
        )
        .unwrap();
    }

    /// to render frames to stdout
    fn render_frames(frames: &[Vec<&str>], index: usize, texts: &[String], should_stop: &[bool]) {
        let first_frame = iterators::balanced_iterator(index, frames);

        let combined_string: String = first_frame
            .iter()
            .zip(texts.iter())
            .zip(should_stop.iter())
            .filter_map(|((frame, text), should_stop)| {
                if *should_stop {
                    Some(format!("{}", text))
                } else {
                    if let Some(frame) = frame {
                        Some(format!("{}  {}", frame, text))
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        MultiSpinner::render_frame(&combined_string);
    }
}
