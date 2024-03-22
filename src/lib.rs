use std::sync::{Arc, Mutex};
use std::thread;

use crate::animations::animation;
pub use crate::animations::frames::spinner;

// declare any modules and submodules
mod animations;

// define the public API of the library
pub struct LoadingAnimation {
    should_stop: Arc<Mutex<bool>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl LoadingAnimation {
    /// Creates a new `LoadingAnimation` instance and starts the loading animation.
    pub fn new(frames: spinner::Frames) -> Self {
        let should_stop = Arc::new(Mutex::new(false));
        let should_stop_clone = Arc::clone(&should_stop);
        let handle = thread::spawn(move || {
            animation::spinner_animation(&frames, should_stop_clone);
        });
        Self {
            should_stop,
            handle: Some(handle),
        }
    }
}

impl Drop for LoadingAnimation {
    /// stops the loading animation thread when the `LoadingAnimation` object is dropped
    ///
    /// when the `LoadingAnimation` object goes out of scope, this method sets the
    /// flag to stop the animation and waits for the animation thread to join
    fn drop(&mut self) {
        // set the flag to stop the animation
        *self.should_stop.lock().unwrap() = true;
        // stop the loading animation thread
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}
