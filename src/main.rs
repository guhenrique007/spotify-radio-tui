mod device;
mod playback;

use device::get_device_id;
use dotenv::dotenv;
use playback::start_playback_from_uris;
use rspotify::{
    model::{AdditionalType, Country, Device, EpisodeId, Market, TrackId},
    prelude::*,
    AuthCodeSpotify, Config, Credentials, OAuth,
};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct DevicePayload {
    pub devices: Vec<Device>,
}

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
    let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");

    let creds = Credentials::new(&client_id, &client_secret);

    let oauth = OAuth {
        redirect_uri: redirect_uri.to_string(),
        scopes: SCOPES.iter().map(|s| s.to_string()).collect(),
        ..Default::default()
    };

    let config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config.clone());
    let url = spotify.get_authorize_url(environment != "dev").unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    // private endpoints
    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(market), Some(&additional_types))
        .await;

    println!("Response: {artists:?}");

    let device_id = get_device_id(&spotify).await.unwrap();

    println!("Device ID: {:?}", device_id);

    let uris = [
        // PlayableId::Track(TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap()),
        PlayableId::Track(TrackId::from_uri("spotify:track:2DzSjFQKetFhkFCuDWhioi").unwrap()),
        PlayableId::Episode(EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap()),
    ];

    start_playback_from_uris(&spotify, uris.iter().map(PlayableId::as_ref), device_id).await;

    // spotify
    //     .start_uris_playback(
    //         uris.iter().map(PlayableId::as_ref),
    //         Some(&device_id),
    //         Some(Offset::Position(chrono::Duration::zero())),
    //         None,
    //     )
    //     .await
    //     .unwrap();
    //
    // let playback = spotify.current_playback(None, None::<&[_]>).await;
    //
    // println!("Response: {playback:?}");
    //
    // let resume = spotify.resume_playback(None, None).await;
    //
    // println!("Response: {resume:?}");
}
