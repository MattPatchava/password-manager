use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine};
use clap::{Parser, Subcommand};
use directories_next::ProjectDirs;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::io::Write;

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
struct Store {
    meta: Meta,
    entries: std::collections::HashMap<String, Entry>,
}

#[derive(Serialize, Deserialize)]
struct Meta {
    salt: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    username: String,
    password: String,
    encrypted: bool,
}

fn load_store(file_path: &std::path::Path) -> Result<Store> {
    let store: Store = match std::fs::File::open(file_path) {
        Ok(file) => serde_json::from_reader(file)?,
        Err(_) => init_new_store()?,
    };

    Ok(store)
}

    let master_password: String = prompt_for_password()?;

fn init_new_store() -> Result<Store> {
    let salt: String = generate_salt();

    let store: Store = Store {
        meta: { Meta { salt } },
        entries: std::collections::HashMap::new(),
    };

    Ok(store)
}

fn generate_salt() -> String {
    let mut salt = [0u8; 32];
    let mut rng: OsRng = OsRng::default();

    rng.fill_bytes(&mut salt);

    general_purpose::STANDARD.encode(salt)
}

fn prompt_for_password() -> Result<String> {
    print!("Set new password: ");

    std::io::stdout().flush()?;

    let mut master_password: String = String::new();

    std::io::stdin().read_line(&mut master_password)?;

    Ok(master_password.trim().to_string())
}

fn add(
    username: String,
    password: String,
    encrypted: bool,
    file_path: &std::path::Path,
) -> Result<()> {
    let mut store: Store = load_store(&file_path)?;
    if store.entries.contains_key(&username) {
        return Err(anyhow::anyhow!("Username already exists"));
    }
    let username_clone: String = username.clone();

    if encrypted {
        println!("Adding encrypted entry:\n{}: {}", username, password);
        let master_password: String = prompt_for_password()?;
        let salt_bytes = general_purpose::STANDARD.decode(&store.meta.salt)?;
        let mut encryption_key: [u8; 32] = [0u8; 32];

        let argon2 = argon2::Argon2::default();

        argon2
            .hash_password_into(
                &master_password.as_bytes(),
                &salt_bytes,
                &mut encryption_key,
            )
            .map_err(|e| anyhow!(e))?;
        );
    } else {
        store.entries.insert(
            username_clone.clone(),
            Entry {
                username: username_clone,
                password,
                encrypted,
                salt: None,
            },
        );
    }

    let file: std::fs::File = std::fs::File::create(file_path)?;
    serde_json::to_writer_pretty(file, &store)?;

    println!("New entry added: {}", username);

    Ok(())
}

fn rm(username: String, file_path: &std::path::Path) -> Result<()> {
    let mut store: Store = load_store(&file_path)?;

    let key: Option<String> = store
        .entries
        .keys()
        .find(|k| k.to_lowercase() == username.to_lowercase())
        .cloned();

    match key {
        Some(k) => {
            store.entries.remove(&k);
            let file: std::fs::File = std::fs::File::create(&file_path)?;
            serde_json::to_writer_pretty(file, &store)?;
        }
        None => return Err(anyhow!("Username not found")),
    }

    println!("Removed entry for: {}", username);

    Ok(())
}

fn show(username: String, file_path: &std::path::Path) -> Result<()> {
    let store: Store = load_store(&file_path)?;

    let username_lower = username.to_lowercase();

    let entry: Option<(&String, &Entry)> = store
        .entries
        .iter()
        .find(|(k, _)| k.to_lowercase() == username_lower);

    match entry {
        Some((key, value)) => println!("{}: {}", key, value.password),
        None => println!("No entry found for username: {}", username),
    }

    Ok(())
}

fn list(file_path: &std::path::Path) -> Result<()> {
    let store: Store = load_store(file_path)?;

    if store.entries.is_empty() {
        println!("No entries found.");
    } else {
        println!(
            "
======================
    Saved Entries
======================\n"
        );
        for username in store.entries.keys() {
            println!("{}", username);
        }
        println!("\n=====================\n\nTo view a password, use the `show` command.");
    }

    Ok(())
}

fn main() -> Result<()> {
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
