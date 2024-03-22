use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::animations::animation;
pub use crate::animations::frames::spinner;
use crate::spinner::{Frames, PreDefined};

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

/// an advanced loading animation with support for more complex features and configurations
///
/// this struct provides additional functionality and flexibility compared to `LoadingAnimation`
/// it is suitable for scenarios where more advanced customization or control over the loading animation is required
pub struct AdvancedLoadingAnimation {
    tasks: HashMap<usize, Task>, // TODO: implement LoadingTask Struct
    default_frames: Frames,
}

impl AdvancedLoadingAnimation {
    /// Creates a new instance of `YourStruct`.
    ///
    /// If `frames` is `Some(Frames)`, it uses the provided frames; otherwise, it uses a default set of frames.
    pub fn new(frames: Option<Frames>) -> Self {
        let default_frames = frames.unwrap_or_else(|| PreDefined::dot_spinner1());

        Self { default_frames }
    }

    pub fn add(frames: Option<Frames>, msg: Option<&str>)-> usize {
        let frames = frames.unwrap_or_else(|| PreDefined::dot_spinner1());
        // TODO: add new loading bar implementation
        // TODO: add uid for each task and retrun so later on the user cna control each loading animation independently


    }
    // TODO: Define methods for more advanced features
}
