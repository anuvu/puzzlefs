extern crate fuse as fuse_ffi;

use std::path::Path;

use format::Result;
use oci::Image;

mod puzzlefs;
pub use puzzlefs::{Inode, InodeMode, PuzzleFS};

pub mod fuse;
pub use crate::fuse::Fuse;

mod walk;
pub use walk::WalkPuzzleFS;

pub fn mount<'a>(
    image: &'a Image,
    tag: &str,
    mountpoint: &Path,
) -> Result<fuse_ffi::BackgroundSession<'a>> {
    let pfs = PuzzleFS::open(image, tag)?;
    let fuse = Fuse::new(pfs);
    let session = fuse_ffi::Session::new(fuse, mountpoint, &[])?;
    let bg = unsafe { fuse_ffi::BackgroundSession::new(session) }?;
    Ok(bg)
}
