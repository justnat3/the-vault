Vault is a tiny(under 2MB) note management tool made in rust,
the idea came from how I personally store my notes, which happens to be in a directory
called "the_vault". This is compatible with all editors, and I am starting to implement
more features. Like creating links to projects READMEs and other forms of documentation.

### Installation / Building
Right now you will have compile this project from source, as I do not have this project
compiled for different distros.

generic-linux:

```bash
# set a valid path to your vault
export VAULT_PATH=/path/to/vault

# set a valid path to your editor
export VAULT_EDITOR=/path/to/your/editor

# after cloning the repo and cd'ing into it.Just run cargo build --release
cargo build --release
```

windows:

cmd:
```bash
# set a valid path to your vault
set VAULT_PATH "C:/path/to/vault"

# set a valid path to your editor
set VAULT_EDITOR "C:/path/to/your/editor"

# after cloning the repo and cd'ing into it.Just run cargo build --release
cargo build --release

# you can access the binary in /target/release/vault
```

powershell:
```bash
# set a valid path to your vault
$env:VAULT_PATH = "C:/path/to/vault"

# set a valid path to your editor
$env:VAULT_EDITOR = "C:/path/to/your/editor"

# clone the repo cd the_vault/vault then run the cargo build like below
cargo build --release

# you can access the binary in /target/release/vault
```

1. Open the start menu
2. Open the option `Edit the system environment variables`
3. Click `Environment variables...` button
4. in `System variables` find path
5. Click `Edit`
6. click `new`
7. Put the path to your vault.exe - recommended to put it in C:/Program Files
8. Please include the file name in the path on the env var you just added
9. close your command_prompt or powershell and reopen it.

### Usage

You can call vault in your terminal with

```bash
# this should run fine as long as your defined VAULT_PATH/VAULT_EDITOR
$ vault
```

You can start a new note by doing the following.
```bash
# note name with spaces. this is normal
$ vault great new note name
```
You may notice something cool here. You can type in the note name with no quotes
and with spaces. This will also create the file and attach your note title in the file

Listing all of your notes
```
$ vault -l
```

Removing a note
```
# notice that the note name has no spaces. this is also intentional
$ vault -r great-new-note-name
```

