use rspotify::{model::Offset, prelude::*, AuthCodeSpotify};

pub async fn start_playback_from_uris<'a>(
    spotify: &AuthCodeSpotify,
    uris: impl IntoIterator<Item = PlayableId<'a>> + Send + 'a,
    device_id: String,
) {
    spotify
        .start_uris_playback(
            uris,
            Some(&device_id),
            Some(Offset::Position(chrono::Duration::zero())),
            None,
        )
        .await
        .unwrap();

    let playback = spotify.current_playback(None, None::<&[_]>).await;

    println!("Response: {playback:?}");

    let resume = spotify.resume_playback(None, None).await;

    println!("Response: {resume:?}");
}
