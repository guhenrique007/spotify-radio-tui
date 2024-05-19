use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use base64;
use dotenv::dotenv;

pub async fn authenticate_with_spotify() -> Result<String, Box<dyn std::error::Error>> {
    let client_id = std::env::var("SPOTIFY_CLIENT_ID")
        .expect("please set spotify_client_id");
    let client_secret = std::env::var("SPOTIFY_CLIENT_SECRET")
        .expect("please set spotify_client_secret");
    let auth = base64::encode(format!("{id}:{secret}",
                                      id = client_id,
                                      secret = client_secret
    ));
    let auth_url = "https://accounts.spotify.com/api/token";
    let auth_response = reqwest::Client::new()
        .post(auth_url)
        .header("Authorization", format!("Basic {auth}", auth = auth))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("grant_type=client_credentials")
        .send()
        .await?;

    let auth_body = auth_response.text().await?;
    let auth_token: serde_json::Value = serde_json::from_str(&auth_body)?;
    let token = auth_token["access_token"].as_str().unwrap();
    println!("Token: {}", token);

    Ok(token.to_string())
}
