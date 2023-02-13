use clap::{Parser, Subcommand};
use color_eyre::eyre;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds files to the index
    Add {
        #[arg(default_value = ".", help = "The path to recursively index")]
        path: String,

        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },

    /// Dumps file information from the index
    Dump {
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path, verbose } => findex::add(verbose, path),
        Commands::Dump { verbose } => findex::dump(verbose),
    }
}
