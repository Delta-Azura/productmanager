

use anyhow::Result;
use rusqlite::Connection;

pub fn writedb(db: &Connection, code:&str, date:&str, qt:u32) -> Result<()> {
    db.execute(
        "CREATE TABLE IF NOT EXISTS produits (
            id   INTEGER PRIMARY KEY,
            code TEXT NOT NULL,
            date TEXT NOT NULL,
            qt   INTEGER NOT NULL,
            UNIQUE(code, date)
        )", 
        [],
    )?;
    db.execute(
        "INSERT INTO produits (code, date, qt) VALUES (?1, ?2, ?3)
        ON CONFLICT(code, data) DO UPDATE SET qt = qt + ?3",
        (code, date, qt),
    )?;
    Ok(())
}