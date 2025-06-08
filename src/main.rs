use clap::{Parser, Subcommand};
use directories_next::ProjectDirs;

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

fn add(
    encrypted: bool,
    username: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if encrypted {
        println!("Adding encrypted entry:\n{}: {}", username, password);
    } else {
        println!("Adding plaintext entry:\n{}: {}", username, password);
    }

    Ok(())
}

fn rm(username: String) {
    println!("Removing entry: {}", username);
}

fn show(username: String) {
    println!("Showing password for: {}", username);
}

fn list() {
    println!("Listing all passwords");
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

    let mut file: std::fs::File = std::fs::File::create(&config_path)?;

    // CLI Parsing

    let args: Args = Args::parse();

    match args.command {
        Commands::Add {
            encrypted,
            username,
            password,
        } => {
            if encrypted {
                add(true, username, password)
            } else {
                add(false, username, password)
            }
        }
        Commands::Rm { username } => {
            rm(username);
            Ok(())
        }
        Commands::Show { username } => {
            show(username);
            Ok(())
        }
        Commands::List => {
            list();
            Ok(())
        }
    }
}
