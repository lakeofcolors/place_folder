use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[command(name = "pf", version, author, about = "Project Folder Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a project: pf add <name> <path>
    Add { name: String, path: String },

    /// Remove a project: pf rm <name>
    Rm { name: String },

    /// List projects: pf ls [--sort name|path] [--filter <substr>] [--favorites] [--recent]
    Ls(LsArgs),

    /// Go to project: pf go <name>
    Go { name: String },

    /// Open project in editor: pf open <name> [--editor code]
    Open(OpenArgs),

    /// Rename project: pf rename <old_name> <new_name>
    Rename { old_name: String, new_name: String },

    /// Mark/unmark favorite: pf fav <name> [--unset]
    Fav { name: String, #[arg(long)] unset: bool },

    /// Show recent projects
    Recent,

    /// Backup/export config: pf export <file>
    Export { file: String },

    /// Import config: pf import <file>
    Import { file: String },

    /// Search и add all git rep in folder
    Scan { dir: String },

}

#[derive(Args, Debug)]
pub struct LsArgs {
    /// Sort by field: name or path
    #[arg(long, default_value = "name")]
    pub sort: String,

    /// Filter by substring
    #[arg(long)]
    pub filter: Option<String>,

    /// Show only favorites
    #[arg(long)]
    pub favorites: bool,

    /// Show only recent
    #[arg(long)]
    pub recent: bool,
}

#[derive(Args, Debug)]
pub struct OpenArgs {
    pub name: String,
    #[arg(long, default_value = "code")]
    pub editor: String,
}
