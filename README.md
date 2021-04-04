# the-vault
Note Taking Helper

### What is this?
I made t his as a way of note taking in vim- central management of my help-texts, docs, notes, things I have learned
etc.
This can also be used outside of vim.

### How do I use it?
* ln -s release/bin /usr/bin/vault
* vault -h # for help text

### Env Vars you need
export VAULT_PATH # path to your vault

export VAULT_EDITOR # path to your editor

### Creating a note
```bash
vault title dont worry about whitespace
```
### Removing a note
```bash
vault -r remove-note
```
### Listing all of the notes
```bash
vault -l # list
```
