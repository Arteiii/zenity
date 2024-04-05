pub struct Frames {
    pub frames: Vec<&'static str>,
    pub speed_ms: u64,
}

pub struct PreDefined;

impl PreDefined {
    fn generate_frames(pattern: Vec<&'static str>, inverted: bool, speed_ms: u64) -> Frames {
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
    /// if `inverted` is set to true, the direction of rotation will be reversed
    pub fn dot_spinner1(inverted: bool) -> Frames {
        let pattern = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        Self::generate_frames(pattern, inverted, 100)
    }

    pub fn dot_spinner2(inverted: bool) -> Frames {
        let pattern = vec!["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"];
        Self::generate_frames(pattern, inverted, 100)
    }

    pub fn dot_spinner3(inverted: bool) -> Frames {
        let pattern = vec!["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"];
        Self::generate_frames(pattern, inverted, 100)
    }

    pub fn dot_spinner4(inverted: bool) -> Frames {
        let pattern = vec![
            "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆",
        ];
        Self::generate_frames(pattern, inverted, 120)
    }
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

    pub fn dot_spinner7(inverted: bool) -> Frames {
        let pattern = vec!["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"];
        Self::generate_frames(pattern, inverted, 100)
    }

    pub fn dot_spinner8(inverted: bool) -> Frames {
        let pattern = vec![
            "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦", "⣮",
            "⣶", "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉", "⠑", "⠡",
            "⢁",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }
    pub fn dot_spinner9(inverted: bool) -> Frames {
        let pattern = vec![
            "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁",
            "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘",
            "⢃⠨", "⡃⢐", "⠍⡐", "⢋⠠", "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩",
            "⠈⢙", "⠈⡙", "⠈⠩", "⠀⢙", "⠀⡙", "⠀⠩", "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    pub fn dot_spinner10(inverted: bool) -> Frames {
        let pattern = vec!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
        Self::generate_frames(pattern, inverted, 100)
    }

    pub fn dot_spinner11(inverted: bool) -> Frames {
        let pattern = vec!["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// (　´･ω)
    /// (´･ω･`)
    /// (ω･`　)
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

    /// ▰▱▱▱▱▱▱
    /// ▰▰▰▰▱▱▱
    /// ▰▰▰▰▰▰▰
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

    /// 🕛 🕘 🕔
    pub fn clock(inverted: bool) -> Frames {
        let pattern = vec![
            "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 ",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// [=   ]  [ ===]  [==  ]
    pub fn bouncing_bar(inverted: bool) -> Frames {
        let pattern = vec![
            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
            "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
        ];
        Self::generate_frames(pattern, inverted, 80)
    }

    /// [=   ]  [ ===]  [==  ]
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
    /// [=   ]  [ ===]  [==  ]
    pub fn short_loading_bar_with_arrow(inverted: bool) -> Frames {
        let pattern = vec![
            "[>        >>>]",
            "[>>        >>]",
            "[>>>        >]",
            "[>>>>        ]",
            "[]>>>>      []",
            "[] >>>>     []",
            "[]  >>>>    []",
            "[]   >>>>   []",
            "[]    >>>>  []",
            "[]     >>>> []",
            "[]      >>>>[]",
            "[]       >>>>]",
        ];
        Self::generate_frames(pattern, inverted, 130)
    }

    /// ████████████▁▁▁▁▁▁▁▁
    /// ▁▁▁▁▁▁▁▁▁█████████▁▁
    /// ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█
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

    /// 🌑 🌒 🌓 🌔 🌕 🌖 🌗 🌘
    pub fn moon(inverted: bool) -> Frames {
        let pattern = vec!["🌑 ", "🌒 ", "🌓 ", "🌔 ", "🌕 ", "🌖 ", "🌗 ", "🌘 "];
        Self::generate_frames(pattern, inverted, 130)
    }

    /// .  .. ... .. .
    pub fn dots_simple1(inverted: bool) -> Frames {
        let pattern = vec![".  ", ".. ", "...", " ..", "  .", "   "];
        Self::generate_frames(pattern, inverted, 260)
    }

    /// .  .. ... .. .
    pub fn dots_simple2(inverted: bool) -> Frames {
        let pattern = vec!["   ", ".  ", ".. ", "..."];
        Self::generate_frames(pattern, inverted, 360)
    }

    /// ｦ ｧ ｨ ｩ
    pub fn japanese(inverted: bool) -> Frames {
        let pattern = vec![
            "ｦ", "ｧ", "ｨ", "ｩ", "ｪ", "ｫ", "ｬ", "ｭ", "ｮ", "ｯ", "ｱ", "ｲ", "ｳ", "ｴ", "ｵ", "ｶ", "ｷ",
            "ｸ", "ｹ", "ｺ", "ｻ", "ｼ", "ｽ", "ｾ", "ｿ", "ﾀ", "ﾁ", "ﾂ", "ﾃ", "ﾄ", "ﾅ", "ﾆ", "ﾇ", "ﾈ",
            "ﾉ", "ﾊ", "ﾋ", "ﾌ", "ﾍ", "ﾎ", "ﾏ", "ﾐ", "ﾑ", "ﾒ", "ﾓ", "ﾔ", "ﾕ", "ﾖ", "ﾗ", "ﾘ", "ﾙ",
            "ﾚ", "ﾛ", "ﾜ", "ﾝ",
        ];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// ________  ____-___   ______-_
    pub fn line(inverted: bool) -> Frames {
        let pattern = vec![
            "________", "-_______", "_-______", "__-_____", "___-____", "____-___", "_____-__",
            "______-_", "_______-", "________", "_______-", "______-_", "_____-__", "____-___",
            "___-____", "__-_____", "_-______", "-_______", "________",
        ];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// |_______  ____|___ ______-_
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

    /// ▃  █ ▁
    pub fn block(inverted: bool) -> Frames {
        let pattern = vec![
            "▁", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▁",
        ];
        Self::generate_frames(pattern, inverted, 100)
    }
    /// |_______  ____|___ ______-_
    pub fn block_spinn(inverted: bool) -> Frames {
        let pattern = vec!["▖", "▘", "▝", "▗"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// ←  ↑ →
    pub fn arrow_spinn(inverted: bool) -> Frames {
        let pattern = vec!["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// ⇐  ⇖ ⇑
    pub fn big_arrow_spinn(inverted: bool) -> Frames {
        let pattern = vec!["⇐", "⇖", "⇑", "⇗", "⇒", "⇘", "⇓", "⇙"];
        Self::generate_frames(pattern, inverted, 140)
    }

    /// ┤ ┘ ├
    pub fn line_spinner(inverted: bool) -> Frames {
        let pattern = vec!["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// | / -
    pub fn line_spinner_simple(inverted: bool) -> Frames {
        let pattern = vec!["|", "/", "-", "\\"];
        Self::generate_frames(pattern, inverted, 120)
    }

    /// ◢  ◣ ◤
    pub fn corner(inverted: bool) -> Frames {
        let pattern = vec!["◢", "◣", "◤", "◥"];
        Self::generate_frames(pattern, inverted, 160)
    }

    /// ⠁ ⠂ ⠄ ⡀ ⢀ ⠠ ⠐ ⠈
    pub fn dot_spinner6(inverted: bool) -> Frames {
        let pattern = vec!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// a b c
    pub fn abc(inverted: bool) -> Frames {
        let pattern = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];
        Self::generate_frames(pattern, inverted, 150)
    }

    /// 🌍 🌎 🌏
    pub fn earth(inverted: bool) -> Frames {
        let pattern = vec!["🌍", "🌎", "🌏"];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// ▹▹▹▹▹  ▹▸▹▹▹ ▹▹▸▹▹
    pub fn arrow_row(inverted: bool) -> Frames {
        let pattern = vec!["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"];
        Self::generate_frames(pattern, inverted, 140)
    }

    /// ⅓  ⅔ ¼
    pub fn fractions(inverted: bool) -> Frames {
        let pattern = vec!["½", "⅓", "⅔", "¼", "¾", "⅛", "⅜", "⅝", "⅞"];
        Self::generate_frames(pattern, inverted, 100)
    }

    /// ✶  ✸ ✺
    pub fn star1(inverted: bool) -> Frames {
        let pattern = vec!["✶", "✸", "✹", "✺", "✹", "✷"];
        Self::generate_frames(pattern, inverted, 180)
    }

    pub fn star2(inverted: bool) -> Frames {
        let pattern = vec!["+", "x", "*"];
        Self::generate_frames(pattern, inverted, 180)
    }

    /// .  O °
    pub fn dot_bounce(inverted: bool) -> Frames {
        let pattern = vec![".", "o", "O", "°", "O", "o", "."];
        Self::generate_frames(pattern, inverted, 120)
    }

    pub fn flip(inverted: bool) -> Frames {
        let pattern = vec!["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"];
        Self::generate_frames(pattern, inverted, 120)
    }

    pub fn binary(inverted: bool) -> Frames {
        let pattern = vec![
            "010010", "001100", "100101", "111010", "111101", "010111", "101011", "111000",
            "110011", "110101",
        ];
        Self::generate_frames(pattern, inverted, 80)
    }

    /// ▒▒▒▒▒▒▒▒▒▒  ███▒▒▒▒▒▒▒  ██████████
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

    /// ▐⠂       ▌  ▐  ⠠     ▌ ▐       ⠠▌
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

    /// ▐__/|__________▌
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

    /// ☱ ☲ ☴
    pub fn stack(inverted: bool) -> Frames {
        let pattern = vec!["☱", "☲", "☴"];
        Self::generate_frames(pattern, inverted, 200)
    }

    /// ⊶ ⊷
    pub fn toggle(inverted: bool) -> Frames {
        let pattern = vec!["⊶", "⊷"];
        Self::generate_frames(pattern, inverted, 250)
    }

    /// ▫ ▪
    pub fn toggle2(inverted: bool) -> Frames {
        let pattern = vec!["▫", "▪"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// □ ■
    pub fn toggle3(inverted: bool) -> Frames {
        let pattern = vec!["□", "■"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ■ □ ▪ ▫
    pub fn toggle4(inverted: bool) -> Frames {
        let pattern = vec!["■", "□", "▪", "▫"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ▮ ▯
    pub fn toggle5(inverted: bool) -> Frames {
        let pattern = vec!["▮ ", "▯ "];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ဝ ၀
    pub fn toggle6(inverted: bool) -> Frames {
        let pattern = vec!["ဝ", "၀"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ⦾ ⦿
    pub fn toggle7(inverted: bool) -> Frames {
        let pattern = vec!["⦾", "⦿"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ◍ ◌
    pub fn toggle8(inverted: bool) -> Frames {
        let pattern = vec!["◍", "◌"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ◉ ◎
    pub fn toggle9(inverted: bool) -> Frames {
        let pattern = vec!["◉", "◎"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ㊂ ㊀ ㊁
    pub fn toggle10(inverted: bool) -> Frames {
        let pattern = vec!["㊂", "㊀", "㊁"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ⧇ ⧆
    pub fn toggle11(inverted: bool) -> Frames {
        let pattern = vec!["⧇", "⧆"];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// ☗ ☖
    pub fn toggle12(inverted: bool) -> Frames {
        let pattern = vec!["☗", "☖"];
        Self::generate_frames(pattern, inverted, 240)
    }
    /// = * -
    pub fn toggle13(inverted: bool) -> Frames {
        let pattern = vec!["=", "*", "-"];
        Self::generate_frames(pattern, inverted, 240)
    }

    pub fn arc(inverted: bool) -> Frames {
        let pattern = vec!["◜", "◠", "◝", "◞", "◡", "◟"];
        Self::generate_frames(pattern, inverted, 120)
    }
    pub fn circle(inverted: bool) -> Frames {
        let pattern = vec!["◡", "⊙", "◠"];
        Self::generate_frames(pattern, inverted, 200)
    }
    pub fn square_corners(inverted: bool) -> Frames {
        let pattern = vec!["◰ ", "◳ ", "◲ ", "◱ "];
        Self::generate_frames(pattern, inverted, 200)
    }
    pub fn circle_corners(inverted: bool) -> Frames {
        let pattern = vec!["◴ ", "◷ ", "◶ ", "◵ "];
        Self::generate_frames(pattern, inverted, 200)
    }
    pub fn circle_halves(inverted: bool) -> Frames {
        let pattern = vec!["◐ ", "◓ ", "◑ ", "◒ "];
        Self::generate_frames(pattern, inverted, 200)
    }

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

    ///😄 😝
    pub fn smiley(inverted: bool) -> Frames {
        let pattern = vec!["😄 ", "😝 "];
        Self::generate_frames(pattern, inverted, 460)
    }

    /// 🙈 🙈 🙉   
    pub fn monkey(inverted: bool) -> Frames {
        let pattern = vec!["🙈 ", "🙈 ", "🙉 ", "🙊 "];
        Self::generate_frames(pattern, inverted, 440)
    }

    /// 💛 💙 💜 💚 ❤️
    pub fn hearts(inverted: bool) -> Frames {
        let pattern = vec!["💛 ", "💙 ", "💜 ", "💚 ", "❤️ "];
        Self::generate_frames(pattern, inverted, 240)
    }

    ///🚶 🏃
    pub fn runner(inverted: bool) -> Frames {
        let pattern = vec!["🚶 ", "🏃 "];
        Self::generate_frames(pattern, inverted, 240)
    }

    /// 🌧 🌧 🌨
    pub fn raining(inverted: bool) -> Frames {
        let pattern = vec!["🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌨 ", "🌧 ", "🌨 "];
        Self::generate_frames(pattern, inverted, 140)
    }

    /// ☀️ ⛅️ ☀️
    pub fn weather(inverted: bool) -> Frames {
        let pattern = vec![
            "☀️ ", "☀️ ", "⛅️ ", "⛅️ ", "☁️ ", "☁️ ", "⛅️ ", "⛅️ ", "☀️ ", "☀️ ",
        ];
        Self::generate_frames(pattern, inverted, 440)
    }
    /// 🌲 🎄
    pub fn christmas_tree(inverted: bool) -> Frames {
        let pattern = vec!["🌲", "🎄"];
        Self::generate_frames(pattern, inverted, 340)
    }

    pub fn nade(inverted: bool) -> Frames {
        let pattern = vec![
            "،  ", "′  ", " ´ ", " ‾ ", "  ⸌", "  ⸊", "  |", "  ⁎", "  ⁕", " ෴ ", "  ⁓", "   ",
            "   ", "   ",
        ];
        Self::generate_frames(pattern, inverted, 180)
    }

    pub fn dots_simple_big1(inverted: bool) -> Frames {
        let pattern = vec!["●∙∙", "∙●∙", "∙∙●"];
        Self::generate_frames(pattern, inverted, 240)
    }
    pub fn dots_simple_big2(inverted: bool) -> Frames {
        let pattern = vec!["∙∙∙", "●∙∙", "∙●∙", "∙∙●"];
        Self::generate_frames(pattern, inverted, 240)
    }
    pub fn dots_simple_big3(inverted: bool) -> Frames {
        let pattern = vec!["∙∙∙", "●∙∙", "●●∙", "●●●", "∙●●", "∙∙●"];
        Self::generate_frames(pattern, inverted, 180)
    }
    pub fn dots_simple_big4(inverted: bool) -> Frames {
        let pattern = vec!["∙∙∙", "●∙∙", "●●∙", "●●●"];
        Self::generate_frames(pattern, inverted, 180)
    }
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
    pub fn finger_dance(inverted: bool) -> Frames {
        let pattern = vec!["🤘 ", "🤟 ", "🖖 ", "✋ ", "🤚 ", "👆 "];
        Self::generate_frames(pattern, inverted, 280)
    }
    pub fn mind_blown(inverted: bool) -> Frames {
        let pattern = vec![
            "😐 ", "😐 ", "😮 ", "😮 ", "😦 ", "😦 ", "😧 ", "😧 ", "🤯 ", "🤯 ","💥 ", "💥 ","✨ ",
        ];
        Self::generate_frames(pattern, inverted, 280)
    }

    pub fn speaker(inverted: bool) -> Frames {
        let pattern = vec!["🔈 ", "🔉 ", "🔊 ", "🔉 "];
        Self::generate_frames(pattern, inverted, 200)
    }
    
}
