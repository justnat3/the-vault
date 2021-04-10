#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;

use std::{
    io::Error,
    io::ErrorKind,
    io::Result,
    path::PathBuf,
    process::Command,
    path::Path,
    env,
    fs,
};

// XXX NEW FEATS
//      search
//      rename
//
// XXX be able to give notes a description and just use that in the note?
// XXX search should instead be a match, so you can search for multiple files
//     and for one file all at the same time
// XXX


/// Keeping our Vault's Context alive
struct VaultContext {
    /// path to the directory that holds all of our vault files
    vault_path: String,
    /// path to the editor executable
    vault_editor: String,
    /// the file that the user passes in that they want to edit
    /// this only applies if they pass in a file that they want to edit
    s_file: String // this all though be changed to PathBufs
}

/// collection of functions that take advantage of the VaultContext structure
impl VaultContext {
    /// Create a new VaultContext
    fn new(vp: String, ve: String, sf: String) -> Self {
        Self { vault_path: vp, vault_editor: ve, s_file: sf }
    }

    /// Create full path context to verify later in the program
    #[cfg(target_os = "linux")]
    fn make_fpath(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(&self.vault_path);
        path.push(&self.s_file);

        path
    }

    /// Create full path context to verify later in the program but windows
    #[cfg(target_os = "windows")]
    fn make_fpath(&self) -> String {
        let mut path = PathBuf::new();

        path.push(self.vault_path);
        path.push(self.file);

        path
    }

}

// All execve does is take the parent process image and replace it with new process
/// This uses the execve syscall under the hood
#[cfg(target_os = "linux")]
fn spawn_vault_editor(vault_editor: String, fpath: PathBuf) {
    let path = fpath.to_string_lossy();
    Command::new(vault_editor)
        .arg(&*path)
        .exec();
}

/// this is currently unsupported
#[cfg(target_os = "windows")]
fn spawn_vault_editor(vault_editor: String, fpath: String) {
    Command::new(vault_editor)
        .arg(fpath)
        .spawn()
        .expect("Could not spawn vault_editor process for unknown reason");
}

