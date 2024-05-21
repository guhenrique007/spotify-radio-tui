// use reqwest;
// use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use std::env;
// use base64;
// use chrono::Duration;
use dotenv::dotenv;
use rspotify::{
    model::{AdditionalType, Country, Device, EpisodeId, Market, Offset, TrackId},
    prelude::*,
    AuthCodeSpotify, Config, Credentials, OAuth,
};

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

fn extract_id(id_option: Option<String>) -> Option<String> {
    match id_option {
        Some(id) => Some(id),
        None => None, // You can handle this case differently if needed
    }
}

async fn get_device(spotify: &AuthCodeSpotify) -> Result<String, &'static str> {
    println!("Getting available devices");
    let devices = spotify.device().await;

    println!("Response: {devices:?}");

    if let Ok(ref device_list) = devices {
        if device_list.is_empty() {
            println!("Warning: No devices found");
            return Err("Warning: No devices found");
        }

        println!("Response 2: {:?}", device_list);

        // if let Some(second_device) = device_list.get(1) {
        //     println!("Second device: {:?}", second_device);
        //     println!("Second device name: {:?}", second_device.name);
        //     println!("Second device id: {:?}", second_device.id);
        //     return Ok(second_device.clone().id);
        // }

        if let Some(first_device) = device_list.first() {
            println!("First device: {:?}", first_device);
            println!("First device name: {:?}", first_device.name);
            println!("First device id: {:?}", first_device.id);

            if first_device.name.contains("Spotifyd") {
                if let Some(id) = extract_id(first_device.id.clone()) {
                    println!("ID: {}", id);

                    return Ok(id);
                }

                Err("No ID found")

                // Ok(first_device.clone().id)
            } else {
                println!("Warning: No Spotifyd devices found");
                Err("Warning: No Spotifyd devices found")
            }
        } else {
            println!("No devices found");
            Err("No devices found")
        }
    } else {
        println!("Failed to retrieve devices");
        Err("Failed to retrieve devices")
    }
}

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

    // let token = spotify.token.lock().await.unwrap();
    // println!("Access token: {}", &token.as_ref().unwrap().access_token);

    // private endpoints
    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(market), Some(&additional_types))
        .await;

    println!("Response: {artists:?}");

    let device_id = get_device(&spotify).await.unwrap();

    println!("Device ID: {:?}", device_id);

    let uris = [
        PlayableId::Track(TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap()),
        PlayableId::Track(TrackId::from_uri("spotify:track:2DzSjFQKetFhkFCuDWhioi").unwrap()),
        PlayableId::Episode(EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap()),
    ];

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

    // let resume = spotify.resume_playback(None, None).await;
    //
    // println!("Response: {resume:?}");
}
