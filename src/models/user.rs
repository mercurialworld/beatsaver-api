use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

use crate::models::enums::{AccountType, PatreonTier};

/// Information about a user on BeatSaver.
///
/// This is an alias for `UserDetail`.
pub type User = UserDetail;

/// Information about a user on BeatSaver.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    /// The numeric ID of a user.
    pub id: i32,

    /// The user's display name.
    pub name: String,

    /// The description the user set, if any.
    #[serde(default)]
    pub description: String,

    // /// TODO: figure this out -- when is this used?
    // #[serde(default)]
    // pub hash: Option<String>,

    // /// TODO: figure this out -- when is this used?
    // #[serde(default)]
    // pub testplay: bool,
    //
    /// The avatar URL.
    pub avatar: String,

    /// Average stats of the user's maps.
    pub stats: Option<UserStats>,

    // /// TODO: figure this out -- when is this used?
    // pub follow_data: Option<UserFollowData>,
    //
    /// The method this account was created.
    #[serde(rename = "type")]
    pub account_type: AccountType,

    // /// The email address associated with the account.
    // /// This isn't shown to anyone publicly, right?
    // #[serde(default)]
    // pub email: String,

    // /// The limit on maps the user is allowed to upload.
    // pub upload_limit: Option<i32>,

    // /// The limit on Vivify maps the user is allowed to upload.
    // pub vivify_limit: Option<i32>,
    //
    /// Whether the user is an admin or not.
    #[serde(default)]
    pub admin: bool,

    /// Whether the user is a curator or not.
    #[serde(default)]
    pub curator: bool,

    /// Whether the user is a senior curator or not.
    #[serde(default)]
    pub senior_curator: bool,

    /// Whether the user has curated maps or not.
    ///
    /// Shows as the "Curations" tab in the user page.
    #[serde(default)]
    pub curator_tab: bool,

    /// Whether the mapper is verified or not.
    #[serde(default)]
    pub verified_mapper: bool,

    /// The date the user was suspended at, if any.
    pub suspended_at: Option<DateTime<Utc>>,

    /// A playlist of the maps the user has made, in .bplist format, if any.
    pub playlist_url: Option<String>,

    /// The Patreon tier the user is, if any.
    pub patreon: Option<PatreonTier>,
    //
    // /// Whether the user wans to blur NSFW or not.
    // #[serde(default)]
    // pub blurnsfw: bool,
}

/// Stats of the user's maps, aggregated by BeatSaver.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    /// Average BPM of the songs a user maps.
    pub avg_bpm: f64,

    /// Average length of the songs a user maps.
    pub avg_duration: f64,

    /// Average score of the user's maps.
    pub avg_score: f64,

    /// How many of each difficulty the user has mapped.
    pub diff_stats: Option<UserDiffStats>,

    /// The date the user uploaded their first map, if any.
    pub first_upload: Option<DateTime<Utc>>,

    /// The date the user uploaded their most recent map, if any.
    pub last_upload: Option<DateTime<Utc>>,

    /// Number of ScoreSaber or BeatLeader ranked maps the user has made.
    pub ranked_maps: i32,

    /// Total number of downvotes on all the user's maps.
    pub total_downvotes: i32,

    /// Total number of maps the user has made.
    pub total_maps: i32,

    /// Total number of upvotes on all the user's maps.
    pub total_upvotes: i32,
}

/// Numbers of how many maps of each difficulty a user has made.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDiffStats {
    /// How many Easy maps the user has made.
    pub easy: i32,
    /// How many Normal maps the user has made.
    pub normal: i32,
    /// How many Hard maps the user has made.
    pub hard: i32,
    /// How many Expert maps the user has made.
    pub expert: i32,
    /// How many Expert+ maps the user has made.
    pub expert_plus: i32,
    /// Total number of maps the user has made.
    pub total: i32,
}

// #[derive(Debug, Deserialize)]
// pub struct UserFollowData {
//     pub collab: bool,
//     pub curation: bool,
//     pub followers: i32,
//     pub following: bool,
//     #[serde(default)]
//     pub follows: i32,
//     pub upload: bool,
// }
