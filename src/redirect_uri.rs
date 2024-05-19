use rspotify::{oauth2::SpotifyOAuth, util::request_token};
use std::{
  io::prelude::*,
  net::{TcpListener, TcpStream},
};

pub fn redirect_uri_web_server(spotify_oauth: &mut SpotifyOAuth, port: u16) -> Result<String, ()> {
}
