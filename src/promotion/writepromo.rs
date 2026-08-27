// ProductManager aims to be a graphical application to manage promotions of products
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use crate::input;
use anyhow::{Result};
use rusqlite::Connection;

pub fn writepromo(db: &Connection, code: &str, date: &str, qt: Option<u32>) -> Result<()> {
    let date = input(date)?;
    if code.len() != 13 {
        anyhow::bail!("Le code barre est incorrect")
    }
    let quantity = qt.unwrap_or_else(|| 0);
    db.execute(
        "CREATE TABLE IF NOT EXISTS promotions (
            id   INTEGER PRIMARY KEY,
            code TEXT NOT NULL,
            date TEXT NOT NULL,
            qt   INTEGER,
            UNIQUE(code, date)
        )", 
        [],
    )?;
    db.execute(
        "INSERT INTO promotions (code, date, qt) VALUES (?1, ?2, ?3)
        ON CONFLICT(code, date) DO UPDATE SET qt = qt + ?3",
        (code, date, quantity),
    )?;
    Ok(())
}