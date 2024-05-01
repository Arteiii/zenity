use std::time::Duration;
use std::{io, thread};

use crossterm::style::Color;

use crate::spinner;
use crate::spinner::Frames;
use crate::style::StyledString;

mod pckgm;

// TODO: add windows support

struct LoadingReq {
    spinner_id: usize,
    name: String,
}

/// Verifies that required packages are installed.
///
/// This function checks if certain packages are installed on the system.
///
/// # Examples
///
/// ```
/// use zenity::menu::requirements::verify_requirements;
///
/// match verify_requirements(vec!["uidmap", "bridge-utils"]) {
///     Ok(_) => println!("All required packages are installed."),
///     Err(err) => eprintln!("Error verifying requirements: {}", err),
/// }
/// ```
pub fn verify_requirements(packages_to_check: Vec<&str>) -> Result<(), io::Error> {
    let mut reqs = Vec::new();
    let spinner = spinner::MultiSpinner::new();

    for package in packages_to_check {
        let spinner_id = spinner.add(Frames::dot_spinner1());
        spinner.set_styled_text(
            &spinner_id,
            StyledString::simple(package, Some(Color::Yellow), None, None),
        );

        reqs.push(LoadingReq {
            spinner_id,
            name: package.to_string(),
        });
    }

    spinner.run_all();

    // Check if each package is installed
    for package in reqs {
        spinner.set_styled_text(
            &package.spinner_id,
            StyledString::simple(
                &format!("Checking if {} is installed...", &package.name),
                Some(Color::Yellow),
                None,
                None,
            ),
        );

        let (is_installed, msg) = match pckgm::is_package_installed(&package.name) {
            Ok(_) => (true, format!("{} is installed\n", &package.name)),
            Err(err) => (false, format!("{}", err)),
        };

        if is_installed {
            spinner.set_styled_text(
                &package.spinner_id,
                StyledString::simple(&msg, Some(Color::Green), None, None),
            )
        } else {
            spinner.set_styled_text(
                &package.spinner_id,
                StyledString::simple(&msg, Some(Color::Red), None, None),
            )
        }
        spinner.stop(&package.spinner_id);

        thread::sleep(Duration::from_secs(3));
    }

    Ok(())
}
