use clap::Parser;
use pvm::commands;
use pvm::constants::get_venv_path;
use std::path::Path;

#[derive(Parser)]
#[clap(
name = "pvm",
version,
author,
about = "Python Virtual env Manager",
long_about = None,
)]
enum Cli {
    /// Create a new env
    Add {
        #[arg(help = "env name")]
        name: String,
        #[arg(short, long, help = "Python version")]
        version: Option<String>,
        #[arg(
            short,
            long,
            default_value_t = false,
            help = "overwrite an existing env"
        )]
        force: bool,
    },
    /// List all envs
    #[clap(alias = "ls")]
    List,
    /// Remove an existing env
    #[clap(alias = "rm")]
    Remove {
        #[clap(required = true, help = "env name(s)")]
        name: Vec<String>,
    },
    /// Activate an existing env
    Use {
        #[arg(help = "env name")]
        name: String,
        #[arg(short, long, default_value_t = false, help = "use PowerShell v7+")]
        pwsh: bool,
    },
}

fn main() {
    let tmp = match get_venv_path() {
        Some(val) => val,
        None => {
            return;
        }
    };
    let venv_path = Path::new(&tmp);
    if !venv_path.exists() {
        return;
    }

    match Cli::parse() {
        Cli::Add {
            name,
            version,
            force,
        } => commands::create(venv_path, &name, version, force),
        Cli::List => commands::list(venv_path),
        Cli::Remove { name: names } => {
            for name in names {
                commands::remove(venv_path, &name)
            }
        }
        Cli::Use { name, pwsh } => commands::activate(venv_path, &name, pwsh),
    };
}
