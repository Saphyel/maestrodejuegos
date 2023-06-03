use std::env;

use ::clap::{Args, Parser, Subcommand};
use maestrodejuegos::{player_achievements, player_profile, vanity_url, HttpClient, SteamClient};

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "Maestro de juegos - Muestra logros de tus juegos",
    long_about = "Maestro de juegos te ayuda a descubrir que logros te faltan por seguir"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args)]
struct Usuario {
    // Returns Steam player id
    nombre: String,
}

#[derive(Args)]
struct Jugador {
    // Returns a profile from Steam
    steam_id: String,
}

#[derive(Args)]
struct Estado {
    // Returns a profile from Steam
    steam_id: String,
    game_id: String,
}

#[derive(Subcommand)]
enum Commands {
    Usuario(Usuario),
    Jugador(Jugador),
    Estado(Estado),
}

fn main() {
    let cli = Cli::parse();
    let client = SteamClient::build(
        String::from("https://api.steampowered.com"),
        env::var("STEAM_API_KEY").unwrap(),
    );
    match &cli.command {
        Some(Commands::Usuario(usuario)) => {
            let msg = vanity_url(&client, &usuario.nombre);
            println!("Steam ID: {:#?}", msg.unwrap());
        }
        Some(Commands::Jugador(jugador)) => {
            let msg = player_profile(&client, &jugador.steam_id);
            println!("Jugador: {:#?}", msg.unwrap());
        }
        Some(Commands::Estado(estado)) => {
            let msg = player_achievements(&client, &estado.steam_id, &estado.game_id);
            println!("Estado: {:#?}", msg.unwrap());
        }
        None => {}
    }
}
