#!/usr/bin/bash

cargo build --release
sudo cp target/release/gitignore /usr/local/bin/gitignore

echo "gitignore installed successfully"
echo "Run 'gitignore [template]' to use it"