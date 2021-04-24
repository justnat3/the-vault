mod context;
mod graveyard;
use crate::graveyard::grave::Grave;
use std::{
    io,
    path::Path,
    fs,
    env,
};

impl Grave for Path {
    #[cfg(not(target_os = "windows"))]
    fn create_tunnel(source_file: &Path, lnk_name: &Path) -> io::Result<()> {
       match std::os::unix::fs::symlink(source_file, lnk_name) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn peek_tunnel(path: &Path) -> io::Result<()> {
        let attr = fs::read_link(path).unwrap();

        attr
            .into_os_string()
            .into_string()
            .expect("could not conver to string");

        Ok(())
    }

    fn move_grave(old: &Path, new: &Path) {
        // if the path does not already exist go ahead and rename it
        if Path::new(&new).exists() { fs::rename(old, new).unwrap(); }
    }

    fn dig_up(path: &Path, f_name: &str) {
        // we already check if the path exists, otherwise there are bad perms
        if path.exists() {
            fs::remove_file(path).unwrap();
            println!("\x1b[0;31mRemoved \x1b[0m{}", f_name);
        }
    }

}

fn main() {


    let args: Vec<String> = env::args().collect();


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
}
