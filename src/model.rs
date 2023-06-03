#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResolveVanityURL {
    pub steamid: Option<String>,
    pub message: Option<String>,
    pub success: u8,
}
#[derive(Debug, Deserialize)]
pub struct ResolveVanityURLResponse {
    pub response: ResolveVanityURL,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub steamid: String,
    pub communityvisibilitystate: u8,
    pub profilestate: u8,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub avatarhash: String,
    pub lastlogoff: u32,
    pub personastate: u8,
    pub realname: String,
    pub primaryclanid: String,
    pub timecreated: u32,
    pub personastateflags: u8,
    pub loccountrycode: String,
    pub locstatecode: String,
    pub games: Option<Vec<Game>>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummaries {
    pub players: Vec<Player>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummariesResponse {
    pub response: PlayerSummaries,
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub appid: u64,
    pub name: String,
    pub playtime_forever: u32,
    pub img_icon_url: String,
    pub rtime_last_played: u32,
    pub has_community_visible_stats: Option<bool>,
    pub has_leaderboards: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct OwnedGames {
    pub games: Vec<Game>,
    pub game_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct OwnedGamesResponse {
    pub response: OwnedGames,
}

#[derive(Debug, Deserialize)]
pub struct GameAchievements {
    pub name: String,
    pub displayName: String,
    pub hidden: u8,
    pub description: Option<String>,
    pub icon: String,
    pub icongray: String,
}

#[derive(Debug, Deserialize)]
pub struct GameStats {
    pub achievements: Vec<GameAchievements>,
}
#[derive(Debug, Deserialize)]
pub struct GameSchema {
    pub availableGameStats: GameStats,
}

#[derive(Debug, Deserialize)]
pub struct GameSchemaResponse {
    pub game: GameSchema,
}

#[derive(Debug, Deserialize)]
pub struct PlayerAchievements {
    pub apiname: String,
    pub achieved: u8,
    pub name: Option<String>,
    pub hidden: Option<u8>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub icongray: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStats {
    pub gameName: String,
    pub achievements: Vec<PlayerAchievements>,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct PlayerAchievementsResponse {
    pub playerstats: PlayerStats,
}
