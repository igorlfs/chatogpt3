// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    chatogpt3_lib::run();

    Ok(())
}
