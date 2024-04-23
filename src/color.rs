//! Mod for color related methods
//! Example:
//!
//! ```
//! use zenity::color::{COLOR_PALETTE, ENABLE_COLOR, ColorPalette};
//! assert_eq!(*ENABLE_COLOR, false); // if color is supported (will be false in default tests)
//! assert_eq!(*COLOR_PALETTE, ColorPalette::None); // which colors are supported
//! ```

use lazy_static::lazy_static;
use supports_color::Stream;

lazy_static! {
   /// Supported color pallet (which colors are supported if ENABLE_COLOR)
   ///
   /// Example:
   ///
   /// ```
   /// use zenity::color::{COLOR_PALETTE, ColorPalette};
   /// assert_eq!(*COLOR_PALETTE, ColorPalette::None); // which colors are supported
   /// ```
    pub static ref COLOR_PALETTE: ColorPalette = {
        CliColorConfig::get_supported_color_palette(Stream::Stdout)
    };

    /// Lazy static ENABLE color bool true if color should be enabled false otherwise
    ///
    /// Example:
    ///
    /// ```
    /// use zenity::color::ENABLE_COLOR;
    /// assert_eq!(*ENABLE_COLOR, false); // if color is supported
    /// ```
    pub static ref ENABLE_COLOR: bool = {
        let conf = CliColorConfig::default();
        conf.should_enable_color()
    };
}

/// represents different color palettes supported by terminals
/// Example:
///
/// ```
/// use zenity::color::{COLOR_PALETTE, ENABLE_COLOR, ColorPalette};
/// # assert_eq!(*ENABLE_COLOR, false);
/// assert_eq!(*COLOR_PALETTE, ColorPalette::None); // None for testing
/// ```
#[derive(PartialEq, Debug)]
pub enum ColorPalette {
    /// color support is not available (pipe or otherwise disabled)
    None,

    /// colors 16: 4-bit color (black, red, green, yellow, blue, magenta, cyan, white, and a "bright" version of each)
    Palette16,

    /// colors 256: 8-bit color the 16 colors from Palette16, a 6×6×6 cube for each of red, green and blue,
    /// and 24 grayscale tones.
    /// for more information, see:
    /// [256-Color Palette](https://www.pixelbeat.org/docs/terminal_colours/#256)
    Palette256,

    /// Truecolor (16 million colors): 24-bit color; eight bits for each of red, green and blue
    /// This is the standard that web pages and most monitors support
    Truecolor,
}

/// Represents different options for controlling color output in the cli
///
/// - Always: always enable color output
/// - Auto: automatically determine whether to enable color output based on the terminal type and capabilities
/// - Never: never enable color output
///
/// [Read More](https://rust-cli-recommendations.sunshowers.io/colors.html#general-recommendations)
#[derive(Debug, PartialEq)]
enum ColorOption {
    /// always enable color output
    Always,

    /// automatically determine whether to enable color output based on the terminal type and capabilities
    Auto,

    /// never enable color output
    Never,
}

/// configuration struct for managing cli color settings
struct CliColorConfig {
    /// the chosen color option for cli output
    color_option: ColorOption,

    /// the color palette supported by the terminal
    color_palette: ColorPalette,
}

impl Default for CliColorConfig {
    /// creates a new `CliColorConfig` instance with default settings
    ///
    /// this function parses command-line arguments to determine the color option,
    /// and checks the terminal capabilities to determine the supported color palette
    fn default() -> Self {
        let args: Vec<String> = std::env::args().collect();

        let color_option = CliColorConfig::parse_arguments(&args);

        let color_palette = CliColorConfig::get_supported_color_palette(Stream::Stdout);

        CliColorConfig::new(color_option, color_palette)
    }
}

impl CliColorConfig {
    /// creates a new `CliColorConfig` instance with custom settings
    ///
    /// # Arguments
    ///
    /// * `color_option` - the chosen color option for CLI output
    /// * `color_palette` - the color palette supported by the terminal
    fn new(color_option: ColorOption, color_palette: ColorPalette) -> Self {
        Self {
            color_option,
            color_palette,
        }
    }

    /// determines whether color output should be enabled based on the configured settings
    ///
    /// # Returns
    ///
    /// * `true` if color output should be enabled, `false` otherwise
    pub fn should_enable_color(&self) -> bool {
        match self.color_option {
            ColorOption::Never => false,
            ColorOption::Always => true,
            ColorOption::Auto => self.color_palette != ColorPalette::None,
        }
    }

