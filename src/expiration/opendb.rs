use anyhow::{Context, Result};
use rusqlite::Connection;
use directories::ProjectDirs;
use chrono::NaiveDate;
use std::fs;

pub fn opendb() -> Result<(Connection)> {
    let db = ProjectDirs::from("com", "PromoChecker", "PromoChecker").context("Unable to locate db")?;
    let folder = db.data_local_dir();
    fs::create_dir_all(folder)?
    let path = folder.join("database.db");
    let db = Connection::open(path)
    Ok(db)
    
}