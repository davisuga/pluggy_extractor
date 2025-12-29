//! This crate contains all shared fullstack server functions.
#[allow(unused_imports)]
use anyhow::Context;
use dioxus::prelude::*;

pub mod pluggy_sync;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}



#[post("/api/fetch-accounts")]
pub async fn fetch_accounts(item_id: String) -> Result<Vec<String>, ServerFnError> {
    let client_id = std::env::var("PLUGGY_CLIENT_ID")
        .context("PLUGGY_CLIENT_ID is required")?;
    let client_secret = std::env::var("PLUGGY_CLIENT_SECRET")
        .context("PLUGGY_CLIENT_SECRET is required")?;
    let client_id = uuid::Uuid::parse_str(&client_id)
        .context("PLUGGY_CLIENT_ID must be a UUID")?;

    let api_key = pluggy_sync::fetch_api_key(client_id, client_secret).await?;
    let config = pluggy_sync::with_api_key(api_key);

    let response = pluggy_rust::apis::account_api::accounts_list(
        &config,
        &item_id,
        None,
    )
    .await
    .map_err(anyhow::Error::from)?;

    let accounts = response
        .results
        .unwrap_or_default()
        .into_iter()
        .map(|account| account.id)
        .collect();

    Ok(accounts)
}
