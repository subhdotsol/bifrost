mod telegram;

use telegram::auth::{authenticate, prompt_for_credentials};
use telegram::client::TelegramClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    println!("╔═══════════════════════════════════╗");
    println!("║         Bifrost v0.1.0            ║");
    println!("║   Telegram TUI with Vim bindings  ║");
    println!("╚═══════════════════════════════════╝");
    println!();

    // Get API credentials (from env or prompt)
    let (api_id, api_hash) = match (
        std::env::var("TELEGRAM_API_ID"),
        std::env::var("TELEGRAM_API_HASH"),
    ) {
        (Ok(id), Ok(hash)) => (id.parse::<i32>().expect("Invalid API_ID"), hash),
        _ => prompt_for_credentials(),
    };

    // Connect to Telegram
    println!("🔌 Connecting to Telegram...");
    let tg = TelegramClient::connect(api_id, &api_hash).await?;
    println!("✅ Connected!");

    // Authenticate if needed
    if !tg.is_authorized().await? {
        authenticate(&tg.client).await?;
        tg.save_session()?;
        println!("💾 Session saved!");
    } else {
        println!("✅ Already logged in (session found)");
    }

    // Get and display user info
    let me = tg.client.get_me().await?;
    println!();
    println!("👤 Logged in as: @{}", me.username().unwrap_or("unknown"));
    println!();

    Ok(())
}
