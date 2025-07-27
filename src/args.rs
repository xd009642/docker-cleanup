use clap::{Parser, Subcommand};
use human_size::Size;
use jiff::Timestamp;
use std::str::FromStr;

/// CLI tool with ls, rm, and size commands.
#[derive(Parser, Debug)]
#[command(name = "cli-tool", version, about = "CLI for managing files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List files
    Ls(FilterOptions),
    /// Remove files
    Rm(FilterOptions),
    /// Show total size
    Size(FilterOptions),
}

impl Commands {
    pub fn filter(&self) -> &FilterOptions {
        match self {
            Self::Ls(f) => f,
            Self::Rm(f) => f,
            Self::Size(f) => f,
        }
    }
}

/// Common filter options for all commands
#[derive(Parser, Debug)]
pub struct FilterOptions {
    /// Only include files created before this timestamp
    #[arg(long)]
    pub created_before: Option<Timestamp>,
    /// Only include files created after this timestamp
    #[arg(long)]
    pub created_after: Option<Timestamp>,
    /// Only include files whose names match this regex pattern
    #[arg(long)]
    pub name_matches: Option<String>,
    /// Only include files with the following substring in the name
    #[arg(long)]
    pub name_contains: Option<String>,
    /// Only include files with the following repository name
    #[arg(long)]
    pub name: Option<String>,
    /// Only include files larger than this size in bytes
    #[arg(long, value_parser = parse_human_size)]
    pub larger_than: Option<usize>,
    /// Only include files smaller than this size in bytes
    #[arg(long, value_parser = parse_human_size)]
    pub smaller_than: Option<usize>,
    /// Doesn't do any operations like RM just lists the images
    #[arg(long)]
    pub dry_run: bool,
    /// Sort any printouts in order
    #[arg(long)]
    pub sort: bool,
}

fn parse_human_size(input: &str) -> Result<usize, String> {
    match Size::from_str(input) {
        Ok(size) => Ok(size.to_bytes() as usize),
        Err(_) => input
            .parse::<usize>()
            .map_err(|e| format!("Invalid size '{}': {}", input, e)),
    }
}
