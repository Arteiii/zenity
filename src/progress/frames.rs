//! Predefined Progress Frames
//! ```
//! use zenity::progress::Frames;
//! use zenity::styled_string;
//! use zenity::style::StyledString;
//!
//! let spinner_frames = Frames {
//!             begin: styled_string!["["],
//!             bar_complete_char: styled_string!["="],
//!             limiter: styled_string![""],
//!             bar_incomplete_char: styled_string!["-"],
//!             end: styled_string!["]"],
//!             size: 30,
//!             goal: 100,
//!             current: 0,
//!         };
//! ```

use crossterm::style::{Attribute, ContentStyle};

use crate::style::combine_attributes;
use crate::{
    style::{Color, StyledString},
    styled_string,
};

/// struct storing the data needed to render a ProgressFrames
///
/// Example
/// ```
/// use zenity::progress::Frames;
/// use zenity::styled_string;
/// use zenity::style::StyledString;
///
/// let spinner_frames = Frames {
///             begin: styled_string!["["],
///             bar_complete_char: styled_string!["="],
///             limiter: styled_string![""],
///             bar_incomplete_char: styled_string!["-"],
///             end: styled_string!["]"],
///             size: 30,
///             goal: 100,
///             current: 0,
///         };
/// # assert_eq!(spinner_frames.begin, styled_string!["["]);
/// # assert_eq!(spinner_frames.bar_complete_char, styled_string!["="]);
/// # assert_eq!(spinner_frames.bar_incomplete_char, styled_string!["-"]);
/// # assert_eq!(spinner_frames.end, styled_string!["]"]);
/// # assert_eq!(spinner_frames.size, 30);
/// # assert_eq!(spinner_frames.goal, 100);
/// # assert_eq!(spinner_frames.current, 0);
/// ```
#[derive(Clone, Debug)]
pub struct Frames {
    /// begin string
    pub begin: Vec<StyledString>,

    /// string to place on complete places
    pub bar_complete_char: Vec<StyledString>,

    /// limiter between bar complete and incomplete
    pub limiter: Vec<StyledString>,

    /// string to place on incomplete places
    pub bar_incomplete_char: Vec<StyledString>,

    /// ending string
    pub end: Vec<StyledString>,

    /// size of progress bar
    pub size: usize,

    /// goal value
    pub goal: usize,

