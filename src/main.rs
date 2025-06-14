use clap::{Parser, Subcommand};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(version, about = "A CLI password storage utility", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        encrypted: bool,
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    Rm {
        #[arg(short, long)]
        username: String,
    },
    Show {
        #[arg(short, long)]
        username: String,
    },
    List,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    username: String,
    password: String,
    encrypted: bool,
}

fn add(
    username: String,
    password: String,
    encrypted: bool,
    file_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if encrypted {
        println!("Adding encrypted entry:\n{}: {}", username, password);
    } else {
        let mut store: std::collections::HashMap<String, Entry> =
            match std::fs::File::open(file_path) {
                Ok(file) => serde_json::from_reader(file)?,
                Err(_) => std::collections::HashMap::new(),
            };

        if store.contains_key(&username) {
            return Err("Username already exists".into());
        }

        let username_clone: String = username.clone();

        store.insert(
            username_clone.clone(),
            Entry {
                username: username_clone,
                password,
                encrypted,
            },
        );

        let file: std::fs::File = std::fs::File::create(file_path)?;
        serde_json::to_writer_pretty(file, &store)?;
    }

    println!("New entry added: {}", username);

    Ok(())
}

fn rm(username: String, file_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut store: std::collections::HashMap<String, Entry> = match std::fs::File::open(file_path) {
        Ok(file) => serde_json::from_reader(file)?,
        Err(_) => std::collections::HashMap::new(),
    };

    let key: Option<String> = store
        .keys()
        .find(|k| k.to_lowercase() == username.to_lowercase())
        .cloned();

    match key {
        Some(k) => {
            store.remove(&k);
            let file: std::fs::File = std::fs::File::create(file_path)?;
            serde_json::to_writer_pretty(file, &store)?;
        }
        None => return Err("Username not found".into()),
    }

    println!("Removed entry for: {}", username);

    Ok(())
}

fn show(username: String, file_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let store: std::collections::HashMap<String, Entry> = match std::fs::File::open(file_path) {
        Ok(file) => serde_json::from_reader(file)?,
        Err(_) => std::collections::HashMap::new(),
    };

    let username_lower = username.to_lowercase();

    let entry: Option<(&String, &Entry)> = store
        .iter()
        .find(|(k, _)| k.to_lowercase() == username_lower);

    match entry {
        Some((key, value)) => println!("{}: {}", key, value.password),
        None => println!("No entry found for username: {}", username),
    }

    Ok(())
}

fn list(file_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let store: std::collections::HashMap<String, Entry> = match std::fs::File::open(file_path) {
        Ok(file) => serde_json::from_reader(file)?,
        Err(_) => std::collections::HashMap::new(),
    };

    if store.is_empty() {
        println!("No entries found.");
    } else {
        println!(
            "
======================
    Saved Entries
======================\n"
        );
        for username in store.keys() {
            println!("{}", username);
        }
        println!("\n=====================\n\nTo view a password, use the `show` command.");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Config directory initialisation

    let project_dirs: ProjectDirs =
        match ProjectDirs::from("com", "mattpatchava", "password-manager") {
            Some(dirs) => dirs,
            None => panic!("Could not determine a valid directory for config storage"),
        };

    let config_dir: &std::path::Path = project_dirs.config_dir();
    let mut config_path: std::path::PathBuf = std::path::PathBuf::from(config_dir);

    std::fs::create_dir_all(&config_path)?;

    config_path.push("config.json");

    // CLI Parsing

    let args: Args = Args::parse();

    match args.command {
        Commands::Add {
            encrypted,
            username,
            password,
        } => add(username, password, encrypted, &config_path),
        Commands::Rm { username } => rm(username, &config_path),
        Commands::Show { username } => show(username, &config_path),
        Commands::List => list(&config_path),
    }
}
