// define a struct to hold frames, speed, and current status for a single animation
pub(crate) struct Frames {
    pub(crate) frames: &'static [&'static str],
    pub(crate) speed_ms: u64,
    pub(crate) current_status: u8,
}

pub(crate) struct ProgressBar {
    pub(crate) progress_bar1: Frames,
}

impl ProgressBar {
    pub(crate) fn new() -> Self {
        Self {
            progress_bar1: Frames {
                frames: &["▁", "▃", "▄", "▅", "▆", "▇", "█"],
                speed_ms: 50,
                current_status: 0,
            },
        }
    }
}

// TODO: handle current status updates min 0 max 100
