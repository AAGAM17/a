mod commands;
mod registry;
mod lockfile;
mod manifest;
mod resolver;
mod cache;

#[cfg(test)]
mod tests {
    include!("../tests/basic_tests.rs");
}

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "A")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version)]
#[command(about = "A — blazing fast package manager", long_about = None)]
#[command(after_help = "For more information, visit: https://github.com/yourusername/a")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    Add { 
        package: String,
        #[arg(short, long)]
        dev: bool,
    },
    Install,
    Remove { 
        package: String,
        #[arg(short, long)]
        dev: bool,
    },
    Search { query: String },
    Init {
        name: Option<String>,
    },
    Run {
        script: String,
    },
    Update {
        package: Option<String>,
    },
    List,
    Cache {
        #[command(subcommand)]
        cmd: CacheCmd,
    },
}

#[derive(Subcommand)]
enum CacheCmd {
    Clean { 
        #[arg(short, long, default_value = "30")]
        days: u64,
    },
    List,
    Clear,
}

#[tokio::main]
async fn main() {
    // Print a nice welcome message
    if std::env::args().len() <= 1 {
        print_banner();
    }
    
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Setup logging based on verbose flag
    let verbose = cli.verbose;

    // Ensure cache directories exist
    if let Err(e) = cache::ensure_cache_dirs() {
        eprintln!("{}", format!("❌ Failed to initialize cache: {}", e).red());
        std::process::exit(1);
    }

    // Execute command
    let result = match cli.command {
        Commands::Add { package, dev } => commands::add(&package, dev).await,
        Commands::Install => commands::install().await,
        Commands::Remove { package, dev } => commands::remove(&package, dev).await,
        Commands::Search { query } => commands::search(&query).await,
        Commands::Init { name } => commands::init(name.as_deref()).await,
        Commands::Run { script } => commands::run_script(&script).await,
        Commands::Update { package } => commands::update(package.as_deref()).await,
        Commands::List => commands::list().await,
        Commands::Cache { cmd } => match cmd {
            CacheCmd::Clean { days } => cache::clean_cache(days),
            CacheCmd::List => commands::list_cache().await,
            CacheCmd::Clear => commands::clear_cache().await,
        },
    };

    // Handle errors
    if let Err(e) = result {
        if verbose {
            eprintln!("{}", format!("❌ Error: {:#?}", e).red());
        } else {
            eprintln!("{}", format!("❌ Error: {}", e).red());
            eprintln!("{}", "Run with --verbose for more details".yellow());
        }
        std::process::exit(1);
    }
}

fn print_banner() {
    println!("{}", r"
   ___ 
  / _ \ 
 / /_\ \
 |  _  |
 | | | |
 \_| |_/
    ".bright_blue());
    
    println!("{}", "A blazingly fast package manager".bright_green());
    println!("{}", "Version 0.1.0".yellow());
    println!("\nUsage: a <command> [options]");
    println!("\nRun 'a --help' for more information");
}
