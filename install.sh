#!/bin/bash

cargo build --manifest-path notebones/Cargo.toml --release

# this is to ensure it works on every system
sudo cp ./notebones/target/release/notebone /bin/notebone

# check to see if the directory already exists
if [ ! -d "$HOME/.notes" ]; then
    mkdir $HOME/.notes
fi

# check if they provide a editor
if [ $# -eq 0 ]; then
    echo "Specify editor as argument to install script"
    exit
fi

# only add the configuration if the grep returns 1
grep -i "VAULT_EDITOR" $HOME/.bashrc &>>/dev/null
if [ $? -eq 1 ]; then
    # $(which $1) to get the path of the editor specified
    echo -e "export VAULT_EDITOR=$(which $1 2>>/dev/null);\nexport VAULT_PATH=$HOME/.notes" >> $HOME/.bashrc
fi

# Thanks Nyx!