    /// parse args to check for --color=always|auto|never
    fn parse_arguments(args: &[String]) -> ColorOption {
        if args.len() > 1 {
            let arg = &args[1];
            return match arg.as_str() {
                "--color=always" => ColorOption::Always,
                "--color=never" => ColorOption::Never,
                _ => {
                    // removed error message print
                    ColorOption::Auto
                }
            };
        }

        ColorOption::Auto
    }

    /// determine the supported color palette based on the terminal capabilities
    fn get_supported_color_palette(stream: Stream) -> ColorPalette {
        match supports_color::on(stream) {
            Some(support) => {
                if support.has_16m {
                    ColorPalette::Truecolor
                } else if support.has_256 {
                    ColorPalette::Palette256
                } else {
                    ColorPalette::Palette16
                }
            }
            None => {
                ColorPalette::None // default to None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::style::*;

    use super::*;

    // Mock Stream enum for testing purposes
    #[derive(Debug, PartialEq)]
    enum MockStream {
        Supports16m,
        Supports256,
        Supports16,
        Unknown,
    }

    #[test]
    fn test_new() {
        let config = CliColorConfig::new(ColorOption::Always, ColorPalette::Palette256);

        assert_eq!(config.color_option, ColorOption::Always);
        assert_eq!(config.color_palette, ColorPalette::Palette256);
    }

    #[test]
    fn test_should_enable_color_never() {
        let settings = CliColorConfig::new(ColorOption::Never, ColorPalette::Palette256);
        assert!(!settings.should_enable_color());
    }

    #[test]
    fn test_should_enable_color_always() {
        let settings = CliColorConfig::new(ColorOption::Always, ColorPalette::None);
        assert!(settings.should_enable_color());
    }
    #[test]
    fn test_should_enable_color_auto_with_palette() {
        let settings = CliColorConfig {
            color_option: ColorOption::Auto,
            color_palette: ColorPalette::Palette16,
        };
        assert!(settings.should_enable_color());
    }

    #[test]
    fn test_should_enable_color_auto_without_palette() {
        let settings = CliColorConfig {
            color_option: ColorOption::Auto,
            color_palette: ColorPalette::None,
        };
        assert!(!settings.should_enable_color());
    }

    fn get_supported_color_palette_mock(stream: MockStream) -> ColorPalette {
        match stream {
            MockStream::Supports16m => ColorPalette::Truecolor,
            MockStream::Supports256 => ColorPalette::Palette256,
            MockStream::Supports16 => ColorPalette::Palette16,
            MockStream::Unknown => ColorPalette::None,
        }
    }

    #[test]
    fn test_get_supported_color_palette_truecolor() {
        let result = get_supported_color_palette_mock(MockStream::Supports16m);
        assert_eq!(result, ColorPalette::Truecolor);
    }

    #[test]
    fn test_get_supported_color_palette_palette256() {
        let result = get_supported_color_palette_mock(MockStream::Supports256);
        assert_eq!(result, ColorPalette::Palette256);
    }

    #[test]
    fn test_get_supported_color_palette_palette16() {
        let result = get_supported_color_palette_mock(MockStream::Supports16);
        assert_eq!(result, ColorPalette::Palette16);
    }

    #[test]
    fn test_get_supported_color_palette_none() {
        let result = get_supported_color_palette_mock(MockStream::Unknown);
        assert_eq!(result, ColorPalette::None);
    }

    #[test]
    fn test_valid_arguments() {
        let args = vec!["my_program".to_string(), "--color=always".to_string()];
        let config = CliColorConfig::parse_arguments(&args);

        assert_eq!(config, ColorOption::Always);
    }

    #[test]
    fn test_invalid_arguments() {
        let args = vec!["my_program".to_string(), "--invalid-option".to_string()];
        let result = CliColorConfig::parse_arguments(&args);

        assert_eq!(result, ColorOption::Auto);
    }

    #[test]
    fn test_combine_attributes() {
        let attributes =
            combine_attributes(&[&Attribute::Bold, &Attribute::Underlined, &Attribute::Italic]);
        assert_eq!(
            attributes,
            Attributes::default() | Attribute::Bold | Attribute::Underlined | Attribute::Italic
        );
    }
}
