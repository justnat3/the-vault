mod context;
mod graveyard;
use crate::graveyard::grave::Grave;
use clap::*;
use std::{
    path::Path,
    env,
    fs,
};

impl Grave for Path {
}



/// print help function
fn print_help() {
    // TODO FOR THE LOVE OF EVERYTHING HOLY REWRITE ME
    println!("notebone: [NOTE_NAME/OPTION] [ARGS]");
    println!("  Bone-Keeping in a small graveyard");
    println!("");
    println!("  Your files are stored @ $VAULT_PATH");
    println!("  know your editor @ $VAULT_EDITOR");
    println!("");
    println!("  Options:");
    println!("    ls | list | -l       Listing all of your bones");
    println!("");
    println!("    remove | rm          Remove individual bones");
    println!("");
    println!("    rename               ");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");

}



fn main() {

    let matches = App::new("NoteBones")
                      .version("0.2.0")
                      .author("Nathan reed. <nreed@linux.com>")
                      .about("Bone-Keeping in a small graveyard")
                      .arg(Arg::with_name("rename")
                           .short("mv")
                           .long("rename")
                           .value_name("FILE")
                           .help("rename file src->dest")
                           .takes_value(true))
                      .arg(Arg::with_name("remove")
                           .help("remove grave with a given name")
                           .required(true)
                           .index(1))
                      .arg(Arg::with_name("v")
                           .short("v")
                           .multiple(true)
                           .help("Sets the level of verbosity"))
                      .subcommand(SubCommand::with_name("test")
                                  .about("controls testing features")
                                  .version("1.3")
                                  .author("Someone E. <someone_else@other.com>")
                                  .arg(Arg::with_name("debug")
                                      .short("d")
                                      .help("print debug information verbosely")))
                      .get_matches();

    let args: Vec<String> = env::args().collect();

    // bounds checking
    if args.len() <= 1 {
        print_help();
        return;
    }

    let s_file: String = args[1..].join("-").split_whitespace().collect();

    // make sure there are no path seperators in the file name
    let s_file = graveyard::utils::strip_seperators(s_file);


    let bones_path: String = env::var("VAULT_PATH").expect("Vault Path not Found");
    let bones_editor: String = env::var("VAULT_EDITOR").expect("Vault Editor not Found");
    let ctx = context::BoneMarrow { bones_path, bones_editor, s_file };

    // panic if the bones path does not exist
    if !Path::new(&ctx.bones_path).exists() {
        println!("VAULT_PATH does not exist");
        std::mem::drop(args);
        panic!();
    }

    // get the full path final dest for bones_path
    let fpath = ctx.make_fpath();

    // check if file already exists- if file exists open it in the bones_editor
    if Path::new(&fpath).is_file() {

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // function defined by operating system at the top of the file
        graveyard::gravekeeper::spawn_bones_editor(&ctx.bones_editor, &fpath);

    } else {

        // create the file if we can
        fs::File::create(&fpath).expect("Could not create file");

        // header for markdown file formats
        let header: String = format!("# {}", args[1..].join(" "));

        // write header and title into file
        fs::write(&fpath, header)
            .expect("could not write to bones file");

        // we just want to clean up after ourselves here
        std::mem::drop(args);

        // function defined by operating system at the top of the file
        graveyard::gravekeeper::spawn_bones_editor(&ctx.bones_editor, &fpath);
    }

    panic!();
}
