// Promochecker aims to be a graphical application to manage promotions of products
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


//#![windows_subsystem = "windows"]

use ProductManager::{opendb, sort, sortpromo};
use anyhow::{Context, Result};
use chrono::{Local, NaiveDate, Days};
use notify_rust::Notification;


pub fn  main() -> Result<()> {
    let conn = opendb().context("Failed to opendatabase")?;
    let list = sort(&conn).context("Failed to sort the database")?;
    let today = Local::now().date_naive();
    let limit = today + Days::new(30);
    for i in list {
        let (code, date, qt, _) = i;
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .context("Date not readable in database")?;
        if date <= limit {
            Notification::new()
                .summary("Produit périmé à retirer")
                .body(&format!("{}, {qt}", code))
                .show()?;
        }
    }
    let conn = opendb().context("Failed to opendatabase")?;
    let list = sortpromo(&conn).context("Failed to sort the database")?;
    let today = Local::now().date_naive();
    let limit = today + Days::new(1);
    for i in list {
        let (code, date, qt, _) = i;
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .context("Date not readable in database")?;
        if date <= limit {
            Notification::new()
                .summary("La promotion pour le produit suivant se termine demain")
                .body(&format!("{}", code))
                .show()?;
        }
    }

    Ok(())
}