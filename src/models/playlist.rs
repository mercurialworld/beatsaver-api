use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::enums::PlaylistType;

use super::{map::MapDetailWithOrder, user::UserDetail};

/// Information about a playlist on BeatSaver.
///
/// This is an alias for `PlaylistFull`.
pub type Playlist = PlaylistFull;

/// Information about a playlist on BeatSaver.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistFull {
    /// The date a playlist was created.
    pub created_at: DateTime<Utc>,

    /// The date a playlist was curated, if any.
    pub curated_at: Option<DateTime<Utc>>,

    /// The curator, if any.
    pub curator: Option<UserDetail>,

    /// The date a playlist was deleted at, if any.
    pub deleted_at: Option<DateTime<Utc>>,

    /// The description of a playlist, if any.
    pub description: Option<String>,

    /// The download URL for the playlist, in .bplist format.
    #[serde(rename = "downloadURL")]
    pub download_url: String,

    /// The name of the playlist.
    pub name: String,

    /// The creator of the playlist.
    pub owner: UserDetail,

    /// The numeric ID of the playlist on BeatSaver.
    pub playlist_id: i32,

    /// A URL of the playlist cover image.
    pub playlist_image: String,

    /// A URL of the playlist cover image, with 512 pixel width(?).
    pub playlist_image_512: String,

    /// The last time the playlist's maps were modified.
    pub songs_changed_at: Option<DateTime<Utc>>,

    /// General stats for the playlist.
    pub stats: PlaylistStats,

    /// The type of playlist.
    #[serde(rename = "type")]
    pub r#type: PlaylistType,

    /// The last time the playlist was modified.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Average statistics for a playlist on BeatSaver.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistStats {
    /// Average score of the playlist.
    pub avg_score: f64,

    /// Number of downvotes of all maps in the playlist.
    pub down_votes: i32,

    /// Total number of mappers of all maps in the playlist.
    pub mapper_count: i64,

    /// Maximum notes per second across all maps in the playlist.
    pub max_nps: f64,

    /// Minimum notes per second across all maps in the playlist.
    pub min_nps: f64,

    /// Total length of all songs in the playlist.
    pub total_duration: i32,

    /// Number of maps in the playlist.
    pub total_maps: i32,

    /// Number of upvotes of all maps in the playlist.
    pub up_votes: i32,
}

/// A page representing the playlist.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPage {
    /// A list of maps, with the total number fetched, on the playlist.
    pub maps: Vec<MapDetailWithOrder>,

    /// Information about the playlist.
    pub playlist: PlaylistFull,
}
