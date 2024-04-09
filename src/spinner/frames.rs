//! Predefined Frames and other aniamtions

/// represents a collection of frames and their display speed, typically used for animations
///
/// # Example
///
/// ```
/// use zenity::spinner::Frames;
///
/// let frames: Frames = Frames { frames: vec!["◐", "◓", "◑", "◒"], speed_ms: 100 };
/// assert_eq!(frames.frames, vec!["◐", "◓", "◑", "◒"]);
/// assert_eq!(frames.speed_ms, 100);
/// ```
pub struct Frames {
    /// the sequence of frames to be displayed
    pub frames: Vec<&'static str>,
    /// the speed at which each frame should be displayed, in milliseconds
    pub speed_ms: u64,
}

impl Default for Frames {
    fn default() -> Self {
        Self::dots_simple_big1(false)
    }
}

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
    ///
    /// let spinner_frames = Frames::generate_frames(vec!["◐", "◓", "◑", "◒"], false, 100);
    ///
    /// assert_eq!(spinner_frames.frames, vec!["◐", "◓", "◑", "◒"]);
    /// assert_eq!(spinner_frames.speed_ms, 100);
    /// ```
    pub fn generate_frames(pattern: Vec<&'static str>, inverted: bool, speed_ms: u64) -> Frames {
        let mut frames = pattern;

        if inverted {
            frames.reverse();
        }

        Frames { frames, speed_ms }
    }


    /// ⠋
    /// ⠹
    /// ⠧
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::Frames;
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner1(false);
    /// ```
    pub fn dot_spinner1(inverted: bool) -> Frames {
        let pattern = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_spinner2
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner2(false);
    /// ```
    pub fn dot_spinner2(inverted: bool) -> Frames {
        let pattern = vec!["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_spinner3
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner3(false);
    /// ```
    pub fn dot_spinner3(inverted: bool) -> Frames {
        let pattern = vec!["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_spinner4
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner4(false);
    /// ```
    pub fn dot_spinner4(inverted: bool) -> Frames {
        let pattern = vec![
            "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆",
        ];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # dot_spinner5
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner5(false);
    /// ```
    pub fn dot_spinner5(inverted: bool) -> Frames {
        let pattern = vec![
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
            "⣿",
        ];
        Self::generate_frames(pattern, inverted, 60)
    }

    /// # dot_spinner6
    ///
    /// ⠁ ⠂ ⠄ ⡀ ⢀ ⠠ ⠐ ⠈
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner6(false);
    /// ```
    pub fn dot_spinner6(inverted: bool) -> Frames {
        let pattern = vec!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_spinner7
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner7(false);
    /// ```
    pub fn dot_spinner7(inverted: bool) -> Frames {
        let pattern = vec!["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_spinner8
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner8(false);
    /// ```
    pub fn dot_spinner8(inverted: bool) -> Frames {
        let pattern = vec![
            "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦", "⣮",
            "⣶", "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉", "⠑", "⠡",
            "⢁",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_spinner9
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner9(false);
    /// ```
    pub fn dot_spinner9(inverted: bool) -> Frames {
        let pattern = vec![
            "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁",
            "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘",
            "⢃⠨", "⡃⢐", "⠍⡐", "⢋⠠", "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩",
            "⠈⢙", "⠈⡙", "⠈⠩", "⠀⢙", "⠀⡙", "⠀⠩", "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_spinner10
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner10(false);
    /// ```
    pub fn dot_spinner10(inverted: bool) -> Frames {
        let pattern = vec!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_spinner11
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_spinner11(false);
    /// ```
    pub fn dot_spinner11(inverted: bool) -> Frames {
        let pattern = vec!["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"];
        Self::generate_frames(pattern, inverted, 100)
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
    /// let spinner_frames: Frames = Frames::kaomoji(false);
    /// ```
    pub fn kaomoji(inverted: bool) -> Frames {
        let pattern = vec![
            "(　´･ω)",
            "( ´･ω･)",
            "(´･ω･`)",
            "(･ω･` )",
            "(ω･`　)",
            "(･ω･` )",
            "(´･ω･`)",
            "( ´･ω･)",
            "(　´･ω)",
        ];
        Self::generate_frames(pattern, inverted, 100)
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
    /// let spinner_frames: Frames = Frames::kaomoji(false);
    /// ```
    pub fn aesthetic_spin(inverted: bool) -> Frames {
        let pattern = vec![
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
            "▱ ▱ ▱ ▱ ▱ ▱ ▱ ",
        ];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # aesthetic_load
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::aesthetic_load(false);
    /// ```
    pub fn aesthetic_load(inverted: bool) -> Frames {
        let pattern = vec![
            "▱ ▱ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▱ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▱ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▱ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▱ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▱ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▰ ▱ ",
            "▰ ▰ ▰ ▰ ▰ ▰ ▰ ",
        ];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// # clock
    ///
    /// 🕛 🕘 🕔
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::clock(false);
    /// ```
    pub fn clock(inverted: bool) -> Frames {
        let pattern = vec![
            "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 ",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # small_bouncing_bar
    ///
    /// [=   ]  [ ===]  [==  ]
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::small_bouncing_bar(false);
    /// ```
    pub fn small_bouncing_bar(inverted: bool) -> Frames {
        let pattern = vec![
            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
            "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
        ];
        Self::generate_frames(pattern, inverted, 80)
    }

    /// # small_loading_bar
    ///
    /// [=   ]  [ ===]  [==  ]
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::small_loading_bar(false);
    /// ```
    pub fn small_loading_bar(inverted: bool) -> Frames {
        let pattern = vec![
            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]",
        ];
        Self::generate_frames(pattern, inverted, 80)
    }

    /// # loading_bar_with_arrow
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::loading_bar_with_arrow(false);
    /// ```
    pub fn loading_bar_with_arrow(inverted: bool) -> Frames {
        let pattern = vec![
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
            "[===================>]",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # short_loading_bar_with_arrow
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::short_loading_bar_with_arrow(false);
    /// ```
    pub fn short_loading_bar_with_arrow(inverted: bool) -> Frames {
        let pattern = vec![
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
            "[ ]       >>>> ]",
        ];
        Self::generate_frames(pattern, inverted, 130)
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
    /// let spinner_frames: Frames = Frames::material(false);
    /// ```
    pub fn material(inverted: bool) -> Frames {
        let pattern = vec![
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
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        ];
        Self::generate_frames(pattern, inverted, 17)
    }

    /// # moon
    ///
    /// 🌑 🌒 🌓 🌔 🌕 🌖 🌗 🌘
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::moon(false);
    /// ```
    pub fn moon(inverted: bool) -> Frames {
        let pattern = vec!["🌑 ", "🌒 ", "🌓 ", "🌔 ", "🌕 ", "🌖 ", "🌗 ", "🌘 "];
        Self::generate_frames(pattern, inverted, 130)
    }

    /// # dots_simple1
    ///
    /// . ... .. .
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple1(false);
    /// ```
    pub fn dots_simple1(inverted: bool) -> Frames {
        let pattern = vec![".  ", ".. ", "...", " ..", "  .", "   "];
        Self::generate_frames(pattern, inverted, 260)
    }

    /// # dots_simple2
    ///
    /// . ... .. .
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple2(false);
    /// ```
    pub fn dots_simple2(inverted: bool) -> Frames {
        let pattern = vec!["   ", ".  ", ".. ", "..."];
        Self::generate_frames(pattern, inverted, 360)
    }

    /// # japanese
    ///
    /// ｦ ｧ ｨ ｩ
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::japanese(false);
    /// ```
    pub fn japanese(inverted: bool) -> Frames {
        let pattern = vec![
            "ｦ", "ｧ", "ｨ", "ｩ", "ｪ", "ｫ", "ｬ", "ｭ", "ｮ", "ｯ", "ｱ", "ｲ", "ｳ", "ｴ", "ｵ", "ｶ", "ｷ",
            "ｸ", "ｹ", "ｺ", "ｻ", "ｼ", "ｽ", "ｾ", "ｿ", "ﾀ", "ﾁ", "ﾂ", "ﾃ", "ﾄ", "ﾅ", "ﾆ", "ﾇ", "ﾈ",
            "ﾉ", "ﾊ", "ﾋ", "ﾌ", "ﾍ", "ﾎ", "ﾏ", "ﾐ", "ﾑ", "ﾒ", "ﾓ", "ﾔ", "ﾕ", "ﾖ", "ﾗ", "ﾘ", "ﾙ",
            "ﾚ", "ﾛ", "ﾜ", "ﾝ",
        ];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// # line
    ///
    /// ________  ____-___   ______-_
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::line(false);
    /// ```
    pub fn line(inverted: bool) -> Frames {
        let pattern = vec![
            "________", "-_______", "_-______", "__-_____", "___-____", "____-___", "_____-__",
            "______-_", "_______-", "________", "_______-", "______-_", "_____-__", "____-___",
            "___-____", "__-_____", "_-______", "-_______", "________",
        ];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # line2
    ///
    /// |_______  ____|___ ______-_
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::line2(false);
    /// ```
    pub fn line2(inverted: bool) -> Frames {
        let pattern = vec![
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
            "_-______",
        ];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # block
    ///
    /// ▃  █ ▁
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::block(false);
    /// ```
    pub fn block(inverted: bool) -> Frames {
        let pattern = vec![
            "▁", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▁",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # block_spinn
    ///
    /// |_______  ____|___ ______-_
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::block_spinn(false);
    /// ```
    pub fn block_spinn(inverted: bool) -> Frames {
        let pattern = vec!["▖", "▘", "▝", "▗"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # arrow_spinn
    ///
    /// ←  ↑ →
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::arrow_spinn(false);
    /// ```
    pub fn arrow_spinn(inverted: bool) -> Frames {
        let pattern = vec!["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # big_arrow_spinn
    ///
    /// ⇐  ⇖ ⇑
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::big_arrow_spinn(false);
    /// ```
    pub fn big_arrow_spinn(inverted: bool) -> Frames {
        let pattern = vec!["⇐", "⇖", "⇑", "⇗", "⇒", "⇘", "⇓", "⇙"];
        Self::generate_frames(pattern, inverted, 140)
    }

    /// # line_spinner
    ///
    /// ┤ ┘ ├
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::line_spinner(false);
    /// ```
    pub fn line_spinner(inverted: bool) -> Frames {
        let pattern = vec!["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # line_spinner_simple
    ///
    /// | / -
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::line_spinner_simple(false);
    /// ```
    pub fn line_spinner_simple(inverted: bool) -> Frames {
        let pattern = vec!["|", "/", "-", "\\"];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # corner
    ///
    /// ◢  ◣ ◤
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::corner(false);
    /// ```
    pub fn corner(inverted: bool) -> Frames {
        let pattern = vec!["◢", "◣", "◤", "◥"];
        Self::generate_frames(pattern, inverted, 160)
    }

    /// # abc
    ///
    /// a b c
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::abc(false);
    /// ```
    pub fn abc(inverted: bool) -> Frames {
        let pattern = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];
        Self::generate_frames(pattern, inverted, 150)
    }

    /// # earth
    ///
    /// 🌍 🌎 🌏
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::earth(false);
    /// ```
    pub fn earth(inverted: bool) -> Frames {
        let pattern = vec!["🌍", "🌎", "🌏"];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// # arrow_row
    ///
    /// ▹▹▹▹▹  ▹▸▹▹▹ ▹▹▸▹▹
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::arrow_row(false);
    /// ```
    pub fn arrow_row(inverted: bool) -> Frames {
        let pattern = vec!["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"];
        Self::generate_frames(pattern, inverted, 140)
    }

    /// # fractions
    ///
    /// ⅓  ⅔ ¼
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::fractions(false);
    /// ```
    pub fn fractions(inverted: bool) -> Frames {
        let pattern = vec!["½", "⅓", "⅔", "¼", "¾", "⅛", "⅜", "⅝", "⅞"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # star1
    ///
    /// ✶  ✸ ✺
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::star1(false);
    /// ```
    pub fn star1(inverted: bool) -> Frames {
        let pattern = vec!["✶", "✸", "✹", "✺", "✹", "✷"];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// # star2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::star2(false);
    /// ```
    pub fn star2(inverted: bool) -> Frames {
        let pattern = vec!["+", "x", "*"];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// # dot_bounce
    ///
    /// .  O °
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_bounce(false);
    /// ```
    pub fn dot_bounce(inverted: bool) -> Frames {
        let pattern = vec![".", "o", "O", "°", "O", "o", "."];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # flip
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::flip(false);
    /// ```
    pub fn flip(inverted: bool) -> Frames {
        let pattern = vec!["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # binary
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::binary(false);
    /// ```
    pub fn binary(inverted: bool) -> Frames {
        let pattern = vec![
            "010010", "001100", "100101", "111010", "111101", "010111", "101011", "111000",
            "110011", "110101",
        ];
        Self::generate_frames(pattern, inverted, 80)
    }

    /// # big_loading_bar
    ///
    /// ▒▒▒▒▒▒▒▒▒▒  ███▒▒▒▒▒▒▒  ██████████
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::big_loading_bar(false);
    /// ```
    pub fn big_loading_bar(inverted: bool) -> Frames {
        let pattern = vec![
            "▒▒▒▒▒▒▒▒▒▒",
            "█▒▒▒▒▒▒▒▒▒",
            "███▒▒▒▒▒▒▒",
            "█████▒▒▒▒▒",
            "███████▒▒▒",
            "██████████",
        ];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # wall_bounce
    ///
    /// ▐⠂       ▌  ▐  ⠠     ▌ ▐       ⠠▌
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wall_bounce(false);
    /// ```
    pub fn wall_bounce(inverted: bool) -> Frames {
        let pattern = vec![
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
            "▐⠠       ▌",
        ];
        Self::generate_frames(pattern, inverted, 140)
    }

    /// # wall_bounce_line
    ///
    /// ▐__/|__________▌
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wall_bounce_line(false);
    /// ```
    pub fn wall_bounce_line(inverted: bool) -> Frames {
        let pattern = vec![
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
            "▐/|____________▌",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # stack
    ///
    /// ☱ ☲ ☴
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::stack(false);
    /// ```
    pub fn stack(inverted: bool) -> Frames {
        let pattern = vec!["☱", "☲", "☴"];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// # toggle
    ///
    /// ⊶ ⊷
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle(false);
    /// ```
    pub fn toggle(inverted: bool) -> Frames {
        let pattern = vec!["⊶", "⊷"];
        Self::generate_frames(pattern, inverted, 250)
    }

    /// # toggle2
    ///
    /// ▫ ▪
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle2(false);
    /// ```
    pub fn toggle2(inverted: bool) -> Frames {
        let pattern = vec!["▫", "▪"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle3
    ///
    /// □ ■
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle3(false);
    /// ```
    pub fn toggle3(inverted: bool) -> Frames {
        let pattern = vec!["□", "■"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle4
    ///
    /// ■ □ ▪ ▫
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle4(false);
    /// ```
    pub fn toggle4(inverted: bool) -> Frames {
        let pattern = vec!["■", "□", "▪", "▫"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle5
    ///
    /// ▮ ▯
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle5(false);
    /// ```
    pub fn toggle5(inverted: bool) -> Frames {
        let pattern = vec!["▮ ", "▯ "];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle6
    ///
    /// ဝ ၀
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle6(false);
    /// ```
    pub fn toggle6(inverted: bool) -> Frames {
        let pattern = vec!["ဝ", "၀"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle7
    ///
    /// ⦾ ⦿
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle7(false);
    /// ```
    pub fn toggle7(inverted: bool) -> Frames {
        let pattern = vec!["⦾", "⦿"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle8
    ///
    /// ◍ ◌
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle8(false);
    /// ```
    pub fn toggle8(inverted: bool) -> Frames {
        let pattern = vec!["◍", "◌"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle9
    ///
    /// ◉ ◎
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle9(false);
    /// ```
    pub fn toggle9(inverted: bool) -> Frames {
        let pattern = vec!["◉", "◎"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle10
    ///
    /// ◉ ◎
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle10(false);
    /// ```
    pub fn toggle10(inverted: bool) -> Frames {
        let pattern = vec!["㊂", "㊀", "㊁"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle11
    ///
    /// ⧇ ⧆
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle11(false);
    /// ```
    pub fn toggle11(inverted: bool) -> Frames {
        let pattern = vec!["⧇", "⧆"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle12
    ///
    /// ☗ ☖
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle12(false);
    /// ```
    pub fn toggle12(inverted: bool) -> Frames {
        let pattern = vec!["☗", "☖"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # toggle13
    ///
    /// = * -
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::toggle13(false);
    /// ```
    pub fn toggle13(inverted: bool) -> Frames {
        let pattern = vec!["=", "*", "-"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # arc
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::arc(false);
    /// ```
    pub fn arc(inverted: bool) -> Frames {
        let pattern = vec!["◜", "◠", "◝", "◞", "◡", "◟"];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// # circle
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::circle(false);
    /// ```
    pub fn circle(inverted: bool) -> Frames {
        let pattern = vec!["◡", "⊙", "◠"];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// # square_corners
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::square_corners(false);
    /// ```
    pub fn square_corners(inverted: bool) -> Frames {
        let pattern = vec!["◰ ", "◳ ", "◲ ", "◱ "];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// # circle_corners
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::circle_corners(false);
    /// ```
    pub fn circle_corners(inverted: bool) -> Frames {
        let pattern = vec!["◴ ", "◷ ", "◶ ", "◵ "];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// # circle_halves
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::circle_halves(false);
    /// ```
    pub fn circle_halves(inverted: bool) -> Frames {
        let pattern = vec!["◐ ", "◓ ", "◑ ", "◒ "];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// # bouncing_ball
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::bouncing_ball(false);
    /// ```
    pub fn bouncing_ball(inverted: bool) -> Frames {
        let pattern = vec![
            "( ●    )",
            "(  ●   )",
            "(   ●  )",
            "(    ● )",
            "(     ●)",
            "(    ● )",
            "(   ●  )",
            "(  ●   )",
            "( ●    )",
            "(●     )",
        ];
        Self::generate_frames(pattern, inverted, 160)
    }

    /// # smiley
    ///
    ///😄 😝
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::smiley(false);
    /// ```
    pub fn smiley(inverted: bool) -> Frames {
        let pattern = vec!["😄 ", "😝 "];
        Self::generate_frames(pattern, inverted, 460)
    }

    /// # monkey
    ///
    /// 🙈 🙈 🙉   
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::monkey(false);
    /// ```
    pub fn monkey(inverted: bool) -> Frames {
        let pattern = vec!["🙈 ", "🙈 ", "🙉 ", "🙊 "];
        Self::generate_frames(pattern, inverted, 440)
    }

    /// # hearts
    ///
    /// 💛 💙 💜 💚 ❤️
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::hearts(false);
    /// ```
    pub fn hearts(inverted: bool) -> Frames {
        let pattern = vec!["💛 ", "💙 ", "💜 ", "💚 ", "❤️ "];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # runner
    ///
    ///🚶 🏃
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::runner(false);
    /// ```
    pub fn runner(inverted: bool) -> Frames {
        let pattern = vec!["🚶 ", "🏃 "];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # raining
    ///
    /// 🌧 🌧 🌨
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::raining(false);
    /// ```
    pub fn raining(inverted: bool) -> Frames {
        let pattern = vec!["🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌨 ", "🌧 ", "🌨 "];
        Self::generate_frames(pattern, inverted, 140)
    }

    /// # weather
    ///
    /// ☀️ ⛅️ ☀️
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::weather(false);
    /// ```
    pub fn weather(inverted: bool) -> Frames {
        let pattern = vec![
            "☀️ ", "☀️ ", "⛅️ ", "⛅️ ", "☁️ ", "☁️ ", "⛅️ ", "⛅️ ", "☀️ ", "☀️ ",
        ];
        Self::generate_frames(pattern, inverted, 440)
    }

    /// # christmas_tree
    ///
    /// 🌲 🎄
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::christmas_tree(false);
    /// ```
    pub fn christmas_tree(inverted: bool) -> Frames {
        let pattern = vec!["🌲", "🎄"];
        Self::generate_frames(pattern, inverted, 340)
    }

    /// # nade
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::nade(false);
    /// ```
    pub fn nade(inverted: bool) -> Frames {
        let pattern = vec![
            "،  ", "′  ", " ´ ", " ‾ ", "  ⸌", "  ⸊", "  |", "  ⁎", "  ⁕", " ෴ ", "  ⁓", "   ",
            "   ", "   ",
        ];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// # dots_simple_big1
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple_big1(false);
    /// ```
    pub fn dots_simple_big1(inverted: bool) -> Frames {
        let pattern = vec!["●∙∙", "∙●∙", "∙∙●"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # dots_simple_big2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple_big2(false);
    /// ```
    pub fn dots_simple_big2(inverted: bool) -> Frames {
        let pattern = vec!["∙∙∙", "●∙∙", "∙●∙", "∙∙●"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// # dots_simple_big3
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple_big3(false);
    /// ```
    pub fn dots_simple_big3(inverted: bool) -> Frames {
        let pattern = vec!["∙∙∙", "●∙∙", "●●∙", "●●●", "∙●●", "∙∙●"];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// # dots_simple_big4
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dots_simple_big4(false);
    /// ```
    pub fn dots_simple_big4(inverted: bool) -> Frames {
        let pattern = vec!["∙∙∙", "●∙∙", "●●∙", "●●●"];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// # fist_bump
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::fist_bump(false);
    /// ```
    pub fn fist_bump(inverted: bool) -> Frames {
        let pattern = vec![
            "🤜                        🤛 ",
            " 🤜                       🤛 ",
            "  🤜                     🤛  ",
            "    🤜                 🤛    ",
            "      🤜             🤛      ",
            "         🤜       🤛         ",
            "           🤜✨🤛            ",
            "         🤜      🤛          ",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # finger_dance
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::finger_dance(false);
    /// ```
    pub fn finger_dance(inverted: bool) -> Frames {
        let pattern = vec!["🤘 ", "🤟 ", "🖖 ", "✋ ", "🤚 ", "👆 "];
        Self::generate_frames(pattern, inverted, 280)
    }

    /// # mind_blown
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::mind_blown(false);
    /// ```
    pub fn mind_blown(inverted: bool) -> Frames {
        let pattern = vec![
            "😐 ", "😐 ", "😮 ", "😮 ", "😦 ", "😦 ", "😧 ", "😧 ", "🤯 ", "🤯 ", "💥 ", "💥 ",
            "✨ ",
        ];
        Self::generate_frames(pattern, inverted, 280)
    }

    /// # speaker
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::speaker(false);
    /// ```
    pub fn speaker(inverted: bool) -> Frames {
        let pattern = vec!["🔈 ", "🔉 ", "🔊 ", "🔉 "];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// # arrows
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::arrows(false);
    /// ```
    pub fn arrows(inverted: bool) -> Frames {
        let pattern = vec!["⇢", "⇨", "⇒", "⇉", "⇶"];
        Self::generate_frames(pattern, inverted, 150)
    }

    /// # dot_box
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_box(false);
    /// ```
    pub fn dot_box(inverted: bool) -> Frames {
        let pattern = vec![".", "·", "•", "¤", "°", "¤", "•", "·"];
        Self::generate_frames(pattern, inverted, 150)
    }

    /// # simple_line_spin
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::simple_line_spin(false);
    /// ```
    pub fn simple_line_spin(inverted: bool) -> Frames {
        let pattern = vec!["+", "\\", "|", "!", "/", "-", "x"];
        Self::generate_frames(pattern, inverted, 150)
    }

    /// # bomb
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::bomb(false);
    /// ```
    pub fn bomb(inverted: bool) -> Frames {
        let pattern = vec![
            "💣  ", " 💣  ", "  💣 ", "   💣", "   💣", "   💣", "   💣", "   💥", "    ", "    ",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dot_bounce2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dot_bounce2(false);
    /// ```
    pub fn dot_bounce2(inverted: bool) -> Frames {
        let pattern = vec![".", "·", "˙", "·", "."];
        Self::generate_frames(pattern, inverted, 110)
    }

    /// # orange_pulse
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::orange_pulse(false);
    /// ```
    pub fn orange_pulse(inverted: bool) -> Frames {
        let pattern = vec!["🔸", "🔶", "🟠", "🟠", "🔶"];
        Self::generate_frames(pattern, inverted, 110)
    }

    /// # blue_pulse
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::blue_pulse(false);
    /// ```
    pub fn blue_pulse(inverted: bool) -> Frames {
        let pattern = vec!["🔹", "🔷", "🔵", "🔵", "🔷"];
        Self::generate_frames(pattern, inverted, 110)
    }

    /// # green_pulse
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::green_pulse(false);
    /// ```
    pub fn green_pulse(inverted: bool) -> Frames {
        let pattern = vec!["🟢", "🟩", "🟩", "🟢"];
        Self::generate_frames(pattern, inverted, 110)
    }

    /// # red_pulse
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::red_pulse(false);
    /// ```
    pub fn red_pulse(inverted: bool) -> Frames {
        let pattern = vec!["🔴", "🟥", "🟥", "🔴"];
        Self::generate_frames(pattern, inverted, 110)
    }

    /// # other
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::other(false);
    /// ```
    pub fn other(inverted: bool) -> Frames {
        let pattern = vec!["d", "q", "p", "b"];
        Self::generate_frames(pattern, inverted, 110)
    }

    /// # pray
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::pray(false);
    /// ```
    pub fn pray(inverted: bool) -> Frames {
        let pattern = vec!["🧍 ", "🚶 ", "🧎 ", "🙇 "];
        Self::generate_frames(pattern, inverted, 210)
    }

    /// # wavy
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wavy(false);
    /// ```
    pub fn wavy(inverted: bool) -> Frames {
        let pattern = vec![
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
            "¸¸¸¸¸¸¸¸·",
        ];
        Self::generate_frames(pattern, inverted, 80)
    }

    /// # wavy2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wavy2(false);
    /// ```
    pub fn wavy2(inverted: bool) -> Frames {
        let pattern = vec![
            "¸.·´¯`·.¸",
            "¸¸.·´¯`·.",
            ".¸¸.·´¯`·",
            "·.¸¸.·´¯`",
            "`·.¸¸.·´¯",
            "¯`·.¸¸.·´",
            "´¯`·.¸¸.·",
            "·´¯`·.¸¸.",
            ".·´¯`·.¸¸",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # wavy3
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wavy3(false);
    /// ```
    pub fn wavy3(inverted: bool) -> Frames {
        let pattern = vec![
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
            "▂▃▄",
        ];
        Self::generate_frames(pattern, inverted, 40)
    }

    /// # wavy4
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::wavy4(false);
    /// ```
    pub fn wavy4(inverted: bool) -> Frames {
        let pattern = vec![
            "ρββββββ",
            "βρβββββ",
            "ββρββββ",
            "βββρβββ",
            "ββββρββ",
            "βββββρβ",
            "ββββββρ",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # soccer
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::soccer(false);
    /// ```
    pub fn soccer(inverted: bool) -> Frames {
        let pattern = vec![
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
            "🧑  ⚽️      🧑 ",
        ];
        Self::generate_frames(pattern, inverted, 80)
    }

    /// # layer
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::layer(false);
    /// ```
    pub fn layer(inverted: bool) -> Frames {
        let pattern = vec!["-", "=", "≡"];
        Self::generate_frames(pattern, inverted, 250)
    }

    /// # matrix_glitch
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::matrix_glitch(false);
    /// ```
    pub fn matrix_glitch(inverted: bool) -> Frames {
        let pattern = vec![
            "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
            "⠙⠹⠸⠼⠴⠦⠧⠇⠏⠋",
            "⠹⠸⠼⠴⠦⠧⠇⠏⠋⠙",
            "⠸⠼⠴⠦⠧⠇⠏⠋⠙⠹",
            "⠼⠴⠦⠧⠇⠏⠋⠙⠹⠸",
            "⠴⠦⠧⠇⠏⠋⠙⠹⠸⠼",
            "⠦⠧⠇⠏⠋⠙⠹⠸⠼⠴",
            "⠧⠇⠏⠋⠙⠹⠸⠼⠴⠦",
            "⠇⠏⠋⠙⠹⠸⠼⠴⠦⠧",
            "⠏⠋⠙⠹⠸⠼⠴⠦⠧⠇",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # matrix_glitch2
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::matrix_glitch2(false);
    /// ```
    pub fn matrix_glitch2(inverted: bool) -> Frames {
        let pattern = vec![
            "█▒▓░█░▒▒▓░░▓▒▓▒█▓░░▓▓░░▓░",
            "█▒▓▒█░▒▓░▓▒▓█▒░▒▓█▒░▓░▓░▓",
            "▒█░░█▒░░▓▓▒░▒▓░░▒▒█▓░▓▓░░",
            "░█░▓█░▒░▒▓░▒▓█▓░░▒░▓░▓░▓▓",
            "▒▓█▓▓▓░▒▒▒▓░▓▒▒▒▒▒▓░░▒▒▓░",
            "▓█▓█▓▒░▒▒▒▓▓▓▓░▒▒▓▒▒▓▒▓▒▒",
            "▒▓█▒▓▓░▓▓▒▓▒▒▒▓▓▒░▓░▒▒▓▓▒",
            "▓░▒▓▒▒▓▓▒░▒▓▒▒░▓▒░▒▓▓▒▒▓▓",
            "▓░░░▓▒▓░░░▒▒▓░░░░▓▓░░▓▒▒▒",
            "▓▒▓▒▓▓▒▓▒▒▓▒▓▓▓▓▒▓▒▓▓▒▒▓▒",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # matrix_glitch2_small
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::Frames;
    ///
    /// let spinner_frames: Frames = Frames::matrix_glitch2_small(false);
    /// ```
    pub fn matrix_glitch2_small(inverted: bool) -> Frames {
        let pattern = vec![
            "█▒▓░█░▒▒▓",
            "█▒▓▒█░▒▓░",
            "▒█░░█▒░░▓",
            "░█░▓█░▒░▒",
            "▒▓█▓▓▓░▒▒",
            "▓█▓█▓▒░▒▒",
            "▒▓█▒▓▓░▓▓",
            "▓░▒▓▒▒▓▓▒",
            "▓░░░▓▒▓░░",
            "▓▒▓▒▓▓▒▓▒",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// # dwarf_fortress
    ///
    ///
    /// # Example
    /// ```
    /// use zenity::spinner::{Frames};
    ///
    /// let spinner_frames: Frames = Frames::dwarf_fortress(false);
    /// ```
    pub fn dwarf_fortress(inverted: bool) -> Frames {
        let pattern = vec![
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
            " ██████£££  ",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }
}

