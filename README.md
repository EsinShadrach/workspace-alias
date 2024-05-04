## Workspace Alias

Creates aliases for work spaces for projects i.e for a NextJS project using bun the alias `bun run`, `bun start`, e.t.c are created

Supports flutter
Rust
JS packages based on package.json and package manager
init function to create alias-thing folder in root dir with

1. Config json
2. alias text

## TODO

- [x] Create Config file and json
- [x] Check if it exists
- [x] shell script or something to add the config.txt source to the ~/.zshrc or bashrc
- [x] Edit shell to include `alias-thing` to init alias-thing
- [x] Determine the repo environment and create commands based off that
- [x] Implement from feature for Alias struct
- [ ] Method to list available aliases
- [ ] command to run this binary
- [ ] add file extension prefix like .zsh or .bash to end of file

# Fixes

- Fix error in src/utils/get_alias_config.rs
