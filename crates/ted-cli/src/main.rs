//! Ted compiler command-line interface.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ted")]
#[command(about = "The Ted compiler - a timing-explicit language for hardware simulation")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Ted source file
    Compile {
        /// Input source file
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Check a Ted source file without generating output
    Check {
        /// Input source file
        input: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output } => {
            println!("Compiling: {}", input.display());
            if let Some(out) = output {
                println!("Output: {}", out.display());
            }

            match std::fs::read_to_string(&input) {
                Ok(source) => {
                    let source_id = ted_ir::SourceId(0);
                    let (module, diags) = ted_parser::parse_source(source_id, &source);

                    for diag in diags.iter() {
                        eprintln!("{:?}: {}", diag.severity, diag.message);
                    }

                    if let Some(m) = module {
                        println!("Parsed module: {}", m.name);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Check { input } => {
            println!("Checking: {}", input.display());

            match std::fs::read_to_string(&input) {
                Ok(source) => {
                    let source_id = ted_ir::SourceId(0);
                    let (_module, diags) = ted_parser::parse_source(source_id, &source);

                    for diag in diags.iter() {
                        eprintln!("{:?}: {}", diag.severity, diag.message);
                    }

                    if diags.has_errors() {
                        std::process::exit(1);
                    } else {
                        println!("No errors found.");
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
