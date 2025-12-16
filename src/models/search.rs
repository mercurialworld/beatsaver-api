use serde::Deserialize;

use super::{
    map::{DeletedMap, MapDetail},
    playlist::PlaylistFull,
    user::UserDetail,
};

/// Results for searching up maps.
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    /// An array of maps relevant to the search criteria.
    pub docs: Vec<MapDetail>,

    /// Information about the search results.
    pub info: SearchInfo,

    /// The redirect URL, supposedly.
    pub redirect: Option<String>,
}

/// Stats and information about the search results.
#[derive(Debug, Deserialize)]
pub struct SearchInfo {
    /// How long the BeatSaver site took to fetch results.
    pub duration: f64,

    /// How many pages of results there are.
    pub pages: i32,

    /// The total number of objects that match the search criteria.
    pub total: i32,
}

/// Results of searching up deleted maps.
#[derive(Debug, Deserialize)]
pub struct DeletedResponse {
    /// A list of deleted maps.
    pub docs: Vec<DeletedMap>,
}

/// Results of searching up users.
#[derive(Debug, Deserialize)]
pub struct UserSearchResponse {
    /// A list of users that fit the search criteria.
    pub docs: Vec<UserDetail>,

    /// Information about the search results.
    pub info: SearchInfo,
}

/// Results of searching up playlists.
#[derive(Debug, Deserialize)]
pub struct PlaylistSearchResponse {
    /// A list of playlists that fit the search criteria.
    pub docs: Vec<PlaylistFull>,

    /// Information about the search results.
    pub info: SearchInfo,
}

/// Result of getting the /latest endpoint.
#[derive(Debug, Deserialize)]
pub struct LatestMapResponse {
    /// An array of maps relevant to the search criteria.
    pub docs: Vec<MapDetail>,
}
