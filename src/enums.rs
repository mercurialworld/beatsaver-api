use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Characteristic {
    Standard,
    OneSaber,
    NoArrows,
    Rotation90Degrees,
    Rotation360Degrees,
    Lightshow,
    Lawless,
    Legacy,
}

impl Characteristic {
    pub fn name(&self) -> &'static str {
        match *self {
            Self::Standard => "Standard",
            Self::OneSaber => "OneSaber",
            Self::NoArrows => "NoArrows",
            Self::Rotation90Degrees => "90Degrees",
            Self::Rotation360Degrees => "360Degrees",
            Self::Lightshow => "Lightshow",
            Self::Lawless => "Lawless",
            Self::Legacy => "Legacy",
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum BeatSaberEnvironment {
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
    GlassDesertEnvironment,
    MultiplayerEnvironment,
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
}

impl BeatSaberEnvironment {
    pub fn name(&self) -> &'static str {
        match *self {
            Self::DefaultEnvironment => "Default",
            Self::TriangleEnvironment => "Triangle",
            Self::NiceEnvironment => "Nice",
            Self::BigMirrorEnvironment => "Big Mirror",
            Self::KDAEnvironment => "KDA",
            Self::MonstercatEnvironment => "Monstercat",
            Self::CrabRaveEnvironment => "Crab Rave",
            Self::DragonsEnvironment => "Dragons",
            Self::OriginsEnvironment => "Origins",
            Self::PanicEnvironment => "Panic",
            Self::RocketEnvironment => "Rocket",
            Self::GreenDayEnvironment => "Green Day",
            Self::GreenDayGrenadeEnvironment => "Green Day Grenade",
            Self::TimbalandEnvironment => "Timbaland",
            Self::FitBeatEnvironment => "Fitbeat",
            Self::LinkinParkEnvironment => "Linkin Park",
            Self::BTSEnvironment => "BTS",
            Self::KaleidoscopeEnvironment => "Kaleidoscope",
            Self::InterscopeEnvironment => "Interscope",
            Self::SkrillexEnvironment => "Skrillex",
            Self::BillieEnvironment => "Billie",
            Self::HalloweenEnvironment => "Halloween",
            Self::GagaEnvironment => "Gaga",
            Self::GlassDesertEnvironment => "Glass Desert",
            Self::MultiplayerEnvironment => "Multiplayer",
            Self::WeaveEnvironment => "Weave",
            Self::PyroEnvironment => "Pyro",
            Self::EDMEnvironment => "EDM",
            Self::TheSecondEnvironment => "The Second",
            Self::LizzoEnvironment => "Lizzo",
            Self::TheWeekndEnvironment => "The Weeknd",
            Self::RockMixtapeEnvironment => "Rock Mixtape",
            Self::Dragons2Environment => "Dragons 2",
            Self::Panic2Environment => "Panic 2",
            Self::QueenEnvironment => "Queen",
            Self::LinkinPark2Environment => "Linkin Park 2",
            Self::TheRollingStonesEnvironment => "The Rolling Stones",
            Self::LatticeEnvironment => "Lattice",
            Self::DaftPunkEnvironment => "Daft Punk",
            Self::HipHopEnvironment => "HipHop",
            Self::ColliderEnvironment => "Collider",
            Self::BritneyEnvironment => "Britney",
            Self::Monstercat2Environment => "Monstercat 2",
            Self::MetallicaEnvironment => "Metallica",
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum MapState {
    Uploaded,
    Testplay,
    Published,
    Feedback,
    Scheduled,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
    Discord,
    Simple,
    Dual,
}

#[derive(Debug, Deserialize)]
pub enum PatreonTier {
    None,
    Supporter,
    SupporterPlus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserSentiment {
    Pending,
    VeryNegative,
    MostlyNegative,
    Mixed,
    MostlyPositive,
    VeryPositive,
}

#[derive(Debug, Deserialize)]
pub enum AIDeclarationType {
    Admin,
    Uploader,
    SageScore,
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MapTag {
    None,
    Tech,
    DanceStyle,
    Speed,
    Balanced,
    Challenge,
    Accuracy,
    Fitness,
    Swing,
    Nightcore,
    Folk,
    Family,
    Ambient,
    Funk,
    Jazz,
    Classical,
    Soul,
    Speedcore,
    Punk,
    RB,
    Holiday,
    Vocaloid,
    JRock,
    Trance,
    DrumBass,
    Comedy,
    Instrumental,
    Hardcore,
    KPop,
    Indie,
    Techno,
    House,
    Game,
    Film,
    Alt,
    Dubstep,
    Metal,
    Anime,
    HipHop,
    JPop,
    Dance,
    Rock,
    Pop,
    Electronic,
}

#[derive(Debug, Deserialize)]
pub enum PlaylistType {
    Private,
    Public,
    System,
    Search,
}
