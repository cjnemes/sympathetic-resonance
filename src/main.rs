use clap::{Arg, Command};
use log::info;
use std::env;

mod core;
mod systems;
mod input;
mod content;
mod persistence;
mod ui;

use crate::core::game_engine::GameEngine;
use crate::persistence::database::DatabaseManager;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let matches = Command::new("Sympathetic Resonance")
        .version("0.1.0")
        .author("Adventure Game Team")
        .about("A text adventure game featuring science-based magic")
        .arg(
            Arg::new("init-db")
                .long("init-db")
                .help("Initialize the game database")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("save-file")
                .short('s')
                .long("save")
                .value_name("FILE")
                .help("Load a specific save file")
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Enable debug mode")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Initialize database
    let db_manager = DatabaseManager::new("content/database.db")?;

    if matches.get_flag("init-db") {
        info!("Initializing database...");
        db_manager.initialize_schema()?;
        db_manager.load_default_content()?;
        println!("Database initialized successfully!");
        return Ok(());
    }

    // Initialize game engine
    let mut game_engine = GameEngine::new(db_manager)?;

    // Load save file if specified
    if let Some(save_file) = matches.get_one::<String>("save-file") {
        info!("Loading save file: {}", save_file);
        game_engine.load_save(save_file)?;
    }

    // Set debug mode
    if matches.get_flag("debug") {
        game_engine.set_debug_mode(true);
    }

    println!("Welcome to Sympathetic Resonance!");
    println!("Type 'help' for available commands or 'quit' to exit.");
    println!();

    // Start main game loop
    game_engine.run()?;

    Ok(())
}