### What is Vault?
Vault is a tiny(under 2.5MB) note management tool made in rust,
the idea came from how I personally store my notes, which happens to be in a directory
called "the_vault". This is compatible with all editors, and I am starting to implement
more features. Like creating links to projects READMEs and other forms of documentation.

### Feedback Wanted.  If this is something you are interested in or if this is something you have used & have feedback. Please shoot me a message either by email nreed@linux.com
or you can create a issue that will be marked feedback or a respective tag.


## Table of Contents
* [Table of Contents](#table-of-contents)
* [Installation](#installation)
  * [generic-linux](#generic-linux)
* [Usage](#usage)
  * [Starting a new Note](#starting-a-new-note)
  * [Listing all of your notes](#listing-all-of-your-notes)
  * [Removing a Note](#removing-a-note)
  * [Renaming a Note](#renaming-a-note)
  * [Searching for Notes](#searching-for-notes)
  * [Vault File Linking](#vault-file-linking)
    * [Linking a project file to vault](#linking-a-project-file-to-vault)
    * [Viewing a Vault-Links Path](#viewing-a-vault-links-path)

### Installation

#### generic-linux:

```bash
# set a valid path to your vault
export VAULT_PATH=/path/to/vault

# set a valid path to your editor
export VAULT_EDITOR=/path/to/your/editor

# after cloning the repo and cd'ing into it.Just run cargo build --release
cargo build --release
```


## **WARNING: Windows is not full supported, however a wsl env with a in-terminal editor definetely is.**

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
8. **Please do not include the file name in the path on the env var you just added**
9. close your command_prompt or powershell and reopen it.

### Usage

#### You can call vault in your terminal with
```bash
# this should run fine as long as your defined VAULT_PATH/VAULT_EDITOR
$ vault
```

#### Starting a new Note
```bash
# note name with spaces. this is normal
$ vault great new note name
```
You may notice something cool here. You can type in the note name with no quotes
and with spaces. This will also create the file and attach your note title in the file

#### Listing all of your notes
```bash
# listing all the vault files
vault list

```

#### Removing a note
```bash
# be careful here because you can not recover removed notes
vault rm great-new-note-name
```

#### Renaming a note
```bash
# This will rename foo -> bar
vault rename foo bar

# this will also rename foo -> bar
vault mv foo bar
```

#### Searching for notes
```bash
# search is by keyword, given a keyword vault will return every file that contains that keyword
vault search note-keyword
```

## Vault file linking

#### Linking a project file to vault
```bash
# this would create a link called "foo" in vault -> this would then edit your project file
vault link /path/to/project-file foo

# to edit your project file you would then just execute this
vault foo
```

=======
```

#### Removing a note
```bash
# be careful here because you can not recover removed notes
vault rm great-new-note-name
```

#### Renaming a note
```bash
# This will rename foo -> bar
vault rename foo bar

# this will also rename foo -> bar
vault mv foo bar
```

#### Searching for notes
```bash
# search is by keyword, given a keyword vault will return every file that contains that keyword
vault search note-keyword
```

## Vault file linking

#### Linking a project file to vault
```bash
# this would create a link called "foo" in vault -> this would then edit your project file
vault link /path/to/project-file foo 

# to edit your project file you would then just execute this
vault foo
```

#### Viewing a vault-links path
```bash
# pulling off of our example above. Lets say you forget where this link goes to
vault view foo

# this ^^ will return  "" Linked to: /path/to/project-file ""
```

