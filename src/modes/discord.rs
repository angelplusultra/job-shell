use std::error::Error;

use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::{discord::initialize_discord_mode, error::AppResult};
pub async fn run() -> AppResult<()> {
    let dialoguer_styles = ColorfulTheme::default();

    let webhook_url = Input::<String>::with_theme(&dialoguer_styles)
        .with_prompt("Enter Discord webhook URL")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.starts_with("https://discord.com/api/webhooks/") {
                Ok(())
            } else {
                Err("Webhook URL must be a valid Discord webhook URL")
            }
        })
        .interact()?;

    let interval = Input::<u64>::with_theme(&dialoguer_styles)
        .with_prompt("Enter scan interval (hours)")
        .validate_with(|input: &u64| -> Result<(), &str> {
            if *input >= 1 && *input <= 12 {
                Ok(())
            } else {
                Err("Interval must be between 1 and 12 hours")
            }
        })
        .default(4)
        .interact()?;

    let scan_all_companies = Confirm::with_theme(&dialoguer_styles)
            .with_prompt("Scan all companies? (otherwise only followed companies or companies where you have at least 1 connection)")
            .default(false)
            .interact()?;

    initialize_discord_mode(webhook_url, interval, scan_all_companies).await?;

    return Ok(());
}
