use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// Path of vault to copy from
    #[arg(short, long)]
    pub(crate) source: Option<String>,
    /// Path to copy to
    #[arg(short, long)]
    pub(crate) target: Option<String>,

    /// Add folder name to a list (`folders` or `forbidden`)
    #[arg(short, long, action)]
    add: Option<String>,
    /// Delete folder from a list (`folders` or `forbidden`)
    #[arg(short, long, action)]
    del: Option<String>,
    ///Collection of folder names to check
    folders: Option<Vec<String>>,
    /// Collection of folder names to skip
    forbidden: Option<Vec<String>>,
}

impl Args {
    /// Process the 'add' and 'del' commands and their potential args 'folders' and 'forbidden'
    pub fn process_args(&self) {}
    // TODO: implement these functions to update the config file
    // fn add_folders() {}
    //
    // fn del_folders() {}
    //
    // fn add_forbidden() {}
    //
    // fn del_forbidden() {}
}

pub fn read_args() -> Args {
    Args::parse()
}
