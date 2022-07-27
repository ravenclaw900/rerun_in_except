//! This library lets you rerun a build script if files in a directory have changed, excluding specific ones that are listed

#![warn(clippy::pedantic)]
use std::path::Path;

/// Return cargo rerun-if-changed lines to rerun build script if files in folder are changed, excluding listed
///
/// Takes the directory to check, and a slice of files under that directory to exclude. Returns a string in the format:
/// ```text
/// cargo:rerun-if-changed=path/to/folder/test.c
/// cargo:rerun-if-changed=path/to/folder/test.h
/// ```
///
/// # Errors
/// Errors if the directory cannot be read. Any invalid files will be skipped.
///
/// # Example
/// ```
/// println!(rerun_in_except("frontend", &["frontend/node_modules", "frontend/artifacts"]))
/// ```
pub fn rerun_in_except(
    run_in: impl AsRef<Path>,
    except: &[impl AsRef<Path>],
) -> std::io::Result<String> {
    let mut string = String::new();
    let paths = except.iter().map(std::convert::AsRef::as_ref);
    string.extend(
        std::fs::read_dir(run_in)?
            .filter_map(Result::ok)
            .filter(|x| !paths.clone().any(|y| y == x.path()))
            .filter_map(|x| x.path().into_os_string().into_string().ok())
            .map(|x| format!("cargo:rerun-if-changed={}\n", x)),
    );
    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rerun_in_except_test() {
        assert_eq!(
            rerun_in_except(
                env!("CARGO_MANIFEST_DIR"),
                &[
                    concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
                    concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock"),
                    concat!(env!("CARGO_MANIFEST_DIR"), "/.git"),
                ],
            )
            .unwrap(),
            concat!(
                "cargo:rerun-if-changed=",
                env!("CARGO_MANIFEST_DIR"),
                "/target\ncargo:rerun-if-changed=",
                env!("CARGO_MANIFEST_DIR"),
                "/.gitignore\ncargo:rerun-if-changed=",
                env!("CARGO_MANIFEST_DIR"),
                "/src\n"
            )
        );
    }
}
