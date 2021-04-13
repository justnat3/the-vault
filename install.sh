# Thank you nyx!

cargo build --manifest-path vault/Cargo.toml --release
sudo cp .target/release/vault /usr/local/bin/vault
mkdir $HOME/.notes
# $(which $1) to get the path of the editor specified
echo "export VAULT_EDITOR=$(which $1)\nexport VAULT_PATH=$HOME/.notes" >> $HOME/.bashrc
