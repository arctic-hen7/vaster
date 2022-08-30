use clap::Parser;

/// The Vaster command-line interface, for managing your Vast Zettelkasten with
/// ease.
#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(Parser)]
pub enum Subcommand {
    /// Get information about what a Vast Zettelkasten is, and what this tool
    /// is capable of
    Info,
    /// Cache your nodes to two files for interacting with them programmatically
    Cache,
}
