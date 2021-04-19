use std::fs;
use std::path::Path;

trait Graveyard {
    // TODO: REFACTOR ME PLEASE
    /// if there are files with the header # bleh blah
    ///
    /// ^ new line
    /// and there is nothing else in the file then we can assume that the file is a empty
    /// initialized note.
    // XXX just do it based on file size you fuck
    fn purge_empty_files(path: &Path) {
        if let Ok(files) = fs::read_dir(path) {
            // iter over the files in ok(direntry)
            for file in files {
                if let Ok(file) = file {
                    let file_read = fs::read_to_string(&file.path());
                    if let Ok(file_read) = file_read {
                        let c = file_read.matches("\n").count();
                        if c == 1 {
                            let file_removed = fs::remove_file(&file.path());
                            if file_removed.is_ok() {
                                println!("Remove File {:?}", &file.path());
                            }
                        }
                    }
                }
            }
        }
    }

    /// the output is the display of all avaliable files based on a search
    ///
    /// bones -s file
    ///
    /// this may return one or more matches based on the input string
    /// treat this more of a grep that highlights the files more than search for a specific
    /// file this is will also allow for smaller search inputs
    fn search_for_file(ctx_path: &Path, k_word: &str) {
        println!("");
        // ok(direntry) list of directory entries
        if let Ok(files) = fs::read_dir(ctx_path) {
            // iter over the files in ok(direntry)
            for file in files {
                if let Ok(file) = file {
                    if file.file_name().to_string_lossy().contains(k_word) {
                        // convert direntry to osstring
                        // convert osstring to string and print it out
                        println!("\x1b[32mFound\x1b[0m {}", file.file_name().to_string_lossy());
                    }
                }
            }
        }
        print!("\n");
    }

    /// a way to list all the bones files in alphabetical order
    fn list_dir(path: &Path) {
        let mut results: Vec<_> = Vec::new();
        // ok(direntry) list of directory entries
        if let Ok(files) = fs::read_dir(path) {
            println!("\n         \x1b[7mGraveyard\x1b[0m");
            println!("  --------------------------");
            // iter over the files in ok(direntry)
            for file in files {
                if let Ok(file) = file {

                    // if the file is a symlink we should include a link tag
                    let attr = fs::read_link(file.path());
                    if attr.is_ok() {
                        let is_sym = format!(
                                "{} <- \x1b[32mLink\x1b[0m",
                                file.file_name().to_string_lossy().to_string()
                                );

                        results.push(is_sym);
                        continue
                    }

                    results.push(file.file_name().to_string_lossy().to_string());
                    // convert direntry to osstring
                    // convert osstring to string and print it out
                }
            }
        }

        results.sort();
        for file in results {
            println!("   {}", file);
        }
        print!("\n");
    }
}
