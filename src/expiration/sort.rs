use anyhow::Result;
use rusqlite::Connection;

pub fn sort(db: &Connection) -> Result<Vec<(String, String, u32)>> {
    let mut list = db.prepare(
        "SELECT code, date, qt FROM produits ORDER BY date ASC"
    )?;
    // execute the prepared query, and for each row, read its columns
    // (code, date, qt) into a tuple
    let lines = list.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    })?;
    let mut data = Vec::new();
    for i in lines {
        data.push(i?);
    }
    Ok(data)
}