#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// #![feature(rustdoc_missing_doc_code_examples)]
// #![warn(rustdoc::missing_doc_code_examples)]

//! # Yet Another Spinner Lib
//!
//! ## How to Use?
//!
//! It's as easy as pie (or maybe even easier, depending on your pie-making skills)! Just follow these simple steps:
//!
//! ## Spinner
//!
//! ```rust
//! // example/multi_spinner.rs
//! # use std::thread::sleep;
//! # use std::time::Duration;
//! use zenity::spinner::{MultiSpinner, Frames};
//!
//! let spinner = MultiSpinner::default();
//! # sleep(Duration::from_secs(2));
//!
//!  // do work...
//!
//! spinner.stop(&spinner.get_last());
//! spinner.set_text(&spinner.get_last(), "Successfully".to_string());
//! ```
//!
//! ## Progress Bar
//!```
//! // examples/progress.rs
//! use zenity::progress::ProgressBar;
//!
//! let progress = ProgressBar::default();
//!
//! let loading = 1_usize;
//! for loading in loading..=100 {
//!   // do work...
//!
//!   progress.set(&progress.get_last(), &loading);
//! }
//! ```
//! # Color Configuration
//!
//! To configure the color output, you can use the `--color` option with one of the following values:
//!
//! - `always`: Enable color output regardless of the terminal type and capabilities
//! - `auto`: Automatically determine whether to enable color output based on the terminal type and capabilities. If stdout is a pipe or if the terminal doesn't support colors, colors will be disabled
//! - `never`: Disable color output
//!
//! **Note**: If the stdout is a pipe or if the terminal doesn't support colors, colors will be automatically disabled
//!
//! Check out the examples for more
//!
//! ## Feature Requests and Bug Reports
//!
//! If you have any ideas for new features or encounter any bugs while using Zenity, please don't hesitate to open an issue on [GitHub](https://github.com/Arteiii/zenity/issues).
//! Your feedback is valuable and will help improve the library for everyone.

pub mod color;
pub(crate) mod iterators;
pub mod progress;
pub mod spinner;
pub mod style;
pub(crate) mod terminal;