// TODO: REFACTOR ME PLEASE
fn purge_empty_files(path: &String) {
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

fn remove_note(path: &String, file_name: &String) {
    // if the path contains the filename as verification then we can go ahead
    // and delete that file.
    if path.contains(file_name) {
        //verify that the file was removed
        let remove_file = fs::remove_file(path);
        if remove_file.is_ok() {
            println!("{}", file_name);
        } else {
            // if the file was not removed let the user know
            println!("Not able to remove {} ", file_name);
        }
    }
}

fn rename_note(old: PathBuf, new: PathBuf) -> Result<()> {
    // if the path does not already exist go ahead and rename it
    if Path::new(&new).exists() {
        Err(Error::new(ErrorKind::Other, "path already exists"))
    } else {
        fs::rename(old, new)?;
        Ok(())
    }
}

/// the output is the display of all avaliable files based on a search
///
/// vault -s file
///
/// this may return one or more matches based on the input string
/// treat this more of a grep that highlights the files more than search for a specific
/// file this is will also allow for smaller search inputs
fn search_for_file(k_word: &String, ctx_path: PathBuf) {
    // ok(direntry) list of directory entries
    if let Ok(files) = fs::read_dir(ctx_path) {
        println!("\n         Found");
        println!("  -----------------------");
        // iter over the files in ok(direntry)
        for file in files {
            if let Ok(file) = file {
                if file.file_name().to_string_lossy().contains(k_word) {
                    // convert direntry to osstring
                    // convert osstring to string and print it out
                    println!("    {}", file.file_name().to_string_lossy());
                }
            }
        }
    }
    print!("\n");
}

// this is for the "--list" feature
fn list_dir(path: &String) {
    // ok(direntry) list of directory entries
    if let Ok(files) = fs::read_dir(path) {
        println!("\n         Vault Files");
        println!("  --------------------------");
        // iter over the files in ok(direntry)
        for file in files {
            if let Ok(file) = file {
                // convert direntry to osstring
                // convert osstring to string and print it out
                println!("  {}", file.file_name().to_string_lossy());
            }
        }
    }
    print!("\n");
}

fn omit_file_extension() { todo!(); }

fn print_help() {
    println!("Usage: vault [OPTION/TITLE]");
    println!("Manage Notes");
    println!("\nFlags:\n");
    println!("--help    / -h:     print help message");
    println!("--purge   / -p:     purge files with one newline char");
    println!("--remove  / -r:     remove a note");
    println!("--list    / -l:     list all of your notes");
    println!("--search  / -s:     search by keyword, display what matches the keyword");
    println!("--rename  / -r:      rename a note\n");
}

// removing the link is just removing the note because the note you are accessing
// is actually just a symlink
#[cfg(target_os = "linux")]
fn create_link(source_file: String, lnk_name: String) -> Result<()> {
    std::os::unix::fs::symlink(source_file, lnk_name)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn verify_path(path_to_lnk: String) { todo!(); }
#[cfg(target_os = "windows")]
fn create_link(source_file: String, lnk_name: String) { todo!(); }


fn search_vault() { todo!(); }

fn main() {

    let args: Vec<String> = env::args().collect();

    // bounds checking
    if args.len() <= 1 {
        print_help();
        return;
    }

    let s_file: String = args[1..].join("-").split_whitespace().collect();
    let vault_path: String = env::var("VAULT_PATH").expect("Vault Path not Found");
    let vault_editor: String = env::var("VAULT_EDITOR").expect("Vault Editor not Found");
    let ctx = VaultContext { vault_path, vault_editor, s_file };

    if args[1] == "-r" || args[1] == "--rename" {
        // bounds checking
        if args.len() != 4 {
            print_help();
            return;
        }
        // create the old path as pathbuf
        let mut old = PathBuf::new();
        old.push(&ctx.vault_path);
        // args-3 is the source file
        old.push(&args[2]);

        // create a path that represent the new file
        let mut new = PathBuf::new();
        new.push(&ctx.vault_path);
        // args-4 is the new file name
        // to see if that file name already exists we verify that in the rename func
        dbg!(&args);
        new.push(&args[3]);

        if rename_note(old, new).is_ok() {
            println!("Note Renamed {}", &args[3]);
        } else {
            println!("Note Already Exists {}", &args[3]);
        }

        return;
    }

    // after we verify args is longer than 1 we can peek at what that arg is
    if args[1] == "-l" || args[1] == "--list" {
        // we just loop over all of the files in the vault
        // then we print them out at an unknown size
        list_dir(&ctx.vault_path);
        // we can exit here to not open a editor process
        return;
    }

    if args[1] == "-s" || args[1] == "--search" {
        // bounds checking
        if args.len() != 3 {
            print_help();
            return;
        }

        let mut n_pbuf = PathBuf::new();
        n_pbuf.push(&ctx.vault_path);
        search_for_file(&args[2], n_pbuf);

        return;
    }

    if args[1] == "-h" || args[1] == "--help" {
        print_help();
        return;
    }

    if args[1] == "-p" || args[1] == "--purge" {
        purge_empty_files(&ctx.vault_path);
        return;
    }

    if args[1] == "-r" || args[1] == "--remove" {
        // it makes sense that the args are of length 3 because we only really
        // want to remove one file
        if !args.len() == 3 || args.len() <= 2 {
            // print if they don't know how to use remove
            print_help();
            return;
        }

        let ftr = format!("{}{}", &ctx.vault_path, args[2]);

        // go ahead and remove the file
        remove_note(&ftr, &args[2]);
        return;
    }

    // panic if the vault path does not exist
    if !Path::new(&ctx.vault_path).exists() {
        println!("VAULT_PATH does not exist");
        std::mem::drop(args);
        panic!();
    }

    // get the full path final dest for vault_path
    let fpath = ctx.make_fpath();

    // check if file already exists- if file exists open it in the vault_editor
    if Path::new(&fpath).is_file() {

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // function defined by operating system at the top of the file
        spawn_vault_editor(ctx.vault_editor, fpath);

    } else {

        fs::File::create(&fpath).expect("Could not create file");

        // write the title of the file and start a new line
        fs::write(&fpath, args[1..].join(" ")).expect("could not write to vault file");

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // function defined by operating system at the top of the file
        spawn_vault_editor(ctx.vault_editor, fpath);
    }

    panic!();
}
