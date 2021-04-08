#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;

use std::{
    io::Result,
    path::PathBuf,
    process::Command,
    path::Path,
    env,
    fs,
};

struct VaultContext {
    vault_path: String,
    vault_editor: String,

}

#[cfg(target_os = "linux")]
fn make_fpath(vault_path: String, file: String) -> PathBuf{
    let mut path = PathBuf::new();
    path.push(vault_path);
    path.push(file);
    // get the full path final dest for vault_path
    //let fpath = format!("{}{}", vault_path, file);
    path
}

#[cfg(target_os = "windows")]
fn make_fpath(vault_path: &String, file: &String) -> String {
    // get the full path final dest for vault_path
    let fpath = format!("{}\\{}", vault_path, file);
    fpath
}

#[cfg(target_os = "linux")]
fn spawn_vault_editor(vault_editor: String, fpath: PathBuf) {
    let path = fpath.to_string_lossy();
    Command::new(vault_editor)
        .arg(&*path)
        .exec();
}

#[cfg(target_os = "windows")]
fn spawn_vault_editor(vault_editor: String, fpath: String) {
    dbg!(&fpath);
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

// this is for the "--list" feature
fn list_dir(path: &String) {
    // ok(direntry) list of directory entries
    if let Ok(files) = fs::read_dir(path) {
        // iter over the files in ok(direntry)
        for file in files {
            if let Ok(file) = file {
                // convert direntry to osstring
                // convert osstring to string and print it out
                println!("{}", file.file_name().to_string_lossy());
            }
        }
    }
}

fn omit_file_extension() { todo!(); }

fn print_help() {
    println!("Usage: vault [OPTION/TITLE]");
    println!("Manage Notes");
    println!("\nFlags:\n");
    println!("--help   / -h:     print help message");
    println!("--purge  / -p:     purge files with one newline char");
    println!("--remove / -r:     remove a note");
    println!("--list   / -l:     list all of your notes\n");
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

    if args.len() <= 1 {
        print_help();
        return;
    }

    // sanatize the stdin and make the file name out of it
    let clean_file: String = args[1..].join("-").split_whitespace().collect();

    // grab both env vars
    let vault_path: String = env::var("VAULT_PATH").expect("Vault Path not Found");
    let vault_editor: String = env::var("VAULT_EDITOR").expect("Vault Editor not Found");

    // after we verify args is longer than 1 we can peek at what that arg is
    if args[1] == "-l" || args[1] == "--list" {
        // we just loop over all of the files in the vault
        // then we print them out at an unknown size
        list_dir(&vault_path);
        // we can exit here to not open a editor process
        return;
    }

    // args have to be four, we are expecting two paths
    if args[1] == "-s" || args[1] == "--link" && args.len() == 4 {
        // create a symlink for the project file
        let lnk = create_link(args[3].clone(), args[4].clone());
        // check if we actually created that symlink
        if lnk.is_ok() { println!("link created!"); } else { println!("link created!"); }
        return;
    }

    if args[1] == "-h" || args[1] == "--help" {
        print_help();
        return;
    }

    if args[1] == "-p" || args[1] == "--purge" {
        purge_empty_files(&vault_path);
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

        let ftr = format!("{}{}", &vault_path, args[2]);

        // go ahead and remove the file
        remove_note(&ftr, &args[2]);
        return;
    }

    // panic if the vault path does not exist
    if !Path::new(&vault_path).exists() {
        println!("VAULT_PATH does not exist");
        std::mem::drop(args);
        std::mem::drop(vault_path);
        std::mem::drop(vault_editor);
        panic!();
    }

    // get the full path final dest for vault_path
    let fpath = make_fpath(vault_path, clean_file);

    // check if file already exists- if file exists open it in the vault_editor
    if Path::new(&fpath).is_file() {

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // function defined by operating system at the top of the file
        spawn_vault_editor(vault_editor, fpath);

    } else {

        fs::File::create(&fpath).expect("Could not create file");

        // write the title of the file and start a new line
        fs::write(&fpath, args[1..].join(" ")).expect("could not write to vault file");

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // function defined by operating system at the top of the file
        spawn_vault_editor(vault_editor, fpath);
    }

    panic!();
}
