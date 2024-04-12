//! Predefined Spinner Frames
//! ```
//! use zenity::spinner::Frames;
//! use zenity::style::StyledString;
//! use zenity::styled_string;
//! let frames: Frames = Frames {
//!         frames: styled_string!["◐", "◓", "◑", "◒"],
//!         speed_ms: 100,
//!         text: StyledString{string: "".to_string(),style: Default::default()},
//!         stop: true,
//!  };
//! # assert_eq!(frames.frames, styled_string!["◐", "◓", "◑", "◒"]);
//! # assert_eq!(frames.speed_ms, 100);
//! ```

use crate::style::StyledString;
use crate::styled_string;

/// represents a collection of frames and their display speed, typically used for animations
///
/// # Example
///
/// ```
/// use zenity::spinner::Frames;
/// use zenity::style::StyledString;
/// use zenity::styled_string;
///
/// let frames: Frames = Frames {
///         frames: styled_string!["◐", "◓", "◑", "◒"],
///         speed_ms: 100,
///         text: StyledString{string: "".to_string(),style: Default::default()},
///         stop: true,
///  };
/// # assert_eq!(frames.frames, styled_string!["◐", "◓", "◑", "◒"]);
/// # assert_eq!(frames.speed_ms, 100);
/// ```
#[derive(Clone)]
pub struct Frames {
    /// the sequence of frames to be displayed
    pub frames: Vec<StyledString>,
    /// the speed at which each frame should be displayed, in milliseconds
    pub speed_ms: u64,
    /// String to display behind the spinner
    pub text: StyledString,
    /// if the animation is active
    pub stop: bool,
}

impl Default for Frames {
    /// creates a new Progress instance
    ///
    /// ## Example
    /// ```
    /// use zenity::spinner::{Frames, MultiSpinner};
    /// let spinner = MultiSpinner::new(Frames::default());
    /// ```
    fn default() -> Self {
        Self::dots_simple_big3()
    }
}

impl AsRef<Frames> for Frames {
    fn as_ref(&self) -> &Frames {
        self
    }
}

/// ```
/// use zenity::spinner::{Frames, MultiSpinner};
/// let spinner = MultiSpinner::new(Frames::default());
/// ```
impl Frames {
    /// generates frames for spinner animation based on the provided pattern, inversion flag, and speed
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
    /// use zenity::spinner::{Frames};
    /// use zenity::style::StyledString;
    /// use zenity::{style, styled_string};
    ///
    /// let spinner_frames = Frames::generate_frames(styled_string!["◐", "◓", "◑", "◒"], 100);
    /// # assert_eq!(spinner_frames.frames, styled_string!["◐", "◓", "◑", "◒"]);
    /// # assert_eq!(spinner_frames.speed_ms, 100);
    /// ```
    pub fn generate_frames(frames: Vec<StyledString>, speed_ms: u64) -> Frames {
        Frames {
            frames,
            speed_ms,
            stop: false,
            text: StyledString {
                string: "".to_string(),
                style: Default::default(),
            },
        }
    }

    /// stops a spinner animation
    ///
    /// ```
    /// # use zenity::spinner::MultiSpinner;
    /// #
    /// # let spinner = MultiSpinner::default();
    /// #
    /// // stop spinner manual:
    /// spinner.stop(&spinner.get_last());
    /// ```
    pub fn stop(&mut self) {
        self.stop = true;
    }

