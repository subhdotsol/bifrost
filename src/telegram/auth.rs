use grammers_client::Client;
use std::io::{self, BufRead, Write};

pub async fn authenticate(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("📱 Telegram Authentication");
    println!("──────────────────────────");

    // Get phone number
    print!("Enter phone number (with country code, e.g. +91...): ");
    io::stdout().flush()?;
    let phone = io::stdin().lock().lines().next().unwrap()?;

    // Request login code
    let token = client.request_login_code(&phone).await?;

    // Get OTP
    print!("Enter the OTP sent to your Telegram: ");
    io::stdout().flush()?;
    let code = io::stdin().lock().lines().next().unwrap()?;

    // Sign in
    match client.sign_in(&token, &code).await {
        Ok(_user) => {
            println!("✅ Logged in successfully!");
        }
        Err(grammers_client::SignInError::PasswordRequired(password_token)) => {
            // 2FA is enabled
            print!("Enter your 2FA password: ");
            io::stdout().flush()?;
            let password = io::stdin().lock().lines().next().unwrap()?;
            client.check_password(password_token, password).await?;
            println!("✅ Logged in with 2FA!");
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

pub fn prompt_for_credentials() -> (i32, String) {
    println!("🔑 Telegram API Credentials");
    println!("Get these from https://my.telegram.org");
    println!("──────────────────────────────────────");

    print!("API_ID: ");
    io::stdout().flush().unwrap();
    let api_id: i32 = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("API_ID must be a number");

    print!("API_HASH: ");
    io::stdout().flush().unwrap();
    let api_hash = io::stdin().lock().lines().next().unwrap().unwrap();

    (api_id, api_hash)
}
