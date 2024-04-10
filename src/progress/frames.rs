//! Predefined Progress Frames
//! ```
//! use std::sync::{Arc, Mutex};
//! use zenity::progress::Frames;
//! 
//! let spinner_frames = Frames {
//!             begin: vec!["["],
//!             bar_complete_char: vec!["="],
//!             bar_incomplete_char: vec!["-"],
//!             end: vec!["]"],
//!             size: Arc::new(Mutex::new(30)),
//!             goal: Arc::new(Mutex::new(100)),
//!             current: Arc::new(Mutex::new(0)),
//!         };
//! ```

use std::sync::{Arc, Mutex};

/// struct storing the data needed to render a ProgressFrames
///
/// Example
/// ```
/// use std::sync::{Arc, Mutex};
/// use zenity::progress::Frames;
///
/// let spinner_frames = Frames {
///             begin: vec!["["],
///             bar_complete_char: vec!["="],
///             bar_incomplete_char: vec!["-"],
///             end: vec!["]"],
///             size: Arc::new(Mutex::new(30)),
///             goal: Arc::new(Mutex::new(100)),
///             current: Arc::new(Mutex::new(0)),
///         };
/// # assert_eq!(spinner_frames.begin, vec!["["]);
/// # assert_eq!(spinner_frames.bar_complete_char, vec!["="]);
/// # assert_eq!(spinner_frames.bar_incomplete_char, vec!["-"]);
/// # assert_eq!(spinner_frames.end, vec!["]"]);
/// # assert_eq!(*spinner_frames.size.lock().unwrap(), 30);
/// # assert_eq!(*spinner_frames.goal.lock().unwrap(), 100);
/// # assert_eq!(*spinner_frames.current.lock().unwrap(), 0);
/// ```
#[derive(Clone, Debug)]
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
    /// default implementation for `Frames`
    ///
    /// by default, it returns a `Frames` instance generated by the `equal()` method, which creates
    /// a simple frame animation with equal frames
    ///
    /// # Example
    ///
    /// ```
    /// use zenity::progress::Frames;
    ///
    /// let frames = Frames::default();
    /// assert_eq!(frames, Frames::equal());
    /// ```
    fn default() -> Self {
        Self::equal()
    }
}

impl PartialEq for Frames {
    fn eq(&self, other: &Self) -> bool {
        self.begin == other.begin
            && self.bar_complete_char == other.bar_complete_char
            && self.bar_incomplete_char == other.bar_incomplete_char
            && self.end == other.end
            && *self.size.lock().unwrap() == *other.size.lock().unwrap()
            && *self.goal.lock().unwrap() == *other.goal.lock().unwrap()
            && *self.current.lock().unwrap() == *other.current.lock().unwrap()
    }
}

impl Eq for Frames {}


/// ```
/// use zenity::progress::Frames;
///
/// let frames = Frames::default();
/// ```
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
    /// A new Frames object with the modified size.
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    ///
    /// let bar = Frames::default().set_size(20);
    /// # assert_eq!(*bar.size.lock().unwrap(), 20);
    /// ```
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
    /// a new Frames object with the modified goal value
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    ///
    /// let bar = Frames::default().set_goal(100);
    /// # assert_eq!(*bar.goal.lock().unwrap(), 100);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    ///
    /// let bar = Frames::default().set_goal(100).inc(&10);
    /// # assert_eq!(*bar.current.lock().unwrap(), 10);
    /// ```
    pub fn inc(&self, num: &usize) -> Self {
        let mut current = self.current.lock().unwrap();
        let goal = *self.goal.lock().unwrap();
        let new_current = (*current + num).min(goal);
        *current = new_current;

        self.clone()
    }

    /// '=' as the complete char and '-' as the incomplete char
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    ///
    /// let bar = Frames::equal();
    /// # assert_eq!(bar.begin, vec!["["]);
    /// # assert_eq!(bar.bar_complete_char, vec!["="]);
    /// # assert_eq!(bar.bar_incomplete_char, vec!["-"]);
    /// # assert_eq!(bar.end, vec!["]"]);
    /// ```
    pub fn equal() -> Self {
        Self::new(vec!["["], vec!["="], vec!["-"], vec!["]"])
    }

    /// '#' as the complete char and '.' as the incomplete char
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    ///
    /// let bar = Frames::hash();
    /// # assert_eq!(bar.begin, vec!["["]);
    /// # assert_eq!(bar.bar_complete_char, vec!["#"]);
    /// # assert_eq!(bar.bar_incomplete_char, vec!["."]);
    /// # assert_eq!(bar.end, vec!["]"]);
    /// ```
    pub fn hash() -> Self {
        Self::new(vec!["["], vec!["#"], vec!["."], vec!["]"])
    }

    /// '■' as the complete char and ' ' as the incomplete char
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    ///
    /// let bar = Frames::rect();
    /// # assert_eq!(bar.begin, vec![" "]);
    /// # assert_eq!(bar.bar_complete_char, vec!["\u{25A0}"]);
    /// # assert_eq!(bar.bar_incomplete_char, vec![" "]);
    /// # assert_eq!(bar.end, vec![" "]);
    /// ```
    pub fn rect() -> Self {
        Self::new(vec![" "], vec!["\u{25A0}"], vec![" "], vec![" "])
    }

    // TODO: add more

}
