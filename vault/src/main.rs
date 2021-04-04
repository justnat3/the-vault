use std::{
    os::unix::process::CommandExt,
    process::Command,
    path::Path,
    env,
    fs,
};

// this is for the "--list" feature
fn list_dir(path: &String) {
    // Ok(DirEntry) list of directory Entries
    if let Ok(files) = fs::read_dir(path) {
        // iter over the files in Ok(DirEntry)
        for file in files {
            if let Ok(file) = file {
                // convert DirEntry to OsString
                // convert OsString to String and print it out
                println!("{}", file.file_name().to_string_lossy());
            }
        }
    }
}

fn main() {

    // take in the arguments
    let args: Vec<String> = env::args().collect();

    // Give them the help prompt
    if args.len() <= 1 {
        // panic! we do not have the information we need
        panic!("THE VAULT:\nPlease Provide an title for your new note");
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

    // Handle Vault_Editor path
    let vault_editor: String = env::var("VAULT_EDITOR").unwrap();

    // panic if the vault path does not exist
    if !Path::new(&vault_path).exists() { panic!("vault path does not exist"); }

    // get the full path final dest for vault_path
    let fpath = format!("{}{}", vault_path, &clean_file);

    // check if file already exists- if file exists open it in the vault_editor
    if Path::new(&fpath).is_file() {

        std::mem::drop(&vault_path);
        std::mem::drop(&clean_file);
        // launch editor
        Command::new(&vault_editor)
            .arg(&fpath)
            .exec();

    } else {
        // create a new note
        fs::File::create(&fpath).unwrap();

        // write the title of the file and start a new line
        fs::write(&fpath, args[1..].join(" ")).expect("problem");

        std::mem::drop(&vault_path);
        std::mem::drop(&clean_file);
        // launch editor
        Command::new(&vault_editor)
            .arg(&fpath)
            .exec();
    }
    return;
}
