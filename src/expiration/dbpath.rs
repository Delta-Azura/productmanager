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
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

// ProjectDirs computes the OS-standard directories for this application.
// It returns an object holding SEVERAL distinct paths, one per purpose --
// they are NOT subfolders of a common parent:
//   config_dir()      ~/.config/ProductManager      Windows: AppData\Roaming
//   data_local_dir()  ~/.local/share/ProductManager Windows: AppData\Local
//   cache_dir()       ~/.cache/ProductManager       Windows: AppData\Local\...\cache
// The distinction matters on Windows domains: Roaming follows the user across
// machines, Local stays on this one. Config belongs in Roaming (small, worth
// keeping), the database in Local (must not be copied at every logon).
// These methods only build a path string from environment variables -- they
// never touch the disk, so the directory may not exist yet: create_dir_all
// is still required. The "config.txt" filename is our own convention, not
// something the crate knows about.


pub fn dbpath() -> Result<PathBuf> {
    let configdir = ProjectDirs::from("", "", "ProductManager").context("Unable to find configuration dir")?;
    let config_file = configdir.config_dir().join("config.txt");
    let content = if config_file.exists() {
        fs::read_to_string(&config_file).unwrap_or_default()
    } else {
        String::new()
    };
    let line = content.lines().nth(1).unwrap_or("").trim();
    if !line.is_empty() {
        return Ok(PathBuf::from(line));
    }
    let datadir = ProjectDirs::from("com", "PromoChecker", "PromoChecker").context("Unable to locate database")?;
    Ok(datadir.data_local_dir().join("database.db"))
}