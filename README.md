# What the fuck is this project?

### The Vault
* The vault is just a dir full of markdown files. The vault is a static dir, meaning that it is placed in a config file. This is just for local persistence.

* for me this will be a bash_alias on my system so that I can bind it in my vimrc, for whenever I want to open a new note

* though args I should be able to say what the title is of the new note

### TODO
[] take a input of a note_title
[] "setup" script preferably in the language
[] create a note dir that should be statico
[] write a new first with "# the_arg_input" as your title then open in a configured editor

### bash helpers
creating a alias that will take the first arg as a note
We will just ignore any other args or say that "you shouldn't have more than 1 arg"
```bash
write_note() {
    # write a note with a title
    note-binary "$1"
}
```

```bash
# or do this instead
alias your_bin='bin() { bin "$@" }'
```

 this should be tested on what will yield the best result
