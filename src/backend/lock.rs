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
// 

pub fn lock(product: &str) -> Result<()> {

    let lockfile = format!("{product}.lock")
    let db = ProjectDirs::from("com", "PromoChecker").expect("Unable to locate db")?;
    let folder = db.data_local_dir();
    let path = folder.join(lockfile);
    fs::create(path).context("Unable to create lock for this file")?;
    Ok()
}

pub fn unlock(product: &str) -> Result<()> {
    let lockfile = format!("{product}.lock")
    let db = ProjectDirs::from("com", "PromoChecker").expect("Unable to locate db")?;
    let folder = db.data_local_dir();
    let path = folder.join(lockfile);
    fs::remove_file(path).context("Unable to create lock for this file")?;
    Ok()
}



pub fn lockcheck(product: &str) -> Result<(bool)> {
    let lockfile = format!("{product}.lock")
    let db = ProjectDirs::from("com", "PromoChecker").expect("Unable to locate db")?;
    let folder = db.data_local_dir();
    let path = folder.join(lockfile);
    let mut lockstatus;
    if Path::new(path).exists() {
        lockstatus = true;
    } else {
        lockstatus = false;
    }
}