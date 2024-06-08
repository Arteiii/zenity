use std::io;
use std::process::Command;

use lazy_static::lazy_static;

use crate::menu::requirements::pckgm;

/// determine which package manager is available on the system
///
/// NOTES:
/// - IDK if there is a better way to do this or maybe already a lib for this
fn detect_package_manager() -> Option<&'static str> {
    if which("apt-get").is_some() {
        Some("apt")
    } else if which("yum").is_some() {
        Some("yum")
    } else if which("dnf").is_some() {
        Some("dnf")
    } else if which("pacman").is_some() {
        Some("pacman")
    } else if which("brew").is_some() {
        Some("brew")
    } else if which("zypper").is_some() {
        Some("zypper")
    } else if which("apk").is_some() {
        Some("apk")
    } else {
        None
    }
}

/// Check if a command is available in the system's PATH
fn which(command: &str) -> Option<String> {
    match Command::new("which").arg(command).output() {
        Ok(output) if output.status.success() => {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        }
        _ => None,
    }
}

lazy_static! {
    pub static ref PACKAGE_MANAGER: Option<&'static str> = detect_package_manager();
}

/// Check if a package is installed using the detected package manager
pub(crate) fn is_package_installed(package: &str) -> Result<(), io::Error> {
    let package_manager = match *pckgm::PACKAGE_MANAGER {
        Some(pm) => pm,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No package manager detected",
            ))
        }
    };

    let output = match package_manager {
        "apt" => Command::new("dpkg-query").args(["-W", package]).output(),
        "yum" => Command::new("rpm").args(["-q", package]).output(),
        "dnf" => Command::new("dnf")
            .args(["list", "installed", package])
            .output(),
        "pacman" => Command::new("pacman").args(["-Q", package]).output(),
        "brew" => Command::new("brew").args(["list", package]).output(),
        "zypper" => Command::new("zypper").args(["se", "-i", package]).output(),
        "apk" => Command::new("apk").args(["info", package]).output(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unsupported package manager",
            ))
        }
    };

    match output {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Package '{}' is not installed", package),
        )),
    }
}
