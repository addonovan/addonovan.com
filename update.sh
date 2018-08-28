#!/bin/bash
set -e

# Get the binary's version before and after a pull 
old_version="$(cat Cargo.toml | grep "^version" | cut -d ' ' -f 3)"
git pull
new_version="$(cat Cargo.toml | grep "^version" | cut -d ' ' -f 3)"

# Only rebuild the server if there's a change in the binary
if [[ "$old_version" = "$new_version" ]]; then
    echo "Website up to date."
else
    echo "Core updated: $old_version => $new_version"
    cargo build --release
    echo "Website up to date."

    echo "Update complete, restarting process"
    killall website
    tmux new-session -c "$(pwd)" -d "./target/release/website"
fi

