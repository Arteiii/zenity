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
        let should_stop_clone = Arc::clone(&should_stop);
        let handle = thread::spawn(move || {
            animation::spinner_animation(&frames, should_stop_clone);
        });
        Self {
            should_stop,
            handle: Some(handle),
        }
    }

    /// stops the loading animation
    ///
    /// this method sets the flag to stop the animation and waits for the animation thread to join
    pub fn finish(&mut self, msg: Option<&str>) {
        *self.should_stop.lock().unwrap() = true;
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl Drop for LoadingAnimation {
    /// stops the loading animation thread when the `LoadingAnimation` object is dropped
    fn drop(&mut self) {
        self.finish(None);
    }
}
