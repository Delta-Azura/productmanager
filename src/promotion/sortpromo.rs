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

use anyhow::Result;
use rusqlite::Connection;


pub fn sortpromo(db: &Connection) -> Result<Vec<(String, String, Option<u32>, i64)>> {
    let mut list = db.prepare(
        "SELECT code, date, qt, id FROM promotions ORDER BY date ASC"
    )?;
    // execute the prepared query, and for each row, read its columns
    // (code, date, qt) into a tuple
    let lines = list.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    })?;
    let mut data = Vec::new();
    for i in lines {
        data.push(i?);
    }
    Ok(data)
}

