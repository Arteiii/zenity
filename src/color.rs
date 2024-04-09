//! Mod for color related methods

use lazy_static::lazy_static;


lazy_static! {
    /// supported color pallet (which colors are supported if ENABLE_COLOR)
    pub static ref COLOR_PALETTE: ColorPalette = {
        CliColorConfig::get_supported_color_palette()
    };
    
    /// lazy static ENABLE color bool true if color should be enabled false otherwise
    pub static ref ENABLE_COLOR: bool = {
        let conf = CliColorConfig::default();
        conf.should_enable_color()
    };
}

/// represents different color palettes supported by terminals
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum ColorPalette {
    /// color support not available (pipe or otherwise disabled)
    None,

    /// 16 colors: 4-bit color (black, red, green, yellow, blue, magenta, cyan, white, and a "bright" version of each)
    Palette16,

    /// 256 colors: 8-bit color the 16 colors from Palette16, a 6×6×6 cube for each of red, green and blue,
    /// and 24 grayscale tones. For more information, see:  [256-Color Palette](https://www.pixelbeat.org/docs/terminal_colours/#256)
    Palette256,

    /// Truecolor (16 million colors): 24-bit color; 8 bits for each of red, green and blue
    /// This is the standard that web pages and most monitors support
    Truecolor,
}

/// represents different options for controlling color output in the cli
///
/// - Always: always enable color output
/// - Auto: automatically determine whether to enable color output based on the terminal type and capabilities
/// - Never: never enable color output
///
/// [Read More](https://rust-cli-recommendations.sunshowers.io/colors.html#general-recommendations)
#[allow(dead_code)]
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
#[allow(dead_code)]
struct CliColorConfig {
    /// the chosen color option for cli output
    #[allow(dead_code)]
    color_option: ColorOption,

    /// the color palette supported by the terminal
    color_palette: ColorPalette,
}

#[allow(dead_code)]
impl Default for CliColorConfig {
    /// creates a new `CliColorConfig` instance with default settings
    ///
    /// this function parses command-line arguments to determine the color option,
    /// and checks the terminal capabilities to determine the supported color palette
    fn default() -> Self {
        let args: Vec<String> = std::env::args().collect();

        let color_option = CliColorConfig::parse_arguments(&args);

        let color_palette = CliColorConfig::get_supported_color_palette();

        Self {
            color_option,
            color_palette,
        }
    }
}
#[allow(dead_code)]
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

    /// retrieves the supported colors for the configured color palette
    ///
    /// # Returns
    ///
    /// * the color palette enum representing the supported colors
    fn supported_colors(&self) -> &ColorPalette {
        &self.color_palette
    }

    /// parse args to check for --color=always|auto|never
    fn parse_arguments(args: &[String]) -> ColorOption {
        if args.len() > 1 {
            let arg = &args[1];
            return match arg.as_str() {
                "--color=always" => ColorOption::Always,
                "--color=never" => ColorOption::Never,
                _ => {
                    eprintln!("Invalid argument: {}", arg);
                    eprintln!("Usage: my_program [--color=always|auto|never]");
                    eprintln!("Using Default: auto");

                    ColorOption::Auto
                }
            };
        }

        ColorOption::Auto
    }

    /// determine the supported color palette based on the terminal capabilities
    fn get_supported_color_palette() -> ColorPalette {
        match supports_color::on(supports_color::Stream::Stdout) {
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
    use super::*;
    use crate::style::*;

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
