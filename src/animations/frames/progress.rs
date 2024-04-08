
pub struct ProgressBarFrames {
    pub begin: Vec<&'static str>,
    pub bar_complete_char: Vec<&'static str>,
    pub bar_incomplete_char: Vec<&'static str>,
    pub end: Vec<&'static str>,
}

// TODO: add animations by adding +1 for each bar so you can have a wave animation and others

impl ProgressBarFrames {
    
    /// '=' as the complete char and '-' as the incomplete char
    pub fn equal() -> Self {
        Self {
            begin: vec!["["],
            bar_complete_char: vec!["="],
            bar_incomplete_char: vec!["-"],
            end: vec!["]"]
        }
    }

    /// '#' as the complete char and '.' as the incomplete char
    pub fn hash() -> Self {
        Self {
            begin: vec!["["],
            bar_complete_char: vec!["#"],
            bar_incomplete_char: vec!["."],
            end: vec!["]"]
        }
    }
    /// '#' as the complete char and '.' as the incomplete char
    pub fn rect() -> Self {
        Self {
            begin: vec![" "],
            bar_complete_char: vec!["\u{25A0}"],
            bar_incomplete_char: vec![" "],
            end: vec![" "]
        }
    }

    // Add more default settings functions as needed
}