    /// ⠋
    /// ⠹
    /// ⠧
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::Frames;
    /// use zenity::style::StyledString;
    /// use zenity::styled_string;
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner1();
    /// # assert_eq!(spinner_frames.frames, styled_string!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]);
    /// ```
    pub fn dot_spinner1() -> Frames {
        let pattern = styled_string!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_spinner2
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner2();
    /// ```
    pub fn dot_spinner2() -> Frames {
        let pattern = styled_string!["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_spinner3
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner3();
    /// ```
    pub fn dot_spinner3() -> Frames {
        let pattern = styled_string!["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_spinner4
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner4();
    /// ```
    pub fn dot_spinner4() -> Frames {
        let pattern = styled_string![
            "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆"
        ];
        Self::generate_frames(pattern, 120)
    }

    /// # dot_spinner5
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner5();
    /// ```
    pub fn dot_spinner5() -> Frames {
        let pattern = styled_string![
            "⠀", "⠁", "⠂", "⠃", "⠄", "⠅", "⠆", "⠇", "⡀", "⡁", "⡂", "⡃", "⡄", "⡅", "⡆", "⡇", "⠈",
            "⠉", "⠊", "⠋", "⠌", "⠍", "⠎", "⠏", "⡈", "⡉", "⡊", "⡋", "⡌", "⡍", "⡎", "⡏", "⠐", "⠑",
            "⠒", "⠓", "⠔", "⠕", "⠖", "⠗", "⡐", "⡑", "⡒", "⡓", "⡔", "⡕", "⡖", "⡗", "⠘", "⠙", "⠚",
            "⠛", "⠜", "⠝", "⠞", "⠟", "⡘", "⡙", "⡚", "⡛", "⡜", "⡝", "⡞", "⡟", "⠠", "⠡", "⠢", "⠣",
            "⠤", "⠥", "⠦", "⠧", "⡠", "⡡", "⡢", "⡣", "⡤", "⡥", "⡦", "⡧", "⠨", "⠩", "⠪", "⠫", "⠬",
            "⠭", "⠮", "⠯", "⡨", "⡩", "⡪", "⡫", "⡬", "⡭", "⡮", "⡯", "⠰", "⠱", "⠲", "⠳", "⠴", "⠵",
            "⠶", "⠷", "⡰", "⡱", "⡲", "⡳", "⡴", "⡵", "⡶", "⡷", "⠸", "⠹", "⠺", "⠻", "⠼", "⠽", "⠾",
            "⠿", "⡸", "⡹", "⡺", "⡻", "⡼", "⡽", "⡾", "⡿", "⢀", "⢁", "⢂", "⢃", "⢄", "⢅", "⢆", "⢇",
            "⣀", "⣁", "⣂", "⣃", "⣄", "⣅", "⣆", "⣇", "⢈", "⢉", "⢊", "⢋", "⢌", "⢍", "⢎", "⢏", "⣈",
            "⣉", "⣊", "⣋", "⣌", "⣍", "⣎", "⣏", "⢐", "⢑", "⢒", "⢓", "⢔", "⢕", "⢖", "⢗", "⣐", "⣑",
            "⣒", "⣓", "⣔", "⣕", "⣖", "⣗", "⢘", "⢙", "⢚", "⢛", "⢜", "⢝", "⢞", "⢟", "⣘", "⣙", "⣚",
            "⣛", "⣜", "⣝", "⣞", "⣟", "⢠", "⢡", "⢢", "⢣", "⢤", "⢥", "⢦", "⢧", "⣠", "⣡", "⣢", "⣣",
            "⣤", "⣥", "⣦", "⣧", "⢨", "⢩", "⢪", "⢫", "⢬", "⢭", "⢮", "⢯", "⣨", "⣩", "⣪", "⣫", "⣬",
            "⣭", "⣮", "⣯", "⢰", "⢱", "⢲", "⢳", "⢴", "⢵", "⢶", "⢷", "⣰", "⣱", "⣲", "⣳", "⣴", "⣵",
            "⣶", "⣷", "⢸", "⢹", "⢺", "⢻", "⢼", "⢽", "⢾", "⢿", "⣸", "⣹", "⣺", "⣻", "⣼", "⣽", "⣾",
            "⣿"
        ];
        Self::generate_frames(pattern, 60)
    }

    /// # dot_spinner6
    ///
    /// ⠁ ⠂ ⠄ ⡀ ⢀ ⠠ ⠐ ⠈
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner6();
    /// ```
    pub fn dot_spinner6() -> Frames {
        let pattern = styled_string!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_spinner7
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner7();
    /// ```
    pub fn dot_spinner7() -> Frames {
        let pattern = styled_string!["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_spinner8
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner8();
    /// ```
    pub fn dot_spinner8() -> Frames {
        let pattern = styled_string![
            "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦", "⣮",
            "⣶", "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉", "⠑", "⠡",
            "⢁"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_spinner9
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner9();
    /// ```
    pub fn dot_spinner9() -> Frames {
        let pattern = styled_string![
            "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁",
            "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘",
            "⢃⠨", "⡃⢐", "⠍⡐", "⢋⠠", "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩",
            "⠈⢙", "⠈⡙", "⠈⠩", "⠀⢙", "⠀⡙", "⠀⠩", "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_spinner10
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner10();
    /// ```
    pub fn dot_spinner10() -> Frames {
        let pattern = styled_string!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_spinner11
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner11();
    /// ```
    pub fn dot_spinner11() -> Frames {
        let pattern = styled_string!["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"];
        Self::generate_frames(pattern, 100)
    }

    /// # kaomoji
    ///
    /// (　´･ω)
    /// (´･ω･`)
    /// (ω･`　)
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::kaomoji();
    /// ```
    pub fn kaomoji() -> Frames {
        let pattern = styled_string![
            "(　´･ω)",
            "( ´･ω･)",
            "(´･ω･`)",
            "(･ω･` )",
            "(ω･`　)",
            "(･ω･` )",
            "(´･ω･`)",
            "( ´･ω･)",
            "(　´･ω)"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # kaomoji
    ///
    /// ▰▱▱▱▱▱▱
    /// ▰▰▰▰▱▱▱
    /// ▰▰▰▰▰▰▰
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::kaomoji();
    /// ```
    pub fn aesthetic_spin() -> Frames {
        let pattern = styled_string![
            "▱ ▱ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▱ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▰ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▰ ▰ ",
            "▱ ▰ ▰ ▰ ▰ ▰ ▰ ",
            "▱ ▱ ▰ ▰ ▰ ▰ ▰ ",
            "▱ ▱ ▱ ▰ ▰ ▰ ▰ ",
            "▱ ▱ ▱ ▱ ▰ ▰ ▰ ",
            "▱ ▱ ▱ ▱ ▱ ▰ ▰ ",
            "▱ ▱ ▱ ▱ ▱ ▱ ▰ ",
            "▱ ▱ ▱ ▱ ▱ ▱ ▱ "
        ];
        Self::generate_frames(pattern, 120)
    }

    /// # aesthetic_load
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::aesthetic_load();
    /// ```
    pub fn aesthetic_load() -> Frames {
        let pattern = styled_string![
            "▱ ▱ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▱ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▰ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▰ ▰ "
        ];
        Self::generate_frames(pattern, 180)
    }

    /// # clock
    ///
    /// 🕛 🕘 🕔
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::clock();
    /// ```
    pub fn clock() -> Frames {
        let pattern = styled_string![
            "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 "
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # small_bouncing_bar
    ///
    /// [=   ]  [ ===]  [==  ]
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::small_bouncing_bar();
    /// ```
    pub fn small_bouncing_bar() -> Frames {
        let pattern = styled_string![
            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
            "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]"
        ];
        Self::generate_frames(pattern, 80)
    }

    /// # small_loading_bar
    ///
    /// [=   ]  [ ===]  [==  ]
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::small_loading_bar();
    /// ```
    pub fn small_loading_bar() -> Frames {
        let pattern =
            styled_string!["[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]"];
        Self::generate_frames(pattern, 80)
    }

    /// # loading_bar_with_arrow
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::loading_bar_with_arrow();
    /// ```
    pub fn loading_bar_with_arrow() -> Frames {
        let pattern = styled_string![
            "[                    ]",
            "[=>                  ]",
            "[===>                ]",
            "[=====>              ]",
            "[======>             ]",
            "[========>           ]",
            "[==========>         ]",
            "[============>       ]",
            "[==============>     ]",
            "[================>   ]",
            "[==================> ]",
            "[===================>]"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # short_loading_bar_with_arrow
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::short_loading_bar_with_arrow();
    /// ```
    pub fn short_loading_bar_with_arrow() -> Frames {
        let pattern = styled_string![
            "[>]         >>>]",
            "[>>>         >>]",
            "[>>>>        |>]",
            "[ >>>>       [ ]",
            "[ |>>>>      [ ]",
            "[ ] >>>>     [ ]",
            "[ ]  >>>>    [ ]",
            "[ ]   >>>>   [ ]",
            "[ ]    >>>>  [ ]",
            "[ ]     >>>> [ ]",
            "[ ]      >>>>| ]",
            "[ ]       >>>> ]"
        ];
        Self::generate_frames(pattern, 130)
    }

    /// # material
    ///
    /// ████████████▁▁▁▁▁▁▁▁
    /// ▁▁▁▁▁▁▁▁▁█████████▁▁
    /// ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::material();
    /// ```
    pub fn material() -> Frames {
        let pattern = styled_string![
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███████▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "██████████▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "█████████████▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁██████████████▁▁▁▁",
            "▁▁▁██████████████▁▁▁",
            "▁▁▁▁█████████████▁▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁██████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁▁█████████████▁▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁▁███████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁▁█████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁"
        ];
        Self::generate_frames(pattern, 17)
    }

    /// # moon
    ///
    /// 🌑 🌒 🌓 🌔 🌕 🌖 🌗 🌘
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::moon();
    /// ```
    pub fn moon() -> Frames {
        let pattern = styled_string!["🌑 ", "🌒 ", "🌓 ", "🌔 ", "🌕 ", "🌖 ", "🌗 ", "🌘 "];
        Self::generate_frames(pattern, 130)
    }

    /// # dots_simple1
    ///
    /// . ... .. .
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple1();
    /// ```
    pub fn dots_simple1() -> Frames {
        let pattern = styled_string![".  ", ".. ", "...", " ..", "  .", "   "];
        Self::generate_frames(pattern, 260)
    }

    /// # dots_simple2
    ///
    /// . ... .. .
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple2();
    /// ```
    pub fn dots_simple2() -> Frames {
        let pattern = styled_string!["   ", ".  ", ".. ", "..."];
        Self::generate_frames(pattern, 360)
    }

    /// # japanese
    ///
    /// ｦ ｧ ｨ ｩ
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::japanese();
    /// ```
    pub fn japanese() -> Frames {
        let pattern = styled_string![
            "ｦ", "ｧ", "ｨ", "ｩ", "ｪ", "ｫ", "ｬ", "ｭ", "ｮ", "ｯ", "ｱ", "ｲ", "ｳ", "ｴ", "ｵ", "ｶ", "ｷ",
            "ｸ", "ｹ", "ｺ", "ｻ", "ｼ", "ｽ", "ｾ", "ｿ", "ﾀ", "ﾁ", "ﾂ", "ﾃ", "ﾄ", "ﾅ", "ﾆ", "ﾇ", "ﾈ",
            "ﾉ", "ﾊ", "ﾋ", "ﾌ", "ﾍ", "ﾎ", "ﾏ", "ﾐ", "ﾑ", "ﾒ", "ﾓ", "ﾔ", "ﾕ", "ﾖ", "ﾗ", "ﾘ", "ﾙ",
            "ﾚ", "ﾛ", "ﾜ", "ﾝ"
        ];
        Self::generate_frames(pattern, 180)
    }

    /// # line
    ///
    /// ________  ____-___   ______-_
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::line();
    /// ```
    pub fn line() -> Frames {
        let pattern = styled_string![
            "________", "-_______", "_-______", "__-_____", "___-____", "____-___", "_____-__",
            "______-_", "_______-", "________", "_______-", "______-_", "_____-__", "____-___",
            "___-____", "__-_____", "_-______", "-_______", "________"
        ];
        Self::generate_frames(pattern, 120)
    }

    /// # line2
    ///
    /// |_______  ____|___ ______-_
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::line2();
    /// ```
    pub fn line2() -> Frames {
        let pattern = styled_string![
            "|_______",
            "_/______",
            "__-_____",
            "___\\____",
            "____|___",
            "_____/__",
            "______-_",
            "_______\\",
            "_______|",
            "______\\_",
            "_____-__",
            "____/___",
            "___|____",
            "__\\_____",
            "_-______"
        ];
        Self::generate_frames(pattern, 120)
    }

    /// # block
    ///
    /// ▃  █ ▁
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::block();
    /// ```
    pub fn block() -> Frames {
        let pattern = styled_string![
            "▁", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▁"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # block_spinn
    ///
    /// |_______  ____|___ ______-_
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::block_spinn();
    /// ```
    pub fn block_spinn() -> Frames {
        let pattern = styled_string!["▖", "▘", "▝", "▗"];
        Self::generate_frames(pattern, 100)
    }

    /// # arrow_spinn
    ///
    /// ←  ↑ →
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::arrow_spinn();
    /// ```
    pub fn arrow_spinn() -> Frames {
        let pattern = styled_string!["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"];
        Self::generate_frames(pattern, 100)
    }

    /// # big_arrow_spinn
    ///
    /// ⇐  ⇖ ⇑
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::big_arrow_spinn();
    /// ```
    pub fn big_arrow_spinn() -> Frames {
        let pattern = styled_string!["⇐", "⇖", "⇑", "⇗", "⇒", "⇘", "⇓", "⇙"];
        Self::generate_frames(pattern, 140)
    }

    /// # line_spinner
    ///
    /// ┤ ┘ ├
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::line_spinner();
    /// ```
    pub fn line_spinner() -> Frames {
        let pattern = styled_string!["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"];
        Self::generate_frames(pattern, 120)
    }

    /// # line_spinner_simple
    ///
    /// | / -
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::line_spinner_simple();
    /// ```
    pub fn line_spinner_simple() -> Frames {
        let pattern = styled_string!["|", "/", "-", "\\"];
        Self::generate_frames(pattern, 120)
    }

    /// # corner
    ///
    /// ◢  ◣ ◤
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::corner();
    /// ```
    pub fn corner() -> Frames {
        let pattern = styled_string!["◢", "◣", "◤", "◥"];
        Self::generate_frames(pattern, 160)
    }

    /// # abc
    ///
    /// a b c
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::abc();
    /// ```
    pub fn abc() -> Frames {
        let pattern = styled_string![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z"
        ];
        Self::generate_frames(pattern, 150)
    }

    /// # earth
    ///
    /// 🌍 🌎 🌏
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::earth();
    /// ```
    pub fn earth() -> Frames {
        let pattern = styled_string!["🌍", "🌎", "🌏"];
        Self::generate_frames(pattern, 200)
    }

    /// # arrow_row
    ///
    /// ▹▹▹▹▹  ▹▸▹▹▹ ▹▹▸▹▹
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::arrow_row();
    /// ```
    pub fn arrow_row() -> Frames {
        let pattern = styled_string!["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"];
        Self::generate_frames(pattern, 140)
    }

    /// # fractions
    ///
    /// ⅓  ⅔ ¼
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::fractions();
    /// ```
    pub fn fractions() -> Frames {
        let pattern = styled_string!["½", "⅓", "⅔", "¼", "¾", "⅛", "⅜", "⅝", "⅞"];
        Self::generate_frames(pattern, 100)
    }

    /// # star1
    ///
    /// ✶  ✸ ✺
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::star1();
    /// ```
    pub fn star1() -> Frames {
        let pattern = styled_string!["✶", "✸", "✹", "✺", "✹", "✷"];
        Self::generate_frames(pattern, 180)
    }

    /// # star2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::star2();
    /// ```
    pub fn star2() -> Frames {
        let pattern = styled_string!["+", "x", "*"];
        Self::generate_frames(pattern, 180)
    }

    /// # dot_bounce
    ///
    /// .  O °
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_bounce();
    /// ```
    pub fn dot_bounce() -> Frames {
        let pattern = styled_string![".", "o", "O", "°", "O", "o", "."];
        Self::generate_frames(pattern, 120)
    }

    /// # flip
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::flip();
    /// ```
    pub fn flip() -> Frames {
        let pattern = styled_string!["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"];
        Self::generate_frames(pattern, 120)
    }

    /// # binary
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::binary();
    /// ```
    pub fn binary() -> Frames {
        let pattern = styled_string![
            "010010", "001100", "100101", "111010", "111101", "010111", "101011", "111000",
            "110011", "110101"
        ];
        Self::generate_frames(pattern, 80)
    }

    /// # big_loading_bar
    ///
    /// ▒▒▒▒▒▒▒▒▒▒  ███▒▒▒▒▒▒▒  ██████████
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::big_loading_bar();
    /// ```
    pub fn big_loading_bar() -> Frames {
        let pattern = styled_string![
            "▒▒▒▒▒▒▒▒▒▒",
            "█▒▒▒▒▒▒▒▒▒",
            "███▒▒▒▒▒▒▒",
            "█████▒▒▒▒▒",
            "███████▒▒▒",
            "██████████"
        ];
        Self::generate_frames(pattern, 240)
    }

    /// # wall_bounce
    ///
    /// ▐⠂       ▌  ▐  ⠠     ▌ ▐       ⠠▌
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wall_bounce();
    /// ```
    pub fn wall_bounce() -> Frames {
        let pattern = styled_string![
            "▐⠂       ▌",
            "▐⠈       ▌",
            "▐ ⠂      ▌",
            "▐ ⠠      ▌",
            "▐  ⡀     ▌",
            "▐  ⠠     ▌",
            "▐   ⠂    ▌",
            "▐   ⠈    ▌",
            "▐    ⠂   ▌",
            "▐    ⠠   ▌",
            "▐     ⡀  ▌",
            "▐     ⠠  ▌",
            "▐      ⠂ ▌",
            "▐      ⠈ ▌",
            "▐       ⠂▌",
            "▐       ⠠▌",
            "▐       ⡀▌",
            "▐      ⠠ ▌",
            "▐      ⠂ ▌",
            "▐     ⠈  ▌",
            "▐     ⠂  ▌",
            "▐    ⠠   ▌",
            "▐    ⡀   ▌",
            "▐   ⠠    ▌",
            "▐   ⠂    ▌",
            "▐  ⠈     ▌",
            "▐  ⠂     ▌",
            "▐ ⠠      ▌",
            "▐ ⡀      ▌",
            "▐⠠       ▌"
        ];
        Self::generate_frames(pattern, 140)
    }

    /// # wall_bounce_line
    ///
    /// ▐__/|__________▌
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wall_bounce_line();
    /// ```
    pub fn wall_bounce_line() -> Frames {
        let pattern = styled_string![
            "▐|\\____________▌",
            "▐_|\\___________▌",
            "▐__|\\__________▌",
            "▐___|\\_________▌",
            "▐____|\\________▌",
            "▐_____|\\_______▌",
            "▐______|\\______▌",
            "▐_______|\\_____▌",
            "▐________|\\____▌",
            "▐_________|\\___▌",
            "▐__________|\\__▌",
            "▐___________|\\_▌",
            "▐____________|\\▌",
            "▐____________/|▌",
            "▐___________/|_▌",
            "▐__________/|__▌",
            "▐_________/|___▌",
            "▐________/|____▌",
            "▐_______/|_____▌",
            "▐______/|______▌",
            "▐_____/|_______▌",
            "▐____/|________▌",
            "▐___/|_________▌",
            "▐__/|__________▌",
            "▐_/|___________▌",
            "▐/|____________▌"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # stack
    ///
    /// ☱ ☲ ☴
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::stack();
    /// ```
    pub fn stack() -> Frames {
        let pattern = styled_string!["☱", "☲", "☴"];
        Self::generate_frames(pattern, 200)
    }

    /// # toggle
    ///
    /// ⊶ ⊷
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle();
    /// ```
    pub fn toggle() -> Frames {
        let pattern = styled_string!["⊶", "⊷"];
        Self::generate_frames(pattern, 250)
    }

    /// # toggle2
    ///
    /// ▫ ▪
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle2();
    /// ```
    pub fn toggle2() -> Frames {
        let pattern = styled_string!["▫", "▪"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle3
    ///
    /// □ ■
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle3();
    /// ```
    pub fn toggle3() -> Frames {
        let pattern = styled_string!["□", "■"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle4
    ///
    /// ■ □ ▪ ▫
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle4();
    /// ```
    pub fn toggle4() -> Frames {
        let pattern = styled_string!["■", "□", "▪", "▫"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle5
    ///
    /// ▮ ▯
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle5();
    /// ```
    pub fn toggle5() -> Frames {
        let pattern = styled_string!["▮ ", "▯ "];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle6
    ///
    /// ဝ ၀
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle6();
    /// ```
    pub fn toggle6() -> Frames {
        let pattern = styled_string!["ဝ", "၀"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle7
    ///
    /// ⦾ ⦿
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle7();
    /// ```
    pub fn toggle7() -> Frames {
        let pattern = styled_string!["⦾", "⦿"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle8
    ///
    /// ◍ ◌
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle8();
    /// ```
    pub fn toggle8() -> Frames {
        let pattern = styled_string!["◍", "◌"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle9
    ///
    /// ◉ ◎
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle9();
    /// ```
    pub fn toggle9() -> Frames {
        let pattern = styled_string!["◉", "◎"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle10
    ///
    /// ◉ ◎
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle10();
    /// ```
    pub fn toggle10() -> Frames {
        let pattern = styled_string!["㊂", "㊀", "㊁"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle11
    ///
    /// ⧇ ⧆
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle11();
    /// ```
    pub fn toggle11() -> Frames {
        let pattern = styled_string!["⧇", "⧆"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle12
    ///
    /// ☗ ☖
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle12();
    /// ```
    pub fn toggle12() -> Frames {
        let pattern = styled_string!["☗", "☖"];
        Self::generate_frames(pattern, 240)
    }

    /// # toggle13
    ///
    /// = * -
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle13();
    /// ```
    pub fn toggle13() -> Frames {
        let pattern = styled_string!["=", "*", "-"];
        Self::generate_frames(pattern, 240)
    }

    /// # arc
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::arc();
    /// ```
    pub fn arc() -> Frames {
        let pattern = styled_string!["◜", "◠", "◝", "◞", "◡", "◟"];
        Self::generate_frames(pattern, 120)
    }

    /// # circle
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::circle();
    /// ```
    pub fn circle() -> Frames {
        let pattern = styled_string!["◡", "⊙", "◠"];
        Self::generate_frames(pattern, 200)
    }

    /// # square_corners
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::square_corners();
    /// ```
    pub fn square_corners() -> Frames {
        let pattern = styled_string!["◰ ", "◳ ", "◲ ", "◱ "];
        Self::generate_frames(pattern, 200)
    }

    /// # circle_corners
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::circle_corners();
    /// ```
    pub fn circle_corners() -> Frames {
        let pattern = styled_string!["◴ ", "◷ ", "◶ ", "◵ "];
        Self::generate_frames(pattern, 200)
    }

    /// # circle_halves
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::circle_halves();
    /// ```
    pub fn circle_halves() -> Frames {
        let pattern = styled_string!["◐ ", "◓ ", "◑ ", "◒ "];
        Self::generate_frames(pattern, 200)
    }

    /// # bouncing_ball
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::bouncing_ball();
    /// ```
    pub fn bouncing_ball() -> Frames {
        let pattern = styled_string![
            "( ●    )",
            "(  ●   )",
            "(   ●  )",
            "(    ● )",
            "(     ●)",
            "(    ● )",
            "(   ●  )",
            "(  ●   )",
            "( ●    )",
            "(●     )"
        ];
        Self::generate_frames(pattern, 160)
    }

    /// # smiley
    ///
    ///😄 😝
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::smiley();
    /// ```
    pub fn smiley() -> Frames {
        let pattern = styled_string!["😄 ", "😝 "];
        Self::generate_frames(pattern, 460)
    }

    /// # monkey
    ///
    /// 🙈 🙈 🙉   
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::monkey();
    /// ```
    pub fn monkey() -> Frames {
        let pattern = styled_string!["🙈 ", "🙈 ", "🙉 ", "🙊 "];
        Self::generate_frames(pattern, 440)
    }

    /// # hearts
    ///
    /// 💛 💙 💜 💚 ❤️
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::hearts();
    /// ```
    pub fn hearts() -> Frames {
        let pattern = styled_string!["💛 ", "💙 ", "💜 ", "💚 ", "❤️ "];
        Self::generate_frames(pattern, 240)
    }

    /// # runner
    ///
    ///🚶 🏃
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::runner();
    /// ```
    pub fn runner() -> Frames {
        let pattern = styled_string!["🚶 ", "🏃 "];
        Self::generate_frames(pattern, 240)
    }

    /// # raining
    ///
    /// 🌧 🌧 🌨
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::raining();
    /// ```
    pub fn raining() -> Frames {
        let pattern = styled_string!["🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌨 ", "🌧 ", "🌨 "];
        Self::generate_frames(pattern, 140)
    }

    /// # weather
    ///
    /// ☀️ ⛅️ ☀️
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::weather();
    /// ```
    pub fn weather() -> Frames {
        let pattern = styled_string![
            "☀️ ", "☀️ ", "⛅️ ", "⛅️ ", "☁️ ", "☁️ ", "⛅️ ", "⛅️ ", "☀️ ", "☀️ "
        ];
        Self::generate_frames(pattern, 440)
    }

    /// # christmas_tree
    ///
    /// 🌲 🎄
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::christmas_tree();
    /// ```
    pub fn christmas_tree() -> Frames {
        let pattern = styled_string!["🌲", "🎄"];
        Self::generate_frames(pattern, 340)
    }

    /// # nade
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::nade();
    /// ```
    pub fn nade() -> Frames {
        let pattern = styled_string![
            "،  ", "′  ", " ´ ", " ‾ ", "  ⸌", "  ⸊", "  |", "  ⁎", "  ⁕", " ෴ ", "  ⁓", "   ",
            "   ", "   "
        ];
        Self::generate_frames(pattern, 180)
    }

    /// # dots_simple_big1
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple_big1();
    /// ```
    pub fn dots_simple_big1() -> Frames {
        let pattern = styled_string![
            "●∙∙",
            "●∙∙",
            "●∙∙",
            "●∙∙",
            "∙●∙",
            "∙●∙",
            "∙●∙",
            "∙●∙",
            "∙∙●",
            "∙∙●",
            "∙∙●",
            "∙∙●"
        ];
        Self::generate_frames(pattern, 240)
    }

    /// # dots_simple_big2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple_big2();
    /// ```
    pub fn dots_simple_big2() -> Frames {
        let pattern = styled_string![
            "∙∙∙",
            "∙∙∙",
            "∙∙∙",
            "∙∙∙",
            "●∙∙",
            "●∙∙",
            "●∙∙",
            "●∙∙",
            "∙●∙",
            "∙●∙",
            "∙●∙",
            "∙●∙",
            "∙∙●",
            "∙∙●",
            "∙∙●",
            "∙∙●"
        ];
        Self::generate_frames(pattern, 240)
    }

    /// # dots_simple_big3
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple_big3();
    /// ```
    pub fn dots_simple_big3() -> Frames {
        let pattern = styled_string![
            "∙∙∙",
            "∙∙∙",
            "∙∙∙",
            "●∙∙",
            "●∙∙",
            "●∙∙",
            "●●∙",
            "●●∙",
            "●●∙",
            "●●●",
            "●●●",
            "●●●",
            "∙●●",
            "∙●●",
            "∙●●",
            "∙∙●",
            "∙∙●",
            "∙∙●"
        ];
        Self::generate_frames(pattern, 180)
    }

    /// # dots_simple_big4
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple_big4();
    /// ```
    pub fn dots_simple_big4() -> Frames {
        let pattern = styled_string!["∙∙∙", "●∙∙", "●●∙", "●●●"];
        Self::generate_frames(pattern, 180)
    }

    /// # fist_bump
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::fist_bump();
    /// ```
    pub fn fist_bump() -> Frames {
        let pattern = styled_string![
            "🤜                        🤛 ",
            " 🤜                       🤛 ",
            "  🤜                     🤛  ",
            "    🤜                 🤛    ",
            "      🤜             🤛      ",
            "         🤜       🤛         ",
            "           🤜✨🤛            ",
            "         🤜      🤛          "
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # finger_dance
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::finger_dance();
    /// ```
    pub fn finger_dance() -> Frames {
        let pattern = styled_string!["🤘 ", "🤟 ", "🖖 ", "✋ ", "🤚 ", "👆 "];
        Self::generate_frames(pattern, 280)
    }

    /// # mind_blown
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::mind_blown();
    /// ```
    pub fn mind_blown() -> Frames {
        let pattern = styled_string![
            "😐 ", "😐 ", "😮 ", "😮 ", "😦 ", "😦 ", "😧 ", "😧 ", "🤯 ", "🤯 ", "💥 ", "💥 ",
            "✨ "
        ];
        Self::generate_frames(pattern, 280)
    }

    /// # speaker
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::speaker();
    /// ```
    pub fn speaker() -> Frames {
        let pattern = styled_string!["🔈 ", "🔉 ", "🔊 ", "🔉 "];
        Self::generate_frames(pattern, 200)
    }

    /// # arrows
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::arrows();
    /// ```
    pub fn arrows() -> Frames {
        let pattern = styled_string!["⇢", "⇨", "⇒", "⇉", "⇶"];
        Self::generate_frames(pattern, 150)
    }

    /// # dot_box
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_box();
    /// ```
    pub fn dot_box() -> Frames {
        let pattern = styled_string![".", "·", "•", "¤", "°", "¤", "•", "·"];
        Self::generate_frames(pattern, 150)
    }

    /// # simple_line_spin
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::simple_line_spin();
    /// ```
    pub fn simple_line_spin() -> Frames {
        let pattern = styled_string!["+", "\\", "|", "!", "/", "-", "x"];
        Self::generate_frames(pattern, 150)
    }

    /// # bomb
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::bomb();
    /// ```
    pub fn bomb() -> Frames {
        let pattern = styled_string![
            "💣  ", " 💣  ", "  💣 ", "   💣", "   💣", "   💣", "   💣", "   💥", "    ", "    "
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # dot_bounce2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_bounce2();
    /// ```
    pub fn dot_bounce2() -> Frames {
        let pattern = styled_string![".", "·", "˙", "·", "."];
        Self::generate_frames(pattern, 110)
    }

    /// # orange_pulse
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::orange_pulse();
    /// ```
    pub fn orange_pulse() -> Frames {
        let pattern = styled_string!["🔸", "🔶", "🟠", "🟠", "🔶"];
        Self::generate_frames(pattern, 110)
    }

    /// # blue_pulse
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::blue_pulse();
    /// ```
    pub fn blue_pulse() -> Frames {
        let pattern = styled_string!["🔹", "🔷", "🔵", "🔵", "🔷"];
        Self::generate_frames(pattern, 110)
    }

    /// # green_pulse
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::green_pulse();
    /// ```
    pub fn green_pulse() -> Frames {
        let pattern = styled_string!["🟢", "🟩", "🟩", "🟢"];
        Self::generate_frames(pattern, 110)
    }

    /// # red_pulse
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::red_pulse();
    /// ```
    pub fn red_pulse() -> Frames {
        let pattern = styled_string!["🔴", "🟥", "🟥", "🔴"];
        Self::generate_frames(pattern, 110)
    }

    /// # other
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::other();
    /// ```
    pub fn other() -> Frames {
        let pattern = styled_string!["d", "q", "p", "b"];
        Self::generate_frames(pattern, 110)
    }

    /// # pray
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::pray();
    /// ```
    pub fn pray() -> Frames {
        let pattern = styled_string!["🧍 ", "🚶 ", "🧎 ", "🙇 "];
        Self::generate_frames(pattern, 210)
    }

    /// # wavy
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wavy();
    /// ```
    pub fn wavy() -> Frames {
        let pattern = styled_string![
            "¸¸¸¸¸¸¸¸¸",
            ".¸¸¸¸¸¸¸¸",
            "·.¸¸¸¸¸¸¸",
            "¯`·.¸¸¸¸¸",
            "´¯`·.¸¸¸¸",
            "·´¯`·.¸¸¸",
            ".·´¯`·.¸¸",
            "¸.·´¯`·.¸",
            "¸¸.·´¯`·.",
            "¸¸¸.·´¯`·",
            "¸¸¸¸.·´¯`",
            "¸¸¸¸¸.·´¯",
            "¸¸¸¸¸¸.·´",
            "¸¸¸¸¸¸¸.·",
            "¸¸¸¸¸¸¸¸·"
        ];
        Self::generate_frames(pattern, 80)
    }

    /// # wavy2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wavy2();
    /// ```
    pub fn wavy2() -> Frames {
        let pattern = styled_string![
            "¸.·´¯`·.¸",
            "¸¸.·´¯`·.",
            ".¸¸.·´¯`·",
            "·.¸¸.·´¯`",
            "`·.¸¸.·´¯",
            "¯`·.¸¸.·´",
            "´¯`·.¸¸.·",
            "·´¯`·.¸¸.",
            ".·´¯`·.¸¸"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # wavy3
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wavy3();
    /// ```
    pub fn wavy3() -> Frames {
        let pattern = styled_string![
            "▃▄▅",
            "▆▇█",
            "▇█▇",
            "█▇▆",
            "▇▆▅",
            "▆▅▄",
            "▅▄▂",
            "▄▂▁",
            "▂▁▂",
            "▁▂▃",
            "▂▃▄"
        ];
        Self::generate_frames(pattern, 40)
    }

    /// # wavy4
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wavy4();
    /// ```
    pub fn wavy4() -> Frames {
        let pattern = styled_string![
            "ρββββββ",
            "βρβββββ",
            "ββρββββ",
            "βββρβββ",
            "ββββρββ",
            "βββββρβ",
            "ββββββρ"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # soccer
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::soccer();
    /// ```
    pub fn soccer() -> Frames {
        let pattern = styled_string![
            " 🧑⚽️       🧑 ",
            "🧑  ⚽️      🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑      ⚽️  🧑 ",
            "🧑       ⚽️🧑  ",
            "🧑      ⚽️  🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑  ⚽️      🧑 "
        ];
        Self::generate_frames(pattern, 80)
    }

    /// # layer
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::layer();
    /// ```
    pub fn layer() -> Frames {
        let pattern = styled_string!["-", "=", "≡"];
        Self::generate_frames(pattern, 250)
    }

    /// # matrix_glitch
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::matrix_glitch();
    /// ```
    pub fn matrix_glitch() -> Frames {
        let pattern = styled_string![
            "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
            "⠙⠹⠸⠼⠴⠦⠧⠇⠏⠋",
            "⠹⠸⠼⠴⠦⠧⠇⠏⠋⠙",
            "⠸⠼⠴⠦⠧⠇⠏⠋⠙⠹",
            "⠼⠴⠦⠧⠇⠏⠋⠙⠹⠸",
            "⠴⠦⠧⠇⠏⠋⠙⠹⠸⠼",
            "⠦⠧⠇⠏⠋⠙⠹⠸⠼⠴",
            "⠧⠇⠏⠋⠙⠹⠸⠼⠴⠦",
            "⠇⠏⠋⠙⠹⠸⠼⠴⠦⠧",
            "⠏⠋⠙⠹⠸⠼⠴⠦⠧⠇"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # matrix_glitch2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::matrix_glitch2();
    /// ```
    pub fn matrix_glitch2() -> Frames {
        let pattern = styled_string![
            "█▒▓░█░▒▒▓░░▓▒▓▒█▓░░▓▓░░▓░",
            "█▒▓▒█░▒▓░▓▒▓█▒░▒▓█▒░▓░▓░▓",
            "▒█░░█▒░░▓▓▒░▒▓░░▒▒█▓░▓▓░░",
            "░█░▓█░▒░▒▓░▒▓█▓░░▒░▓░▓░▓▓",
            "▒▓█▓▓▓░▒▒▒▓░▓▒▒▒▒▒▓░░▒▒▓░",
            "▓█▓█▓▒░▒▒▒▓▓▓▓░▒▒▓▒▒▓▒▓▒▒",
            "▒▓█▒▓▓░▓▓▒▓▒▒▒▓▓▒░▓░▒▒▓▓▒",
            "▓░▒▓▒▒▓▓▒░▒▓▒▒░▓▒░▒▓▓▒▒▓▓",
            "▓░░░▓▒▓░░░▒▒▓░░░░▓▓░░▓▒▒▒",
            "▓▒▓▒▓▓▒▓▒▒▓▒▓▓▓▓▒▓▒▓▓▒▒▓▒"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # matrix_glitch2_small
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::Frames;
    ///
    /// let spinner_frames: Frames = Frames::matrix_glitch2_small();
    /// ```
    pub fn matrix_glitch2_small() -> Frames {
        let pattern = styled_string![
            "█▒▓░█░▒▒▓",
            "█▒▓▒█░▒▓░",
            "▒█░░█▒░░▓",
            "░█░▓█░▒░▒",
            "▒▓█▓▓▓░▒▒",
            "▓█▓█▓▒░▒▒",
            "▒▓█▒▓▓░▓▓",
            "▓░▒▓▒▒▓▓▒",
            "▓░░░▓▒▓░░",
            "▓▒▓▒▓▓▒▓▒"
        ];
        Self::generate_frames(pattern, 100)
    }

    /// # dwarf_fortress
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dwarf_fortress();
    /// ```
    pub fn dwarf_fortress() -> Frames {
        let pattern = styled_string![
            " ██████£££  ",
            "☺██████£££  ",
            "☺██████£££  ",
            "☺▓█████£££  ",
            "☺▓█████£££  ",
            "☺▒█████£££  ",
            "☺▒█████£££  ",
            "☺░█████£££  ",
            "☺░█████£££  ",
            "☺ █████£££  ",
            " ☺█████£££  ",
            " ☺█████£££  ",
            " ☺▓████£££  ",
            " ☺▓████£££  ",
            " ☺▒████£££  ",
            " ☺▒████£££  ",
            " ☺░████£££  ",
            " ☺░████£££  ",
            " ☺ ████£££  ",
            "  ☺████£££  ",
            "  ☺████£££  ",
            "  ☺▓███£££  ",
            "  ☺▓███£££  ",
            "  ☺▒███£££  ",
            "  ☺▒███£££  ",
            "  ☺░███£££  ",
            "  ☺░███£££  ",
            "  ☺ ███£££  ",
            "   ☺███£££  ",
            "   ☺███£££  ",
            "   ☺▓██£££  ",
            "   ☺▓██£££  ",
            "   ☺▒██£££  ",
            "   ☺▒██£££  ",
            "   ☺░██£££  ",
            "   ☺░██£££  ",
            "   ☺ ██£££  ",
            "    ☺██£££  ",
            "    ☺██£££  ",
            "    ☺▓█£££  ",
            "    ☺▓█£££  ",
            "    ☺▒█£££  ",
            "    ☺▒█£££  ",
            "    ☺░█£££  ",
            "    ☺░█£££  ",
            "    ☺ █£££  ",
            "     ☺█£££  ",
            "     ☺█£££  ",
            "     ☺▓£££  ",
            "     ☺▓£££  ",
            "     ☺▒£££  ",
            "     ☺▒£££  ",
            "     ☺░£££  ",
            "     ☺░£££  ",
            "     ☺ £££  ",
            "      ☺£££  ",
            "      ☺£££  ",
            "      ☺▓££  ",
            "      ☺▓££  ",
            "      ☺▒££  ",
            "      ☺▒££  ",
            "      ☺░££  ",
            "      ☺░££  ",
            "      ☺ ££  ",
            "       ☺££  ",
            "       ☺££  ",
            "       ☺▓£  ",
            "       ☺▓£  ",
            "       ☺▒£  ",
            "       ☺▒£  ",
            "       ☺░£  ",
            "       ☺░£  ",
            "       ☺ £  ",
            "        ☺£  ",
            "        ☺£  ",
            "        ☺▓  ",
            "        ☺▓  ",
            "        ☺▒  ",
            "        ☺▒  ",
            "        ☺░  ",
            "        ☺░  ",
            "        ☺   ",
            "        ☺  &",
            "        ☺ ☼&",
            "       ☺ ☼ &",
            "       ☺☼  &",
            "      ☺☼  & ",
            "      ‼   & ",
            "     ☺   &  ",
            "    ‼    &  ",
            "   ☺    &   ",
            "  ‼     &   ",
            " ☺     &    ",
            "‼      &    ",
            "      &     ",
            "      &     ",
            "     &   ░  ",
            "     &   ▒  ",
            "    &    ▓  ",
            "    &    £  ",
            "   &    ░£  ",
            "   &    ▒£  ",
            "  &     ▓£  ",
            "  &     ££  ",
            " &     ░££  ",
            " &     ▒££  ",
            "&      ▓££  ",
            "&      £££  ",
            "      ░£££  ",
            "      ▒£££  ",
            "      ▓£££  ",
            "      █£££  ",
            "     ░█£££  ",
            "     ▒█£££  ",
            "     ▓█£££  ",
            "     ██£££  ",
            "    ░██£££  ",
            "    ▒██£££  ",
            "    ▓██£££  ",
            "    ███£££  ",
            "   ░███£££  ",
            "   ▒███£££  ",
            "   ▓███£££  ",
            "   ████£££  ",
            "  ░████£££  ",
            "  ▒████£££  ",
            "  ▓████£££  ",
            "  █████£££  ",
            " ░█████£££  ",
            " ▒█████£££  ",
            " ▓█████£££  ",
            " ██████£££  ",
            " ██████£££  "
        ];
        Self::generate_frames(pattern, 100)
    }
}
