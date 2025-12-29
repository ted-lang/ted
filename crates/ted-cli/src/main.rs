//! Ted compiler command-line interface.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "ted")]
#[command(about = "The Ted compiler - a systems language with explicit logical time")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Ted source file to an object file or executable
    Compile {
        /// Input source file
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format
        #[arg(long, value_enum, default_value = "exe")]
        emit: EmitKind,

        /// Optimization level
        #[arg(long, value_enum, default_value = "none")]
        opt_level: OptLevelArg,

        /// Target triple (defaults to the host target)
        #[arg(long)]
        target: Option<String>,
    },

    /// Check a Ted source file without generating output
    Check {
        /// Input source file
        input: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum EmitKind {
    Obj,
    Exe,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OptLevelArg {
    None,
    Speed,
    SpeedAndSize,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            output,
            emit,
            opt_level,
            target,
        } => {
            println!("Compiling: {}", input.display());
            let source_unit = match read_and_parse(&input) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    std::process::exit(1);
                }
            };

            for diag in source_unit.diagnostics.iter() {
                eprintln!("{:?}: {}", diag.severity, diag.message);
            }

            if source_unit.diagnostics.has_errors() {
                std::process::exit(1);
            }

            if source_unit.module.is_none() {
                eprintln!("Warning: parser produced no module; using prototype frontend.");
            }

            let output_path = output.unwrap_or_else(|| default_output_path(&input, emit));
            println!("Output: {}", output_path.display());

            let program = match ted_codegen::Program::from_source(&source_unit.source) {
                Ok(program) => program,
                Err(e) => {
                    eprintln!("Prototype frontend failed: {}", e);
                    std::process::exit(1);
                }
            };
            if matches!(emit, EmitKind::Exe) && target.is_some() {
                eprintln!("Cross-linking is not supported yet; use --emit obj.");
                std::process::exit(1);
            }

            let config = ted_codegen::CodegenConfig {
                opt_level: map_opt_level(opt_level),
                target,
            };

            let backend = ted_codegen::cranelift::CraneliftBackend::new(config);
            let object = match backend.compile_object(&program) {
                Ok(object) => object,
                Err(e) => {
                    eprintln!("Codegen failed: {}", e);
                    std::process::exit(1);
                }
            };

            if let Err(e) = write_object_and_maybe_link(&object.bytes, &output_path, emit) {
                eprintln!("Codegen failed: {}", e);
                std::process::exit(1);
            }

            eprintln!("Note: prototype backend only supports literal print calls right now.");
        }
        Commands::Check { input } => {
            println!("Checking: {}", input.display());

            let source_unit = match read_and_parse(&input) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    std::process::exit(1);
                }
            };

            for diag in source_unit.diagnostics.iter() {
                eprintln!("{:?}: {}", diag.severity, diag.message);
            }

            if source_unit.diagnostics.has_errors() {
                std::process::exit(1);
            } else {
                println!("No errors found.");
            }
        }
    }
}

struct SourceUnit {
    source: String,
    module: Option<ted_parser::Module>,
    diagnostics: ted_diagnostics::Diagnostics,
}

fn read_and_parse(input: &Path) -> Result<SourceUnit, std::io::Error> {
    let source = std::fs::read_to_string(input)?;
    let source_id = ted_ir::SourceId(0);
    let (module, diags) = ted_parser::parse_source(source_id, &source);
    Ok(SourceUnit {
        source,
        module,
        diagnostics: diags,
    })
}

fn map_opt_level(level: OptLevelArg) -> ted_codegen::OptLevel {
    match level {
        OptLevelArg::None => ted_codegen::OptLevel::None,
        OptLevelArg::Speed => ted_codegen::OptLevel::Speed,
        OptLevelArg::SpeedAndSize => ted_codegen::OptLevel::SpeedAndSize,
    }
}

fn default_output_path(input: &Path, emit: EmitKind) -> PathBuf {
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("a.out");
    match emit {
        EmitKind::Obj => PathBuf::from(format!("{stem}.o")),
        EmitKind::Exe => PathBuf::from(stem),
    }
}

fn write_object_and_maybe_link(object: &[u8], output: &Path, emit: EmitKind) -> Result<(), String> {
    match emit {
        EmitKind::Obj => std::fs::write(output, object).map_err(|e| e.to_string())?,
        EmitKind::Exe => {
            let obj_path = temp_object_path(output);
            std::fs::write(&obj_path, object).map_err(|e| e.to_string())?;
            let link_result = link_executable(&obj_path, output);
            let _ = std::fs::remove_file(&obj_path);
            link_result?;
        }
    }
    Ok(())
}

fn temp_object_path(output: &Path) -> PathBuf {
    let stem = output.file_stem().and_then(|s| s.to_str()).unwrap_or("ted");
    let filename = format!("{stem}-{}.o", std::process::id());
    std::env::temp_dir().join(filename)
}

fn link_executable(object: &Path, output: &Path) -> Result<(), String> {
    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());
    let status = Command::new(&cc)
        .arg(object)
        .arg("-o")
        .arg(output)
        .status()
        .map_err(|e| format!("failed to run linker `{cc}`: {e}"))?;

    if !status.success() {
        return Err(format!(
            "linker `{cc}` failed (exit code: {:?}); try --emit obj",
            status.code()
        ));
    }

    Ok(())
}
