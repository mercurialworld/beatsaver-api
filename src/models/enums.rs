use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Characteristic {
    Standard,
    OneSaber,
    NoArrows,
    #[serde(rename = "90Degree")]
    Rotation90Degrees,
    #[serde(rename = "360Degree")]
    Rotation360Degrees,
    Lightshow,
    Lawless,
    Legacy,
}

impl fmt::Display for Characteristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            Self::OneSaber => write!(f, "OneSaber"),
            Self::NoArrows => write!(f, "NoArrows"),
            Self::Rotation90Degrees => write!(f, "90Degrees"),
            Self::Rotation360Degrees => write!(f, "360Degrees"),
            Self::Lightshow => write!(f, "Lightshow"),
            Self::Lawless => write!(f, "Lawless"),
            Self::Legacy => write!(f, "Legacy"),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub enum BeatSaberEnvironment {
    // normal
    #[default]
    DefaultEnvironment,
    TriangleEnvironment,
    NiceEnvironment,
    BigMirrorEnvironment,
    KDAEnvironment,
    MonstercatEnvironment,
    CrabRaveEnvironment,
    DragonsEnvironment,
    OriginsEnvironment,
    PanicEnvironment,
    RocketEnvironment,
    GreenDayEnvironment,
    GreenDayGrenadeEnvironment,
    TimbalandEnvironment,
    FitBeatEnvironment,
    LinkinParkEnvironment,
    BTSEnvironment,
    KaleidoscopeEnvironment,
    InterscopeEnvironment,
    SkrillexEnvironment,
    BillieEnvironment,
    HalloweenEnvironment,
    GagaEnvironment,
    Halloween2Environment,

    // 360deg
    GlassDesertEnvironment,
    MultiplayerEnvironment,

    // GLS
    WeaveEnvironment,
    PyroEnvironment,
    EDMEnvironment,
    TheSecondEnvironment,
    LizzoEnvironment,
    TheWeekndEnvironment,
    RockMixtapeEnvironment,
    Dragons2Environment,
    Panic2Environment,
    QueenEnvironment,
    LinkinPark2Environment,
    TheRollingStonesEnvironment,
    LatticeEnvironment,
    DaftPunkEnvironment,
    HipHopEnvironment,
    ColliderEnvironment,
    BritneyEnvironment,
    Monstercat2Environment,
    MetallicaEnvironment,
    GridEnvironment,
    ColdplayEnvironment,
}

