use beatsaver_api::{client::BeatSaverClient, models::map::Map};

async fn get_map(client: &BeatSaverClient) -> anyhow::Result<Map> {
    let res = client.map("4bb13").await;
    match res {
        Ok(map) => Ok(map),
        Err(e) => Err(e.into()),
    }
}

#[tokio::main]
async fn main() {
    let client = BeatSaverClient::default();
    match get_map(&client).await {
        Ok(map) => print!("{:?}", map),
        Err(err) => {
            eprintln!("ERROR: {}", err);
            err.chain()
                .skip(1)
                .for_each(|cause| eprintln!("because: {:?}", cause));
            std::process::exit(1);
        }
    }
}
