use std::{
    os::unix::process::CommandExt,
    process::Command,
    path::Path,
    env,
    fs,
};

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

fn print_help() {
    println!("Usage: vault [OPTION/TITLE]");
    println!("Manage Notes");
    println!("Flags:\n");
    println!("--help  / -h:     print help message");
    println!("--purge / -p:     purge files with one newline char");
    println!("--remove / -r:    remove a note");

}

fn main() {

    // take in the arguments
    let args: Vec<String> = env::args().collect();

    // Give them the help prompt
    if args.len() <= 1 {
        print_help();
        return;
        // panic! we do not have the information we need
        //panic!("THE VAULT:\nPlease Provide an title for your new note");
    }

    // sanatize the stdin and make the file name out of it
    let clean_file: String = args[1..].join("-").split_whitespace().collect();

    // Handle Vault_Path
    let vault_path: String = env::var("VAULT_PATH").unwrap();

    // after we verify args is longer than 1 we can peek at what that arg is
    if args[1] == "-l" || args[1] == "--list" {
        // we just loop over all of the files in the vault
        // then we print them out at an unknown size
        list_dir(&vault_path);
        // we can exit here to not open a editor process
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
        // format into a path
        let ftr = format!("{}{}", &vault_path, args[2]);

        // go ahead and remove the file
        remove_note(&ftr, &args[2]);
        return;
    }

    // Handle Vault_Editor path
    let vault_editor: String = env::var("VAULT_EDITOR").unwrap();

    // panic if the vault path does not exist
    if !Path::new(&vault_path).exists() {
        println!("VAULT_PATH does not exist");
        std::mem::drop(args);
        std::mem::drop(vault_path);
        std::mem::drop(vault_editor);
        panic!();
    }

    // get the full path final dest for vault_path
    let fpath = format!("{}{}", vault_path, &clean_file);

    // check if file already exists- if file exists open it in the vault_editor
    if Path::new(&fpath).is_file() {

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // launch editor
        Command::new(vault_editor)
            .arg(&fpath)
            .exec();

    } else {
        // create a new note
        fs::File::create(&fpath).unwrap();

        // write the title of the file and start a new line
        fs::write(&fpath, args[1..].join(" ")).expect("problem");

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // launch editor
        Command::new(vault_editor)
            .arg(fpath)
            .exec();

    } // fpath drops at end of scope

    std::mem::drop(vault_path);
    // panic_immediate_abort this is safe
    panic!();
}
