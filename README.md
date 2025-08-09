# Coopy
A Command Line tool that syncs files between your Obsidian vault and your Digital Garden / blog's contents.

## Story
My [Digital Garden](https://garden.aabuharrus.dev/) required a tool to sync between my main Obsidian vault and the DG's content. 
**Coopy** scans MD files for frontmatter and other metadata to verify they're marked for publishing and then syncs the
content between vaults.<br>
I paired this with a Bash script to be able to run the entire process in a single command from my terminal.

The next step is to convert it into a proper Obsidian Plugin!

# Todo
 - [X] Prompt for Paths
 - [X] Traverse all folders and their sub folders.
 - [X] parse files for frontmatter.
 - [X] Check if ready for publishing
 - [X] Copy the files to the destination directory
 - [X] Only copy files in specific folders
 - [ ] Clean up the feedback to user
 - [ ] Refactor and modularize the functions


# Usage
## Saved settings mode
## Com