use clap::{Parser, Subcommand};
use color_eyre::eyre;
use findex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds files to the index
    Add {
        #[arg(default_value = ".", help = "The path to recursively index")]
        path: String,
    },

    /// Dumps file information from the index
    Dump,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path } => findex::add(cli.verbose, path),
        Commands::Dump {} => findex::dump(cli.verbose),
    }
}
