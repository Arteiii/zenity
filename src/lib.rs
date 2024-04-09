#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! # Yet Another Spinner Lib
//!
//! ## Changes
//! 
//! **CHANGELOG.md**
//! 
//! ## How to Use?
//!
//! It's as easy as pie (or maybe even easier, depending on your pie-making skills)! Just follow these simple steps:
//!
//! ## Spinner
//! 
//! ```rust
//! // example/multi_spinner.rs
//! 
//! # use std::thread::sleep;
//! # use std::time::Duration;
//!
//! use zenity::spinner::MultiSpinner;
//! use zenity::spinner::PreDefined;
//! 
//! fn main() {
//!         let mut spinner = MultiSpinner::default();
//!         let spinner1 = spinner.add(PreDefined::dot_spinner11(false));
//!         let spinner2 = spinner.add(PreDefined::binary(false));
//!
//!         // access the spinner through the Arc<Mutex<MultiSpinner>> reference
//!         spinner.run_all();
//! 
//!         spinner.set_text(&spinner2, "spinner2".to_string());
//! 
//!         sleep(Duration::from_secs(2));
//! 
//!         spinner.stop(&spinner2);
//!         spinner.set_text(&spinner2, "Successfully".to_string());
//! 
//!         sleep(Duration::from_secs(2));
//!         spinner.set_text(&spinner1, "spinner1 stopped".to_string());
//!}
//! ```
//! 
//! ## Progress Bar
//!```
//! // examples/progress.rs
//! # use rand::Rng;
//! # use std::thread;
//! # use std::time::Duration;
//! use zenity::progress::{Bar, ProgressBar, ProgressBarFrames};
//!
//! fn main() {
//!     let progress = ProgressBar::default();
//!     # let loading = 1_usize;
//!
//!     for loading in loading..=100 {
//!         # thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(1..=70)));
//!         // do work...
//!         progress.set(&progress.get_last(), &loading);
//!     }
//!     
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

pub(crate) mod terminal;
pub mod progress;
pub mod style;
mod iterators;
mod color;
pub mod spinner;
