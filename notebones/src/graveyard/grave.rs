// file system operations can be substutied for traits
use std::{
    io::Result,
    path::Path,
};

/// Just another word for Note
pub trait Grave {
    // removing the link is just removing the note because the note you are accessing
    // is actually just a symlink
    #[cfg(not(target_os = "windows"))]
    fn create_tunnel(source_file: &Path, lnk_name: &Path) -> Result<()>;

    /// view symlinks
    fn peek_tunnel(path: &Path) -> Result<()>;

    /// rename notes without having to interact with the bones path
    fn move_grave(old: &Path, new: &Path);

    /// remove notes without having to interact with the bones path
    fn dig_up(path: &Path, f_name: &str);
}
