
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
    link_name.push(&ctx.bones_path);

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
    old.push(&ctx.bones_path);
    // args-3 is the source file
    old.push(&args[2]);

    // create a path that represent the new file
    let mut new = PathBuf::new();
    new.push(&ctx.bones_path);
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
    // we just loop over all of the files in the bones
    // then we print them out at an unknown size
    list_dir(&ctx.bones_path);
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
    n_path.push(&ctx.bones_path);
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
    n_pbuf.push(&ctx.bones_path);
    search_for_file(&args[2], n_pbuf);

    return;
}

if args[1] == "-h" || args[1] == "--help" {
    print_help();
    return;
}

if args[1] == "-p" || args[1] == "--purge" {
    purge_empty_files(&ctx.bones_path);
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

    let mut ftr = PathBuf::new();
    ftr.push(&ctx.bones_path);
    ftr.push(&args[2]);

    // go ahead and remove the file
    if remove_note(&ftr, &args[2]).is_ok() {
        return;
    } else { println!("could not remove file"); return; }
}

