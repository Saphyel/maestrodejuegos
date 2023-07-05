mod model;

use async_trait::async_trait;
use model::{
    Game, GameAchievements, GameSchemaResponse, OwnedGamesResponse, Player,
    PlayerAchievementsResponse, PlayerStats, PlayerSummariesResponse, ResolveVanityURLResponse,
};
use reqwest::{Client, Response};
use std::io::{Error, ErrorKind};

#[async_trait]
pub trait HttpClient {
    fn build(base_url: String, secret: String) -> SteamClient;
    async fn get<'a: 'c, 'c>(
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

#[async_trait]
impl HttpClient for SteamClient {
    fn build(base_url: String, secret: String) -> SteamClient {
        let client = reqwest::Client::builder().build().unwrap();
        SteamClient {
            base_url,
            secret,
            client,
        }
    }
    async fn get<'a: 'c, 'c>(
        &'a self,
        endpoint: String,
        params: &mut Vec<(String, &'c String)>,
    ) -> Result<Response, reqwest::Error> {
        params.push((String::from("key"), &self.secret));
        self.client
            .get(self.base_url.to_string() + &endpoint)
            .query(params)
            .send()
            .await
    }
}

pub async fn vanity_url(client: &SteamClient, name: &String) -> Result<String, Error> {
    let mut params = Vec::new();
    params.push((String::from("vanityurl"), name));

    match client
        .get(String::from("/ISteamUser/ResolveVanityURL/v1"), &mut params)
        .await
    {
        Ok(response) => match response.json::<ResolveVanityURLResponse>().await {
            Ok(json) => match json.response.steamid {
                Some(steamid) => Ok(steamid),
                None => Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Usuario {name} no existe."),
                )),
            },
            Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Error in the request.")),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
}

async fn player_summary(client: &SteamClient, steam_id: &String) -> Result<Player, Error> {
    let mut params = Vec::new();
    params.push((String::from("steamids"), steam_id));

    match client
        .get(
            String::from("/ISteamUser/GetPlayerSummaries/v2"),
            &mut params,
        )
        .await
    {
        Ok(response) => match response.json::<PlayerSummariesResponse>().await {
            Ok(json) => Ok(json.response.players.into_iter().next().unwrap()),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
}

async fn owned_games(client: &SteamClient, steam_id: &String) -> Result<Vec<Game>, Error> {
    let mut params = Vec::new();
    let binding = String::from("1");
    params.push((String::from("steamid"), steam_id));
    params.push((String::from("include_appinfo"), &binding));

    match client
        .get(
            String::from("/IPlayerService/GetOwnedGames/v1"),
            &mut params,
        )
        .await
    {
        Ok(response) => match response.json::<OwnedGamesResponse>().await {
            Ok(json) => Ok(json.response.games),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
}
pub async fn player_profile(client: &SteamClient, steam_id: &String) -> Result<Player, Error> {
    let mut summary = player_summary(client, steam_id).await?;
    summary.games = Some(owned_games(client, steam_id).await?);

    Ok(summary)
}

async fn game_schema(
    client: &SteamClient,
    game_id: &String,
) -> Result<Vec<GameAchievements>, Error> {
    let mut params = Vec::new();
    params.push((String::from("appid"), game_id));

    match client
        .get(
            String::from("/ISteamUserStats/GetSchemaForGame/v2"),
            &mut params,
        )
        .await
    {
        Ok(response) => match response.json::<GameSchemaResponse>().await {
            Ok(json) => Ok(json.game.availableGameStats.achievements),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
}

async fn achievements(
    client: &SteamClient,
    steam_id: &String,
    game_id: &String,
) -> Result<PlayerStats, Error> {
    let mut params = Vec::new();
    params.push((String::from("steamid"), steam_id));
    params.push((String::from("appid"), game_id));

    match client
        .get(
            String::from("/ISteamUserStats/GetPlayerAchievements/v1"),
            &mut params,
        )
        .await
    {
        Ok(response) => match response.json::<PlayerAchievementsResponse>().await {
            Ok(json) => Ok(json.playerstats),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        },
        Err(error) => Err(Error::new(ErrorKind::NotConnected, error.to_string())),
    }
    // response.text().unwrap()
}

pub async fn player_achievements(
    client: &SteamClient,
    steam_id: &String,
    game_id: &String,
) -> Result<PlayerStats, Error> {
    let mut stats = achievements(client, steam_id, game_id).await?;
    let game = game_schema(client, game_id).await?;
    let mut index = 0;
    while index < game.len() {
        if stats.achievements[index].apiname != game[index].name {
            return Err(Error::new(ErrorKind::InvalidInput, "Error in the request."));
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
