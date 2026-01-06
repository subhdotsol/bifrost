use grammers_client::{Client, Config, InitParams};
use grammers_session::Session;
use std::path::Path;

const SESSION_FILE: &str = ".bifrost_session";

pub struct TelegramClient {
    pub client: Client,
}

impl TelegramClient {
    pub async fn connect(api_id: i32, api_hash: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let session = if Path::new(SESSION_FILE).exists() {
            Session::load_file(SESSION_FILE)?
        } else {
            Session::new()
        };

        let client = Client::connect(Config {
            session,
            api_id,
            api_hash: api_hash.to_string(),
            params: InitParams {
                ..Default::default()
            },
        })
        .await?;

        Ok(Self { client })
    }

    pub fn save_session(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.client.session().save();
        std::fs::write(SESSION_FILE, data)?;
        Ok(())
    }

    pub async fn is_authorized(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.client.is_authorized().await?)
    }
}
