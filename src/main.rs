use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::SpotifyClientCredentials;
use rspotify::senum::{Country, SearchType};
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("client.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let client_credentials = json::parse(&contents).unwrap();
    let client_id = client_credentials["client_id"].as_str().unwrap();
    let client_secret = client_credentials["client_secret"].as_str().unwrap();
    let client_credentials = SpotifyClientCredentials::default()
        .client_id(client_id)
        .client_secret(client_secret)
        .build();

    let track_query = "blinding";
    let spotify = Spotify::default()
        .client_credentials_manager(client_credentials)
        .build();

    let result = spotify.search(
        track_query,
        SearchType::Track,
        2,
        0,
        Some(Country::UnitedStates),
        None,
    ).unwrap();

    println!("Search results for \"blinding\": {:?}", result);

    match result {
        rspotify::model::search::SearchResult::Tracks(tracks) => {
            let id = tracks.items[0].id.as_ref().unwrap();
            let analysis = spotify.audio_analysis(&id);
            println!("Analysis: {:?}", analysis);
        },
        _ => {
            eprintln!("Unexpected result!");
        },
    };

    Ok(())
}
