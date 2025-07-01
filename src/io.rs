use anyhow::Result;
use std::io::Write;

fn prompt_password(prompt: &str, confirm: bool) -> Result<String> {
    print!("{}", prompt);
    std::io::stdout().flush()?;

    let mut password: String = String::new();
    std::io::stdin().read_line(&mut password)?;

    if confirm {
        print!("Confirm password: ");
        let mut password_confirm: String = String::new();

        std::io::stdin().read_line(&mut password_confirm)?;

        if password.trim() == password_confirm.trim() {
            return Ok(password.trim().to_string());
        } else {
            return Err(anyhow::anyhow!("Passwords did not match."));
        }
    }

    Ok(password.trim().to_string())
}

pub fn prompt_for_password() -> Result<String> {
    prompt_password("Enter password: ", false)
}

pub fn prompt_for_new_password() -> Result<String> {
    prompt_password("Set new password: ", true)
}
