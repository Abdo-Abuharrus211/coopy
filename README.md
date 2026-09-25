# Coopy
Building a tool to sync my blog content with my Obsidian vault. Selectively choosing which files to copy

# Todo
 - [X] Prompt for Paths
 - [X] Traverse all folders and their subfolders.
 - [X] parse files for frontmatter.
 - [X] Check if ready for publishing
 - [X] Copy the files to the destination directory
 - [X] Only copy files in specific folders
 - [X] Move User I/O to separate function
 - [X] Add command line arguments and flags
 - [X] Complete config implementation
 - [X] Update NOT Overwrite the `config.toml` file
 - [X] Write the new paths to the config file if user flags to save
 - [X] User message when config is updated - in `main.rs`
 - [ ] Write tests suite
 - [ ] Looping and Error handling for the path selection process
 - [ ] Add function doc strings? Is this idiomatic?
 - [X] Refactor and modularize the functions

# Functionality

# Commands And Options
- `sync`: synchronize files from source Obsidian vault to target folder (blog content)
- `add`: add a folder to the list of permitted or forbidden folders
- `remove`: remove a folder from the list of permitted or forbidden folders
- `set`: set the source or target path
- `config`: display the current configuration
- `help`: display help information for the commands and options## Grammar
```shell
coopy [SOURCE] [TARGET]                          # bare sync (implicit)
coopy sync [SOURCE] [TARGET] [--dry-run] [-v/--verbose]   # explicit sync + options
coopy add folders|forbidden <value>...
coopy remove [--alias: rmv] folders|forbidden <value>...
coopy set source|target <path>
coopy config
```
# Instructions
