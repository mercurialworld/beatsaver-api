use reqwest::Client;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fmt::Debug;
use thiserror::Error;
use url::Url;

use crate::models::{
    map::Map,
    playlist::PlaylistPage,
    search::{PlaylistSearchResponse, SearchResponse},
    user::User,
};

const BASE_URL: &str = "https://api.beatsaver.com/";

// TODO: switch to thiserror
#[derive(Error)]
pub enum ClientError {
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error("URL error")]
    URLError(#[from] url::ParseError),
}

impl Debug for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}

type AnyResult<T> = Result<T, ClientError>;

pub struct BeatSaverClient {
    client: Client,
}

impl Default for BeatSaverClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BeatSaverClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        Self { client }
    }

    pub async fn get_endpoint<T>(&self, endpoint: &str) -> AnyResult<T>
    where
        T: DeserializeOwned,
    {
        let url = Url::parse(BASE_URL)
            .map_err(ClientError::URLError)?
            .join(endpoint);

        let res = self
            .client
            .get(url?.as_str())
            .send()
            .await
            .map_err(ClientError::ReqwestError)?
            .text()
            .await?;

        serde_json::from_str::<T>(&res).map_err(ClientError::SerdeError)
    }

    /// Get map information given an ID.
    pub async fn map(&self, id: &str) -> AnyResult<Map> {
        self.get_endpoint(&format!("maps/id/{id}")).await
    }

    /// Get user information given an ID.
    pub async fn user(&self, id: &str) -> AnyResult<User> {
        self.get_endpoint(&format!("users/id/{id}")).await
    }

    /// Get user information given a name.
    pub async fn user_from_name(&self, name: &str) -> AnyResult<User> {
        self.get_endpoint(&format!("users/name/{name}")).await
    }

    /// Get playlist information given an ID.
    pub async fn playlist_info(&self, id: &str) -> AnyResult<PlaylistPage> {
        self.get_endpoint(&format!("playlists/id/{id}/0")).await
    }
    /// Get a list of playlists matching search criteria.
    pub async fn search_playlists(&self, query: &str) -> AnyResult<PlaylistSearchResponse> {
        self.get_endpoint(&format!("playlists/search/0?q={query}"))
            .await
    }

    /// Get a list of maps matching search criteria.
    /// TODO: add default parameters when i'm not sleepy
    pub async fn search_maps(&self, query: &str) -> AnyResult<SearchResponse> {
        self.get_endpoint(&format!("search/text/0?pageSize=25&q={query}"))
            .await
    }
}
