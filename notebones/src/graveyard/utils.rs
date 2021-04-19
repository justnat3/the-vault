// this should also not have the case that you want to "bones another file"
// that should never happen in the first place. stick it in the bones folder or symlink it
/// Make sure that we strip and replace any path seperators in the name of the file
pub fn strip_seperators(s_file: String) -> String {

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
