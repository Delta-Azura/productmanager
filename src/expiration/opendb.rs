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

use anyhow::{Context, Result};
use rusqlite::Connection;
use directories::ProjectDirs;
use std::fs;

pub fn opendb() -> Result<Connection> {
    let db = ProjectDirs::from("com", "PromoChecker", "PromoChecker").context("Unable to locate db")?;
    let folder = db.data_local_dir();
    fs::create_dir_all(folder)?;
    let path = folder.join("database.db");
    let db = Connection::open(path)?;
    Ok(db)
    
}