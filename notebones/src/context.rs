use std::path::PathBuf;
/// Keeping our Vault's Context alive
pub struct BoneMarrow {
    /// path to the directory that holds all of our bones files
    pub bones_path: String,
    /// path to the editor executable
    pub bones_editor: String,
    /// the file that the user passes in that they want to edit
    /// this only applies if they pass in a file that they want to edit
    pub s_file: String // this all though be changed to PathBufs
}

/// collection of functions that take advantage of the BoneMarrow structure
impl BoneMarrow {
    /// Create full path context to verify later in the program
    pub fn make_fpath(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(&self.bones_path);
        path.push(&self.s_file);

        path
    }
}

