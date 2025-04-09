#! /usr/bin/bash

echo
echo "---------------------"
echo "starts install script"
echo "---------------------"
echo

sudo apt update && sudo apt -y upgrade
npm install npm@latest -g

git clone https://github.com/helix-editor/helix.git
cargo install --path ./helix/helix-term --locked
hx --grammar fetch
hx --grammar build
cp -r ./helix/runtime/queries ~/.config/helix/runtime/
cp -r ./helix/runtime/themes ~/.config/helix/runtime/
cp ./helix/runtime/tutor ~/.config/helix/runtime/
echo -e "[editor]\ntrue-color = true" > ~/.config/helix/config.toml
rm -f helix -R

cargo install bacon --locked
cargo install cargo-leptos --locked
cargo install leptosfmt --locked

rustup component add rust-analyzer
npm i -g vscode-langservers-extracted
rustup target add wasm32-unknown-unknown

echo
echo "---------------------"
echo "install script ends"
echo "---------------------"
echo
