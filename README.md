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


