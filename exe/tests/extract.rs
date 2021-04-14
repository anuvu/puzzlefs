use std::ffi::OsStr;
use std::process::Command;

extern crate dir_diff;

use tempfile::tempdir;

mod helpers;
use helpers::{get_image, puzzlefs};

#[test]
fn build_and_extract_is_noop() {
    let dir = tempdir().unwrap();
    let ubuntu = dir.path().join("ubuntu");
    get_image(&ubuntu).unwrap();

    // TODO: remove this once we can actually do symbolic links
    assert!(Command::new("find")
        .args(&[
            ubuntu.as_ref(),
            OsStr::new("-type"),
            OsStr::new("l"),
            OsStr::new("-delete")
        ])
        .status()
        .unwrap()
        .success());

    // TODO: figure out a better way to do all this osstr stuff...
    let oci = dir.path().join("oci");
    puzzlefs(&[
        OsStr::new("build"),
        ubuntu.as_ref(),
        oci.as_ref(),
        OsStr::new("test"),
    ]);

    let extracted = dir.path().join("extracted");
    puzzlefs(&[
        OsStr::new("extract"),
        oci.as_os_str(),
        OsStr::new("test"),
        extracted.as_os_str(),
    ]);
    assert!(!dir_diff::is_different(ubuntu, extracted).unwrap())
}
