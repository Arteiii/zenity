//! progressbar frames

use std::sync::{Arc, Mutex};

/// struct storing the data needed to render a ProgressBar
#[derive(Clone)]
pub struct Frames {
    /// begin string
    pub begin: Vec<&'static str>,

    /// string to place on complete places
    pub bar_complete_char: Vec<&'static str>,

    /// string to place on incomplete places
    pub bar_incomplete_char: Vec<&'static str>,

    /// ending string
    pub end: Vec<&'static str>,

    /// size of progress bar
    pub size: Arc<Mutex<usize>>,

    /// goal value
    pub goal: Arc<Mutex<usize>>,

    /// current value
    pub current: Arc<Mutex<usize>>,
}

// TODO: add animations by adding +1 for each bar so you can have a wave animation and others

impl Default for Frames {
    fn default() -> Self {
        Self::equal()
    }
}

impl Frames {
    /// generates frames for
    ///
    /// # Arguments
    ///
    /// * `pattern` - a vector of strings representing the frames of the spinner animation
    /// * `inverted` - a boolean flag indicating whether the direction of rotation should be reversed
    /// * `speed_ms` - the speed at which each frame should be displayed, in milliseconds
    ///
    /// # Example
    ///
    /// ```
    /// use zenity::progress::Frames;
    ///
    /// let spinner_frames = Frames::new(vec!["["], vec!["="], vec!["-"], vec!["]"]);
    ///
    /// # assert_eq!(spinner_frames.begin, vec!["["]);
    /// # assert_eq!(spinner_frames.bar_complete_char, vec!["="]);
    /// # assert_eq!(spinner_frames.bar_incomplete_char, vec!["-"]);
    /// # assert_eq!(spinner_frames.end, vec!["]"]);
    /// ```
    pub fn new(
        begin: Vec<&'static str>,
        bar_complete_char: Vec<&'static str>,
        bar_incomplete_char: Vec<&'static str>,
        end: Vec<&'static str>,
    ) -> Frames {
        Frames {
            begin,
            bar_complete_char,
            bar_incomplete_char,
            end,
            size: Arc::new(Mutex::new(30)),
            goal: Arc::new(Mutex::new(100)),
            current: Arc::new(Mutex::new(0)),
        }
    }

    /// Sets the size of the progress bar.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the progress bar as an usize, where 1 represents one character in the loading bar.
    ///
    /// # Returns
    ///
    /// A new Bar object with the modified size.
    pub fn set_size(&self, size: usize) -> Self {
        *self.size.lock().unwrap() = size;

        self.clone()
    }

    /// sets the goal value
    ///
    /// # Arguments
    ///
    /// * `goal` - the new goal value
    ///
    /// # Returns
    ///
    /// a new Bar object with the modified goal value
    pub fn set_goal(&self, goal: usize) -> Self {
        let mut current = self.current.lock().unwrap();
        let mut goal_ref = self.goal.lock().unwrap();
        *goal_ref = goal;
        *current = current.min(goal);

        self.clone()
    }

    /// increments the current value
    ///
    /// # Arguments
    ///
    /// * `num` - the amount to increment by
    ///
    /// # Returns
    ///
    /// a new object with the modified current value
    pub fn inc(&self, num: &usize) -> Self {
        let mut current = self.current.lock().unwrap();
        let goal = *self.goal.lock().unwrap();
        let new_current = (*current + num).min(goal);
        *current = new_current;

        self.clone()
    }

    /// '=' as the complete char and '-' as the incomplete char
    pub fn equal() -> Self {
        Self::new(vec!["["], vec!["="], vec!["-"], vec!["]"])
    }

    /// '#' as the complete char and '.' as the incomplete char
    pub fn hash() -> Self {
        Self::new(vec!["["], vec!["#"], vec!["."], vec!["]"])
    }
    /// '#' as the complete char and '.' as the incomplete char
    pub fn rect() -> Self {
        Self::new(vec![" "], vec!["\u{25A0}"], vec![" "], vec![" "])
    }

    // TODO: add more
}
