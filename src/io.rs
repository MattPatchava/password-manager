use anyhow::Result;
use std::io::Write;

pub fn prompt_for_password() -> Result<String> {
    print!("Set new password: ");

    std::io::stdout().flush()?;

    let mut master_password: String = String::new();

    std::io::stdin().read_line(&mut master_password)?;

    Ok(master_password.trim().to_string())
}
