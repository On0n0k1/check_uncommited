# check_uncommited
Repo that check all directories with uncommited git directories on given path.

## What it does:

Starting from the given path, it will look for a `Cargo.toml` file. If it finds a `Cargo.toml` file, it will attempt to run `git status`, then allocate the output to one of the expected categories. If it doesn't find a `Cargo.toml` file, it will recursively search all directories for directories with it and do the same.

## What is it for

I was about to format my computer before I reminded myself that I have a directory full of rust crates that I don't remember if I pushed the latest changes. Doing that manually was too annoying so I made this project for it.


## What is required

 - Git
 - Rust

How to run:

```bash
# Help
cargo run --release -- --help

# It will just find the current directory. This cargo toml with the current changes
cargo run --release

# Specify the path. Finding all rust crates of each specific category
cargo run --release -- --path /path/to/target/directory

# Will print the paths for each category found
cargo run --release -- --long --path /path/to/target/directory

# For debugging. It will print each status found. Very messy at the moment
cargo run --release -- --debug --path /path/to/target/directory

# I don't know why you would want to run this. Even messier
cargo run --release -- --debug --long --path /path/to/target/directory
```

## Disclaimer

I wrote this in an afternoon. It looks bad. I know. Feel free to send messages, suggestions, criticism... etc, as you please. I would be surprised if anyone at all cared about this crate more than I.

