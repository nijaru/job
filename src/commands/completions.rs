use anyhow::{Result, bail};
use clap::CommandFactory;
use clap_complete::Shell;
use std::fs;
use std::path::Path;

use crate::Cli;

pub fn execute(shell: Shell, install: bool) -> Result<()> {
    if install {
        install_completions(shell)
    } else {
        clap_complete::generate(shell, &mut Cli::command(), "jb", &mut std::io::stdout());
        Ok(())
    }
}

fn install_completions(shell: Shell) -> Result<()> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("could not find home directory"))?;

    let (path, content) = match shell {
        Shell::Zsh => {
            let dir = home.join(".zsh/completions");
            fs::create_dir_all(&dir)?;
            (dir.join("_jb"), generate_completions(shell))
        }
        Shell::Bash => {
            let dir = home.join(".local/share/bash-completion/completions");
            fs::create_dir_all(&dir)?;
            (dir.join("jb"), generate_completions(shell))
        }
        Shell::Fish => {
            let dir = home.join(".config/fish/completions");
            fs::create_dir_all(&dir)?;
            (dir.join("jb.fish"), generate_completions(shell))
        }
        _ => bail!("unsupported shell for --install: {shell}"),
    };

    fs::write(&path, content)?;
    println!("Installed completions to {}", path.display());
    print_activation_hint(shell, &path);

    Ok(())
}

fn generate_completions(shell: Shell) -> Vec<u8> {
    let mut buf = Vec::new();
    clap_complete::generate(shell, &mut Cli::command(), "jb", &mut buf);
    buf
}

fn print_activation_hint(shell: Shell, path: &Path) {
    match shell {
        Shell::Zsh => {
            println!("\nAdd to ~/.zshrc if not already present:");
            println!("  fpath=(~/.zsh/completions $fpath)");
            println!("  autoload -Uz compinit && compinit");
        }
        Shell::Bash => {
            println!("\nAdd to ~/.bashrc if not already present:");
            println!("  [[ -f {} ]] && source {}", path.display(), path.display());
        }
        Shell::Fish => {
            println!("\nCompletions will be automatically loaded by Fish.");
        }
        _ => {}
    }
}
