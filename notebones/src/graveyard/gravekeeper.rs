#[cfg(not(target_os = "windows"))]
use std::os::unix::process::CommandExt;

use std::path::Path;
use std::process::Command;

// All execve does is take the parent process image and replace it with new process
/// This uses the execve syscall under the hood
#[cfg(not(target_os = "windows"))]
pub fn spawn_bones_editor(bones_editor: &str, fpath: &Path) {
    let path = fpath.to_string_lossy();
    Command::new(bones_editor)
        .arg(&*path)
        .exec();
}

/// this is currently unsupported
#[cfg(target_os = "windows")]
pub fn spawn_bones_editor(bones_editor: &str, fpath: &Path) {
    Command::new(bones_editor)
        .arg(fpath)
        .spawn()
        .expect("Could not spawn bones_editor process for unknown reason");
}
