use std::io;

mod pckgm;

// TODO: add windows support

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
    // Check if each package is installed
    for package in packages_to_check {
        pckgm::is_package_installed(package)?
    }

    Ok(())
}
