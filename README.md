# aoc2025

## Installation

follow instructions here
https://rustup.rs/

then run these 2 commands
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu

in vscode install these extensions
"CodeLLDB" "Even Better TOML"
## Add a new day

cargo new --vcs none day01
then add it to the root Cargo.toml file in the members array like '"day01",'

## Run a day

cargo run -p day01

## Debug

Add this to launch.json

```json
{
    "name": "Debug executable 'day01'",
    "type": "lldb",
    "request": "launch",
    "cargo": {
        "args": [
            "run",
            "--bin=day01",
            "--package=day01"
        ]
    },
    "args": []
},

## Fetch instructions for a day to a txt file

Copy the session from a browser for Advent of Code
Then follow the instructions in the fetch_input.ps1 file in the root

## Help

run "rustup doc" that will open a browser with steps you can take to expand your knowledge with rust