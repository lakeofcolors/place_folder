use crate::cli::{Cli, Command, LsArgs, OpenArgs};
use crate::config::Config;
use anyhow::{Result, bail};
use std::env;
use std::process::Command as ShellCommand;
use std::fs;
use walkdir::WalkDir;


pub fn handle_command(cli: Cli) -> Result<()> {
    let mut config = Config::load()?;

    match cli.command {
        Command::Add { name, path } => {
            let real_path = if path == "." {
                env::current_dir()?.to_string_lossy().to_string()
            } else {
                path
            };
            if !fs::metadata(&real_path).is_ok() {
                println!("Warning: Path '{}' does not exist!", real_path);
            }
            config.projects.insert(name.clone(), real_path.clone());
            config.save()?;
            println!("\x1b[42mAdded project: {} ({})\x1b[0m", name, real_path);
        },
        Command::Rm { name } => {
            config.projects.remove(&name);
            config.set_favorite(&name, false);
            config.save()?;
            println!("\x1b[41mRemoved project: {}\x1b[0m", name);
        },
        Command::Ls(args) => {
            let mut projects: Vec<_> = config.projects.iter().collect();
            // Filter
            if let Some(filter) = &args.filter {
                let filter = filter.to_lowercase();
                projects.retain(|(n, p)| n.to_lowercase().contains(&filter) || p.to_lowercase().contains(&filter));
            }
            // Favorites
            if args.favorites {
                projects.retain(|(n, _)| config.is_favorite(n));
            }
            // Recent
            if args.recent {
                let recent_set: std::collections::HashSet<_> = config.recent.iter().collect();
                projects.retain(|(n, _)| recent_set.contains(n));
            }
            // Sort
            match args.sort.as_str() {
                "name" => projects.sort_by(|a, b| a.0.cmp(b.0)),
                "path" => projects.sort_by(|a, b| a.1.cmp(b.1)),
                _ => {}
            }
            println!("{:<2} {:<20} {:<}", "★", "PROJECT", "PATH");
            for (name, path) in projects {
                let fav = if config.is_favorite(name) {"*"} else {" "};
                println!("{:<2} {:<20} {:<}", fav, name, path);
            }
        },
        Command::Go { name } => {
            if let Some(path) = config.projects.get(&name).cloned() {
                config.goto_path = Some(path.clone());
                config.add_recent(&name);
                config.save()?;
                println!("\x1b[42mGo to dir: {}\x1b[0m", path);
            } else {
                bail!("Project '{}' not found", name);
            }
        },
        Command::Open(OpenArgs { name, editor }) => {
            if let Some(path) = config.projects.get(&name).cloned() {
                config.add_recent(&name);
                config.save()?;
                let status = ShellCommand::new(&editor)
                    .arg(&path)
                    .status()
                    .unwrap_or_else(|_| {
                        println!("Failed to launch '{}'. Is it installed?", editor);
                        std::process::exit(1);
                    });
                if status.success() {
                    println!("Opened '{}' in editor '{}'", name, editor);
                }
            } else {
                bail!("Project '{}' not found", name);
            }
        },
        Command::Rename { old_name, new_name } => {
            if let Some(path) = config.projects.remove(&old_name) {
                config.projects.insert(new_name.clone(), path);
                config.set_favorite(&old_name, false);
                config.save()?;
                println!("Renamed '{}' → '{}'", old_name, new_name);
            } else {
                bail!("Project '{}' not found", old_name);
            }
        },
        Command::Fav { name, unset } => {
            if config.projects.contains_key(&name) {
                config.set_favorite(&name, !unset);
                config.save()?;
                println!(
                    "{} favorite: {}",
                    if unset { "Unset" } else { "Set as" },
                    name
                );
            } else {
                bail!("Project '{}' not found", name);
            }
        },
        Command::Recent => {
            println!("{:<20} {:<}", "PROJECT", "PATH");
            for name in &config.recent {
                if let Some(path) = config.projects.get(name) {
                    println!("{:<20} {:<}", name, path);
                }
            }
        },
        Command::Export { file } => {
            let serialized = serde_json::to_string_pretty(&config)?;
            fs::write(&file, serialized)?;
            println!("Config exported to '{}'", file);
        },
        Command::Import { file } => {
            let data = fs::read_to_string(&file)?;
            let imported: Config = serde_json::from_str(&data)?;
            // merge projects
            for (k, v) in imported.projects {
                config.projects.insert(k, v);
            }
            for fav in imported.favorites {
                config.set_favorite(&fav, true);
            }
            for name in imported.recent {
                config.add_recent(&name);
            }
            config.save()?;
            println!("Config imported from '{}'", file);
        },
    }
    Ok(())
}
