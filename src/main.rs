mod game_api;
mod game_constants;
mod game_engine;
mod game_entropy;
mod game_species;
mod game_universe;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use game_constants::UniversePlane;
use std::{
    sync::{mpsc::channel, Arc, RwLock},
    thread,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load Game Shared Data
    let mut universe = game_universe::Universe::new();
    game_species::load_plaintext_species(&mut universe);
    game_species::load_rle_species(&mut universe);
    let rwlock_root = Arc::new(RwLock::new(universe));

    // Start Game Engine
    let (sender, receiver) = channel::<UniversePlane>();

    let rwlock_engine = Arc::clone(&rwlock_root);
    match thread::Builder::new()
        .name("Game Engine".into())
        .spawn(move || {
            game_engine::engine_loop(rwlock_engine, &receiver);
        }) {
        Err(e) => {
            eprintln!("{:?}", e);
        }
        Ok(_) => {}
    }

    let rwlock_entropy = Arc::clone(&rwlock_root);
    match thread::Builder::new()
        .name("Game Entropy".into())
        .spawn(move || {
            game_entropy::entropy_loop(rwlock_entropy, &sender);
        }) {
        Err(e) => {
            eprintln!("{:?}", e);
        }
        Ok(_) => {}
    }

    // Start API Server
    let current_dir = std::env::current_dir()?;
    let current_path = std::path::Path::new(&current_dir).join("public");
    HttpServer::new(move || {
        let rwlock_app = Arc::clone(&rwlock_root);
        App::new()
            .app_data(web::Data::new(rwlock_app))
            .wrap(middleware::Compress::default())
            .service(game_api::gettext)
            .service(game_api::getimage)
            .service(game_api::getsvg)
            .service(fs::Files::new("/", &current_path).index_file("index.html"))
    })
    .bind((game_constants::API_ADDRESS, game_constants::API_PORT))?
    .run()
    .await
}
