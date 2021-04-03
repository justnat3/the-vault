use nix::{
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};

use std::{
    thread::sleep,
    time::Duration,
    process::exit,
    process::Command,
    fs,
    env,
    path::Path,
};

// current progress:
//  * check vault path and editor in env
//  * check if the vault path exists
//  * check if the file at that path exists
//  * take and verify args
//  * create a full note path
//  * file titles are the same as the file name by default
//  * create and edit file

fn main() {

    // take in the arguments
    let args: Vec<String> = env::args().collect();

    // Give them the help prompt
    if args.len() <= 1 {
        // panic! we do not have the information we need
        panic!("THE VAULT:\nPlease Provide an title for your new note");
    }

    let clean_file: String = args[1].to_owned().split_whitespace().collect();

    // Handle Vault_Path
    let vault_path: String = env::var("VAULT_PATH")
        .unwrap_or_else(|err| {
            // We want to let the user know that we could not find VAULT_PATH
            eprintln!("Oh No! {} -> VAULT_PATH", err);
            std::process::exit(1);
        });

    let vault_editor: String = env::var("VAULT_EDITOR")
        .unwrap_or_else(|err| {
            // We want to let the user know that we could not find VAULT_PATH
            eprintln!("Oh No! {} -> VAULT_EDITOR", err);
            std::process::exit(1);
        });

    // check if the vault path exists
    if !Path::new(&vault_path).exists() {
        // warn and exit if the path does exist
        eprintln!("\nVault Path does not exist");
        std::process::exit(1);
    }

    // get the full path final dest for vault_path
    let fpath = format!("{}{}", vault_path, &clean_file);

    dbg!(&fpath);

    // check if file already exists- if file exists open it in the vault_editor
    if Path::new(&fpath).is_file() {

        match unsafe{fork().expect("did not fork process")} {
            ForkResult::Parent { child } => {
                println!("try to kill me to check if the target process will be killed");
                // wait for the fork to prevent zombie processes
                waitpid(Some(child), None).unwrap();

                println!("exit");
                exit(0)
            }

            ForkResult::Child => {

                // spawn a process
                Command::new(&vault_editor)
                    .arg(&fpath)
                    .spawn()
                    .expect("Failed to launch editor");
                exit(0);
            }
        };
    } else {
        // create a new note
        fs::File::create(&fpath)
            .unwrap_or_else(|err| {
                // We want to let the user know that we could not find VAULT_PATH
                eprintln!("Oh No! {}", err);
                std::process::exit(1);
            });

        // write the title of the file and start a new line
        // fs::write(&fpath, format!("# {}\n", args[1]))
        //    .unwrap_or_else(|err| {
                // We want to let the user know that we could not find VAULT_PATH
        //        eprintln!("Oh No! {}", err);
        //        std::process::exit(1);
        //    });


        match unsafe{fork().expect("did not fork process")} {
            ForkResult::Parent { child } => {
                // wait for the fork to prevent zombie processes
                waitpid(Some(child), None).unwrap();

                exit(0)
            }

            ForkResult::Child => {
                // spawn a process
                Command::new(&vault_editor)
                    .arg(&fpath)
                    .spawn()
                    .expect("Failed to launch editor");
            }
        };
    }
    exit(0)
}
