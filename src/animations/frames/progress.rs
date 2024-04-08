use lazy_static::lazy_static;

/// Represents a collection of frames and their display speed, typically used for animations.
pub struct Frames {
    /// The sequence of frames to be displayed.
    pub frames: Vec<&'static str>,
}

/// Provides predefined spinner animations.
pub struct PreDefined;

impl PreDefined {

    /// creates a predefined progress bar animation
    ///
    /// # Returns
    ///
    /// A `Frames` struct containing the predefined progress bar frames
    pub fn progress_bar() -> Frames {
        lazy_static! {
            static ref PROGRESS_BAR_FRAMES: Vec<&'static str> = vec![
                "▏", "▎", "▍", "▌", "▋", "▊", "▉", "█", "▉", "▊", "▋", "▌", "▍", "▎"
            ];
        }

        Frames {
            frames: PROGRESS_BAR_FRAMES.iter().cloned().collect(),
        }
    }
    
    // TODO: add more animations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        let progress_bar_frames = PreDefined::progress_bar();
        assert_eq!(progress_bar_frames.frames.len(), 14);
    }
}