impl fmt::Display for BeatSaberEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // older
            Self::DefaultEnvironment => write!(f, "Default"),
            Self::TriangleEnvironment => write!(f, "Triangle"),
            Self::NiceEnvironment => write!(f, "Nice"),
            Self::BigMirrorEnvironment => write!(f, "Big Mirror"),
            Self::KDAEnvironment => write!(f, "KDA"),
            Self::MonstercatEnvironment => write!(f, "Monstercat"),
            Self::CrabRaveEnvironment => write!(f, "Crab Rave"),
            Self::DragonsEnvironment => write!(f, "Dragons"),
            Self::OriginsEnvironment => write!(f, "Origins"),
            Self::PanicEnvironment => write!(f, "Panic"),
            Self::RocketEnvironment => write!(f, "Rocket"),
            Self::GreenDayEnvironment => write!(f, "Green Day"),
            Self::GreenDayGrenadeEnvironment => write!(f, "Green Day Grenade"),
            Self::TimbalandEnvironment => write!(f, "Timbaland"),
            Self::FitBeatEnvironment => write!(f, "Fitbeat"),
            Self::LinkinParkEnvironment => write!(f, "Linkin Park"),
            Self::BTSEnvironment => write!(f, "BTS"),
            Self::KaleidoscopeEnvironment => write!(f, "Kaleidoscope"),
            Self::InterscopeEnvironment => write!(f, "Interscope"),
            Self::SkrillexEnvironment => write!(f, "Skrillex"),
            Self::BillieEnvironment => write!(f, "Billie"),
            Self::HalloweenEnvironment => write!(f, "Spooky"),
            Self::GagaEnvironment => write!(f, "Gaga"),
            Self::Halloween2Environment => write!(f, "Spoooky"),

            // 360deg / multiplayer
            Self::GlassDesertEnvironment => write!(f, "Glass Desert"),
            Self::MultiplayerEnvironment => write!(f, "Multiplayer"),

            // GLS
            Self::WeaveEnvironment => write!(f, "Weave"),
            Self::PyroEnvironment => write!(f, "Pyro"),
            Self::EDMEnvironment => write!(f, "EDM"),
            Self::TheSecondEnvironment => write!(f, "The Second"),
            Self::LizzoEnvironment => write!(f, "Lizzo"),
            Self::TheWeekndEnvironment => write!(f, "The Weeknd"),
            Self::RockMixtapeEnvironment => write!(f, "Rock Mixtape"),
            Self::Dragons2Environment => write!(f, "Dragons 2"),
            Self::Panic2Environment => write!(f, "Panic 2"),
            Self::QueenEnvironment => write!(f, "Queen"),
            Self::LinkinPark2Environment => write!(f, "Linkin Park 2"),
            Self::TheRollingStonesEnvironment => write!(f, "The Rolling Stones"),
            Self::LatticeEnvironment => write!(f, "Lattice"),
            Self::DaftPunkEnvironment => write!(f, "Daft Punk"),
            Self::HipHopEnvironment => write!(f, "HipHop"),
            Self::ColliderEnvironment => write!(f, "Collider"),
            Self::BritneyEnvironment => write!(f, "Britney"),
            Self::Monstercat2Environment => write!(f, "Monstercat 2"),
            Self::MetallicaEnvironment => write!(f, "Metallica"),
            Self::GridEnvironment => write!(f, "Cube"),
            Self::ColdplayEnvironment => write!(f, "Coldplay"),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum MapState {
    Uploaded,
    Testplay,
    Published,
    Feedback,
    Scheduled,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
    Discord,
    Simple,
    Dual,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum PatreonTier {
    None,
    Supporter,
    SupporterPlus,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserSentiment {
    #[default]
    Pending,
    VeryNegative,
    MostlyNegative,
    Mixed,
    MostlyPositive,
    VeryPositive,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum AIDeclarationType {
    Admin,
    Uploader,
    SageScore,
    None,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum MapTag {
    None,

    // Map types
    Tech,
    DanceStyle, // not to be confused with the genre dance
    Speed,
    Balanced,
    Challenge,
    Accuracy,
    Fitness,
    Poodle,

    // Song genres
    Swing,
    Nightcore,
    #[serde(rename = "folk-acoustic")]
    Folk,
    #[serde(rename = "kids-family")]
    Family,
    Ambient,
    #[serde(rename = "funk-disco")]
    Funk,
    Jazz,
    #[serde(rename = "classical-orchestral")]
    Classical,
    Soul,
    Speedcore,
    Punk,
    #[serde(rename = "rb")]
    RB,
    Holiday,
    Vocaloid,
    JRock,
    Trance,
    #[serde(rename = "drum-and-bass")]
    DrumBass,
    #[serde(rename = "comedy-meme")]
    Comedy,
    Instrumental,
    Hardcore,
    KPop,
    Indie,
    Techno,
    House,
    #[serde(rename = "video-game-soundtrack")]
    Game,
    #[serde(rename = "tv-movie-soundtrack")]
    Film,
    #[serde(rename = "alternative")]
    Alt,
    Dubstep,
    Metal,
    Anime,
    #[serde(rename = "hip-hop-rap")]
    HipHop,
    JPop,
    Dance,
    Rock,
    Pop,
    Electronic,
    #[serde(rename = "ai")]
    AI,
}

// ...there are THREE different ways to write all of these genres/tags
impl fmt::Display for MapTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapTag::None => write!(f, "N/A"),

            MapTag::Tech => write!(f, "Tech"),
            MapTag::DanceStyle => write!(f, "Dance"),
            MapTag::Speed => write!(f, "Speed"),
            MapTag::Balanced => write!(f, "Balanced"),
            MapTag::Challenge => write!(f, "Challenge"),
            MapTag::Accuracy => write!(f, "Accuracy"),
            MapTag::Fitness => write!(f, "Fitness"),
            MapTag::Poodle => write!(f, "Poodle"),

            MapTag::Swing => write!(f, "Swing"),
            MapTag::Nightcore => write!(f, "Nightcore"),
            MapTag::Folk => write!(f, "Folk & Acoustic"),
            MapTag::Family => write!(f, "Kids & Family"),
            MapTag::Ambient => write!(f, "Ambient"),
            MapTag::Funk => write!(f, "Funk & Disco"),
            MapTag::Jazz => write!(f, "Jazz"),
            MapTag::Classical => write!(f, "Classical & Orchestral"),
            MapTag::Soul => write!(f, "Soul"),
            MapTag::Speedcore => write!(f, "Speedcore"),
            MapTag::Punk => write!(f, "Punk"),
            MapTag::RB => write!(f, "R&B"),
            MapTag::Holiday => write!(f, "Holiday"),
            MapTag::Vocaloid => write!(f, "Vocaloid"),
            MapTag::JRock => write!(f, "J-Rock"),
            MapTag::Trance => write!(f, "Trance"),
            MapTag::DrumBass => write!(f, "Drum and Bass"),
            MapTag::Comedy => write!(f, "Comedy & Meme"),
            MapTag::Instrumental => write!(f, "Instrumental"),
            MapTag::Hardcore => write!(f, "Hardcore"),
            MapTag::KPop => write!(f, "K-Pop"),
            MapTag::Indie => write!(f, "Indie"),
            MapTag::Techno => write!(f, "Techno"),
            MapTag::House => write!(f, "House"),
            MapTag::Game => write!(f, "Video Game"),
            MapTag::Film => write!(f, "TV & Film"),
            MapTag::Alt => write!(f, "Alternative"),
            MapTag::Dubstep => write!(f, "Dubstep"),
            MapTag::Metal => write!(f, "Metal"),
            MapTag::Anime => write!(f, "Anime"),
            MapTag::HipHop => write!(f, "Hip Hop & Rap"),
            MapTag::JPop => write!(f, "J-Pop"),
            MapTag::Dance => write!(f, "Dance"),
            MapTag::Rock => write!(f, "Rock"),
            MapTag::Pop => write!(f, "Pop"),
            MapTag::Electronic => write!(f, "Electronic"),
            MapTag::AI => write!(f, "AI"),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum PlaylistType {
    Private,
    Public,
    System,
    Search,
}
