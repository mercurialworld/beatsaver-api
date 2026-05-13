use core::panic;

use beatsaver_api::{client::BeatSaverClient, models::map::Map};

use crate::common::setup;

mod common;

async fn get_map(client: &BeatSaverClient, key: &str) -> anyhow::Result<Map> {
    let res = client.map(key).await;
    match res {
        Ok(map) => Ok(map),
        Err(e) => Err(e.into()),
    }
}

#[tokio::test]
async fn it_gets_focus_by_jonas() {
    let client = setup();

    let map = get_map(&client, "4b6f1").await;
    match map {
        Ok(focus) => {
            assert_eq!(focus.metadata.song_name.expect("no title??"), "FOCUS");
        }
        Err(focusnt) => {
            let mut panic_message = format!("ERROR: {}", focusnt);

            focusnt
                .chain()
                .skip(1)
                .for_each(|cause| panic_message.push_str(&format!("\nbecause: {:?}", cause)));

            panic!("{}", panic_message)
        }
    }
}
