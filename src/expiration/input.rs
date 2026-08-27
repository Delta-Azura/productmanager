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

use chrono::NaiveDate;
use anyhow::{Context, Result};


pub fn input(date: &str) -> Result<NaiveDate> {
    let dateinputs: Vec<&str> = date.split("/").collect();
    if dateinputs.len() != 3 {
        anyhow::bail!("Date format invalid");
    }
    let day: u32 = dateinputs[0].parse().context("Failed to get the day")?;
    let month: u32 = dateinputs[1].parse().context("Failed to get the month")?;
    let year: i32 = dateinputs[2].parse().context("Failed to get the year")?;
    let date = NaiveDate::from_ymd_opt(year, month, day).context("Date format isn't valid")?;
    Ok(date)
}




