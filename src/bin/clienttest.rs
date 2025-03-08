use beatsaver_api::{client::BeatSaverClient, models::map::Map};

async fn reality_check(client: &BeatSaverClient) -> anyhow::Result<Map> {
    let sans = client.map("25f").await;
    match sans {
        Ok(map) => Ok(map),
        Err(e) => Err(e.into()),
    }
}

#[tokio::main]
async fn main() {
    let client = BeatSaverClient::new();
    match reality_check(&client).await {
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