    /// current value
    pub current: usize,
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
            && self.size == other.size
            && self.goal == other.goal
            && self.current == other.current
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
    /// use zenity::style::StyledString;
    /// use zenity::styled_string;
    ///
    /// let spinner_frames = Frames::new(
    ///     styled_string!["["],
    ///     styled_string!["="],
    ///     styled_string![""],
    ///     styled_string!["-"],
    ///     styled_string!["]"]
    /// );
    /// # assert_eq!(spinner_frames.begin, styled_string!["["]);
    /// # assert_eq!(spinner_frames.bar_complete_char, styled_string!["="]);
    /// # assert_eq!(spinner_frames.bar_incomplete_char, styled_string!["-"]);
    /// # assert_eq!(spinner_frames.end, styled_string!["]"]);
    /// ```
    pub fn new(
        begin: Vec<StyledString>,
        bar_complete_char: Vec<StyledString>,
        limiter: Vec<StyledString>,
        bar_incomplete_char: Vec<StyledString>,
        end: Vec<StyledString>,
    ) -> Frames {
        Frames {
            begin,
            bar_complete_char,
            limiter,
            bar_incomplete_char,
            end,
            size: 30,
            goal: 100,
            current: 0,
        }
    }
    /// Sets the size of the progress bar.
    ///
    /// # Arguments
    ///
    /// * `Size` - The size of the progress bar as an usize,
    /// where 1 represents one character in the loading bar.
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
    /// # assert_eq!(bar.size, 20);
    /// ```
    pub fn set_size(&mut self, size: usize) -> Self {
        self.size = size;

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
    /// # assert_eq!(bar.goal, 100);
    /// ```
    pub fn set_goal(&self, goal: usize) -> Self {
        let _ = self.current.min(goal);

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
    /// # assert_eq!(bar.current, 10);
    /// ```
    pub fn inc(&mut self, num: &usize) -> Self {
        self.current = (self.current + num).min(self.goal);

        self.clone()
    }

    /// '=' as the complete char and '-' as the incomplete char
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    /// use zenity::style::StyledString;
    /// use zenity::styled_string;
    ///
    /// let bar = Frames::equal();
    /// # assert_eq!(bar.begin, styled_string!["["]);
    /// # assert_eq!(bar.bar_complete_char, styled_string!["="]);
    /// # assert_eq!(bar.bar_incomplete_char, styled_string!["-"]);
    /// # assert_eq!(bar.end, styled_string!["]"]);
    /// ```
    pub fn equal() -> Self {
        Self::new(
            styled_string!["["],
            styled_string!["="],
            styled_string![""],
            styled_string!["-"],
            styled_string!["]"],
        )
    }

    /// '#' as the complete char and '.' as the incomplete char
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    /// use zenity::style::StyledString;
    /// use zenity::styled_string;
    ///
    /// let bar = Frames::hash();
    /// # assert_eq!(bar.begin, styled_string!["["]);
    /// # assert_eq!(bar.bar_complete_char, styled_string!["#"]);
    /// # assert_eq!(bar.bar_incomplete_char, styled_string!["."]);
    /// # assert_eq!(bar.end, styled_string!["]"]);
    /// ```
    pub fn hash() -> Self {
        Self::new(
            styled_string!["["],
            styled_string!["#"],
            styled_string![""],
            styled_string!["."],
            styled_string!["]"],
        )
    }

    /// '■' as the complete char and ' ' as the incomplete char
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    /// use zenity::style::StyledString;
    /// use zenity::styled_string;
    ///
    /// let bar = Frames::rect();
    /// # assert_eq!(bar.begin, styled_string![" "]);
    /// # assert_eq!(bar.bar_complete_char, styled_string!["\u{25A0}"]);
    /// # assert_eq!(bar.bar_incomplete_char, styled_string![" "]);
    /// # assert_eq!(bar.end, styled_string![" "]);
    /// ```
    pub fn rect() -> Self {
        Self::new(
            styled_string![" "],
            styled_string!["\u{25A0}"],
            styled_string![""],
            styled_string![" "],
            styled_string![" "],
        )
    }

    /// '■' as the complete char and ' ' as the incomplete char
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    /// use zenity::style::StyledString;
    /// use zenity::styled_string;
    ///
    /// let bar = Frames::dotted_rich();
    /// # assert_eq!(bar.begin, styled_string![" "]);
    /// # assert_eq!(bar.end, styled_string![" "]);
    /// ```
    pub fn dotted_rich() -> Self {
        Self::new(
            styled_string![" "],
            vec![StyledString {
                string: "━".to_string(),
                style: ContentStyle {
                    foreground_color: Some(Color::Rgb {
                        r: 245,
                        g: 48,
                        b: 119,
                    }),
                    background_color: None,
                    underline_color: None,
                    attributes: combine_attributes(&[&Attribute::Bold]),
                },
            }],
            vec![StyledString::simple("╸", None, None, None)],
            vec![StyledString {
                string: "╸".to_string(),
                style: ContentStyle {
                    foreground_color: Some(Color::DarkGrey),
                    background_color: None,
                    underline_color: None,
                    attributes: combine_attributes(&[&Attribute::NoBold]),
                },
            }],
            styled_string![" "],
        )
    }

    /// '━' as the complete char and '━' as the incomplete char
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::progress::Frames;
    /// use zenity::style::StyledString;
    /// use zenity::styled_string;
    ///
    /// let bar = Frames::rich();
    /// # assert_eq!(bar.begin, styled_string![" "]);
    /// # assert_eq!(bar.end, styled_string![" "]);
    /// ```
    pub fn rich() -> Self {
        Self::new(
            styled_string![" "],
            vec![StyledString {
                string: "━".to_string(),
                style: ContentStyle {
                    foreground_color: Some(Color::Rgb {
                        r: 245,
                        g: 48,
                        b: 119,
                    }),
                    background_color: None,
                    underline_color: None,
                    attributes: combine_attributes(&[&Attribute::Bold]),
                },
            }],
            vec![StyledString::simple("╺", Some(Color::DarkGrey), None, None)],
            vec![StyledString {
                string: "━".to_string(),
                style: ContentStyle {
                    foreground_color: Some(Color::DarkGrey),
                    background_color: None,
                    underline_color: None,
                    attributes: combine_attributes(&[]),
                },
            }],
            styled_string![" "],
        )
    }

    // TODO: add more
}
