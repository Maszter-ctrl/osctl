mod config;
use config::load_config;

mod resources;
mod system;
use system::SystemState;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "osctl")]
#[command(about = "Declarative OS configuration tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply configuration to the system
    Apply {
        /// Show what would change without applying
        #[arg(long)]
        dry_run: bool,
    },

    /// Show differences between desired and current state
    Diff,
}

fn main() {
    // Parse CLI args
    let cli = Cli::parse();

    // Initialize system state
    let system = SystemState::new();
    println!("Current system state:");
    println!("Installed packages: {:?}", system.installed_packages);
    println!("Enabled services: {:?}", system.enabled_services);

    // Match commands
    match cli.command {
        Commands::Apply { dry_run } => {
            let cfg = load_config("os.yaml");
            println!("Loaded config:");
            println!("{:#?}", cfg);

            println!("Diff between desired and current state:");
            system.diff(&cfg);

            // Apply changes safely
            system.apply(&cfg, dry_run);

            if dry_run {
                println!("Dry run complete — no changes made");
            } else {
                println!("Changes applied!");
            }
        }

        Commands::Diff => {
            let cfg = load_config("os.yaml");
            println!("Diff:");
            system.diff(&cfg);
        }
    }
}
