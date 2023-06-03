mod model;

use model::{
    Game, GameAchievements, GameSchemaResponse, OwnedGamesResponse, Player,
    PlayerAchievementsResponse, PlayerStats, PlayerSummariesResponse, ResolveVanityURLResponse,
};
use reqwest::blocking::{Client, Response};
use std::io::{Error, ErrorKind};

pub trait HttpClient {
    fn build(base_url: String, secret: String) -> SteamClient;
    fn get<'a: 'c, 'c>(
        &'a self,
        endpoint: String,
        params: &mut Vec<(String, &'c String)>,
    ) -> Result<Response, reqwest::Error>;
}

pub struct SteamClient {
    base_url: String,
    secret: String,
    client: Client,
}

impl HttpClient for SteamClient {
    fn build(base_url: String, secret: String) -> SteamClient {
        let client = reqwest::blocking::Client::builder().build().unwrap();
        SteamClient {
            base_url,
            secret,
            client,
        }
    }
    fn get<'a: 'c, 'c>(
        &'a self,
        endpoint: String,
        params: &mut Vec<(String, &'c String)>,
    ) -> Result<Response, reqwest::Error> {
        params.push((String::from("key"), &self.secret));
        self.client
            .get(self.base_url.to_string() + &endpoint)
            .query(params)
            .send()
    }
}
pub fn vanity_url(client: &SteamClient, name: &String) -> Result<String, Error> {
    let mut params = Vec::new();
    params.push((String::from("vanityurl"), name));

    match client.get(String::from("/ISteamUser/ResolveVanityURL/v1"), &mut params) {
        Ok(response) => match response.json::<ResolveVanityURLResponse>() {
            Ok(json) => match json.response.success {
                1 => Ok(json.response.steamid.unwrap()),
                _ => Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Usuario {} no existe.", name),
                )),
            },
            Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Error in the request.")),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
}

fn player_summary(client: &SteamClient, steam_id: &String) -> Result<Player, Error> {
    let mut params = Vec::new();
    params.push((String::from("steamids"), steam_id));

    match client.get(
        String::from("/ISteamUser/GetPlayerSummaries/v2"),
        &mut params,
    ) {
        Ok(response) => match response.json::<PlayerSummariesResponse>() {
            Ok(json) => Ok(json.response.players.into_iter().next().unwrap()),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
}

fn owned_games(client: &SteamClient, steam_id: &String) -> Result<Vec<Game>, Error> {
    let mut params = Vec::new();
    let binding = String::from("1");
    params.push((String::from("steamid"), steam_id));
    params.push((String::from("include_appinfo"), &binding));

    match client.get(
        String::from("/IPlayerService/GetOwnedGames/v1"),
        &mut params,
    ) {
        Ok(response) => match response.json::<OwnedGamesResponse>() {
            Ok(json) => Ok(json.response.games),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
}
pub fn player_profile(client: &SteamClient, steam_id: &String) -> Result<Player, Error> {
    let mut summary = player_summary(client, steam_id)?;
    summary.games = Some(owned_games(client, steam_id)?);

    Ok(summary)
}

fn game_schema(client: &SteamClient, game_id: &String) -> Result<Vec<GameAchievements>, Error> {
    let mut params = Vec::new();
    params.push((String::from("appid"), game_id));

    match client.get(
        String::from("/ISteamUserStats/GetSchemaForGame/v2"),
        &mut params,
    ) {
        Ok(response) => match response.json::<GameSchemaResponse>() {
            Ok(json) => Ok(json.game.availableGameStats.achievements),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
}

fn achievements(
    client: &SteamClient,
    steam_id: &String,
    game_id: &String,
) -> Result<PlayerStats, Error> {
    let mut params = Vec::new();
    params.push((String::from("steamid"), steam_id));
    params.push((String::from("appid"), game_id));

    match client.get(
        String::from("/ISteamUserStats/GetPlayerAchievements/v1"),
        &mut params,
    ) {
        Ok(response) => match response.json::<PlayerAchievementsResponse>() {
            Ok(json) => Ok(json.playerstats),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
    // response.text().unwrap()
}

pub fn player_achievements(
    client: &SteamClient,
    steam_id: &String,
    game_id: &String,
) -> Result<PlayerStats, Error> {
    let mut stats = achievements(client, steam_id, game_id)?;
    let game = game_schema(client, game_id)?;
    let mut index = 0;
    while index < game.len() {
        if stats.achievements[index].apiname != game[index].name {
            return Err(Error::new(ErrorKind::InvalidInput, "Error in the request."))
        }
        
        stats.achievements[index].name = Some(game[index].displayName.to_string());
        stats.achievements[index].description = game[index].description.clone();
        stats.achievements[index].hidden = Some(game[index].hidden);
        stats.achievements[index].icon = Some(game[index].icon.to_string());
        stats.achievements[index].icongray = Some(game[index].icongray.to_string());

        index += 1;
    }

    Ok(stats)
}
