// use reqwest;
// use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
// use serde::{Deserialize, Serialize};
use std::env;
// use base64;
use dotenv::dotenv;
use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};

const SCOPES: [&str; 14] = [
  "playlist-read-collaborative",
  "playlist-read-private",
  "playlist-modify-private",
  "playlist-modify-public",
  "user-follow-read",
  "user-follow-modify",
  "user-library-modify",
  "user-library-read",
  "user-modify-playback-state",
  "user-read-currently-playing",
  "user-read-playback-state",
  "user-read-playback-position",
  "user-read-private",
  "user-read-recently-played",
];

#[tokio::main]
async fn main() {
  dotenv().ok();
  let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
  let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
  let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

  let creds = Credentials::new(&client_id, &client_secret);

  let oauth = OAuth {
    redirect_uri: "http://localhost:8888/callback".to_string(),
    scopes: SCOPES.iter().map(|s| s.to_string()).collect(),
    ..Default::default()
  };

  let spotify = AuthCodeSpotify::new(creds, oauth);

  let url = spotify.get_authorize_url(false).unwrap();

  spotify.prompt_for_token(&url).await.unwrap();

  let market = Market::Country(Country::Spain);
  let additional_types = [AdditionalType::Episode];
  let artists = spotify
      .current_playing(Some(market), Some(&additional_types))
      .await;

  println!("Response: {artists:?}");
}
