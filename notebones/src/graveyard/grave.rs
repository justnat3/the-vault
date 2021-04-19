// file system operations can be substutied for traits
use std::{
    io::Error,
    io::ErrorKind,
    io::Result,
    path::Path,
    fs,
};

/// Just another word for Note
pub trait Grave {
    // removing the link is just removing the note because the note you are accessing
    // is actually just a symlink
    #[cfg(not(target_os = "windows"))]
    fn create_tunnel(source_file: &Path, lnk_name: &Path) -> Result<()> {
        std::os::unix::fs::symlink(source_file, lnk_name)?;
        Ok(())
    }

    fn peek_tunnel(path: &Path) -> Result<()> {
        let attr = fs::read_link(path);

        if let Ok(attr) = attr {
            let attr = attr
                .into_os_string()
                .into_string()
                .expect("could not conver to string");
            println!("\x1b[32mLinked To\x1b[0m: {}", attr);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "File is not a symlink"))
        }
    }

    /// rename notes without having to interact with the bones path
    fn move_grave(old: &Path, new: &Path) -> Result<()> {
        // if the path does not already exist go ahead and rename it
        if Path::new(&new).exists() {
            Err(Error::new(ErrorKind::Other, "path already exists"))
        } else {
            fs::rename(old, new)?;
            Ok(())
        }
    }

    /// remove notes without having to interact with the bones path
    fn dig_up(path: &Path, f_name: &str) -> Result<()> {
        // convert path(PathBuf) to &Path with .to_path()
        if path.exists() {
            fs::remove_file(path)?;
            println!("\x1b[0;31mRemoved \x1b[0m{}", f_name);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Could not remove file"))
        }
    }
}
