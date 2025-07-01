use aes_gcm::{aead::KeyInit, Aes256Gcm};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine};
use clap::{Parser, Subcommand};
use directories_next::ProjectDirs;

mod hashing;
use hashing::password::hash_password;
mod io;
use io::prompt_for_password;
mod store;
use store::load_store;
mod models;
use models::{Entry, Store};
mod crypto;
use crypto::password::{decrypt_password, encrypt_password};

#[derive(Parser)]
#[command(version, about = "A CLI password storage utility", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long, default_value_t = true)]
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
        let salt_bytes: Vec<u8> = general_purpose::STANDARD.decode(&store.meta.salt)?;
        let aes_encryption_key: [u8; 32] = hash_password(&master_password, salt_bytes)?;

        let cipher: Aes256Gcm =
            Aes256Gcm::new_from_slice(&aes_encryption_key).map_err(|e| anyhow!(e))?;

        let (ciphertext, nonce): (Vec<u8>, [u8; 12]) = encrypt_password(cipher, &password)?;

        store.entries.insert(
            username_clone.clone(),
            Entry {
                username: username_clone,
                password: general_purpose::STANDARD.encode(ciphertext),
                encrypted,
                nonce: Some(general_purpose::STANDARD.encode(nonce)),
            },
        );
    } else {
        store.entries.insert(
            username_clone.clone(),
            Entry {
                username: username_clone,
                password,
                encrypted,
                nonce: None,
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
        Some((key, value)) => {
            if value.encrypted {
                let master_password: String = prompt_for_password()?;
                let salt_bytes: Vec<u8> = general_purpose::STANDARD.decode(&store.meta.salt)?;
                let aes_encryption_key: [u8; 32] = hash_password(&master_password, salt_bytes)?;

                let cipher: Aes256Gcm =
                    Aes256Gcm::new_from_slice(&aes_encryption_key).map_err(|e| anyhow!(e))?;

                let nonce_b64: &str = value.nonce.as_ref().ok_or(anyhow!("Missing nonce"))?;

                let decrypted: String = decrypt_password(&cipher, nonce_b64, &value.password)?;

                println!(
                    "
======================
Username: {}
Password: {}
======================\n",
                    key, decrypted
                );
            } else {
                println!("{}: {}", key, value.password);
            }
        }
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
