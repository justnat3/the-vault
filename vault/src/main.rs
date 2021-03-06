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

// XXX be able to give notes a description and just use that in the note?
// XXX search should instead be a match, so you can search for multiple files
//     and for one file all at the same time

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
    /// Create full path context to verify later in the program
    fn make_fpath(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(&self.vault_path);
        path.push(&self.s_file);

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
/// if there are files with the header # bleh blah
///
/// ^ new line
/// and there is nothing else in the file then we can assume that the file is a empty
/// initialized note.
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

/// remove notes without having to interact with the vault path
fn remove_note(path: &String, file_name: &String) {
    // if the path contains the filename as verification then we can go ahead
    // and delete that file.
    if path.contains(file_name) {
        //verify that the file was removed
        let remove_file = fs::remove_file(path);
        if remove_file.is_ok() {
            println!("\x1b[0;31mRemoved \x1b[0m{}", file_name);
        } else {
            // if the file was not removed let the user know
            println!("Not able to remove {} ", file_name);
        }
    }
}

/// rename notes without having to interact with the vault path
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

/// a way to list all the vault files in alphabetical order
fn list_dir(path: &String) {
    let mut results: Vec<_> = Vec::new();
    // ok(direntry) list of directory entries
    if let Ok(files) = fs::read_dir(path) {
        println!("\n         \x1b[7mVault Files\x1b[0m");
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

fn view_symlink(path: PathBuf) -> Result<()> {
    let attr = fs::read_link(path);
    if let Ok(attr) = attr {
        let attr = attr.into_os_string().into_string().expect("could not conver to string");
        println!("\x1b[32mLinked To\x1b[0m: {}", attr);
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "File is not a symlink"))
    }
}

/// print help function
fn print_help() {
    // TODO FOR THE LOVE OF EVERYTHING HOLY REWRITE ME
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
fn create_link(source_file: PathBuf, lnk_name: PathBuf) -> Result<()> {
    std::os::unix::fs::symlink(source_file, lnk_name)?;
    Ok(())
}

// this should also not have the case that you want to "vault another file"
// that should never happen in the first place. stick it in the vault folder or symlink it
/// Make sure that we strip and replace any path seperators in the name of the file
fn strip_seperators(s_file: String) -> String {
    // make no assumptions, s_file may have a path seperator in it
    // PathBuf will just allow you to push it thinking its the rest of the path
    if s_file.contains("\\")  {
        // make sure there is only one hyphen next to a seperator
        if s_file.contains("-\\") {
            let s_file = s_file.replace("-\\", "-");
            let s_file = s_file.replace("\\", "-");
            return s_file;
        } else {
            // replace seperator with hyphen
            let s_file = s_file.replace("\\", "-");
            return s_file;
        };
    }

    if s_file.contains("/")  {
        // make sure there is only one hyphen next to a seperator
        if s_file.contains("-/") {
            let s_file = s_file.replace("-/", "-");
            let s_file = s_file.replace("/", "-");
            return s_file;
        } else {
            // replace seperator with hyphen
            let s_file = s_file.replace("/", "-");
            return s_file;
        };
    }

    // later in the program may still fail because of this
    // however we are assume at this point there is no seperators
    s_file
}

fn main() {

    let args: Vec<String> = env::args().collect();

    // bounds checking
    if args.len() <= 1 {
        print_help();
        return;
    }

    let s_file: String = args[1..].join("-").split_whitespace().collect();

    // make sure there are no path seperators in the file name
    let s_file = strip_seperators(s_file);


    let vault_path: String = env::var("VAULT_PATH").expect("Vault Path not Found");
    let vault_editor: String = env::var("VAULT_EDITOR").expect("Vault Editor not Found");
    let ctx = VaultContext { vault_path, vault_editor, s_file };

    if args[1] == "link" {
        // some bounds checking
        if args.len() != 4 {
            print_help();
            return;
        }

        // the first arg or arg-2 is what file you plan to link
        let link_path = PathBuf::from(&args[2]);
        let link_path = fs::canonicalize(&link_path).expect("Could not create path");


        // the second arg or arg-3 is what the link name should be
        let mut link_name = PathBuf::new();
        link_name.push(&ctx.vault_path);

        // create link address
        link_name.push(&args[3]);

        // finally we can create the link
        create_link(link_path, link_name).expect("Failed to create the link");
        println!("Link Created! {}", &args[3]);

        return;
    }

    if args[1] == "rename" || args[1] == "mv" {
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
        new.push(&args[3]);

        if rename_note(old, new).is_ok() {
            println!("Note Renamed {}", &args[3]);
        } else {
            println!("Note Already Exists {}", &args[3]);
        }

        return;
    }

    // after we verify args is longer than 1 we can peek at what that arg is

    if args[1] == "-l" || args[1] == "list" || args[1] == "ls" {
        // we just loop over all of the files in the vault
        // then we print them out at an unknown size
        list_dir(&ctx.vault_path);
        // we can exit here to not open a editor process
        return;
    }

    if args[1] == "view" {
        if args.len() != 3 {
            print_help();
            return
        }
        // there will be this idea that you can look into files
        let mut n_path = PathBuf::new();
        n_path.push(&ctx.vault_path);
        n_path.push(&args[2]);
        let view = view_symlink(n_path);

        // FIXME this is kind of ugly
        if view.is_ok() { return; } else { return; }
    }

    if args[1] == "-s" || args[1] == "search" {
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

    if args[1] == "remove" || args[1] == "rm" {
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

        // create the file if we can
        fs::File::create(&fpath).expect("Could not create file");

        // header for markdown file formats
        let header: String = format!("# {}", args[1..].join(" "));

        // write header and title into file
        fs::write(&fpath, header)
            .expect("could not write to vault file");

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // function defined by operating system at the top of the file
        spawn_vault_editor(ctx.vault_editor, fpath);
    }

    panic!();
}
