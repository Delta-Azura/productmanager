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

use std::fs;
use anyhow::{Result};
use crate::expiration::encoding::Catalogue;
use crate::compare;

pub fn html(products: &[(String, String, u32, i64)], catalogue: &Catalogue) -> Result<()> {
    let mut html = String::from(
        "<html><head><meta charset='utf-8'><style>
         body { font-family: sans-serif; }
         table { border-collapse: collapse; width: 100%; }
         th, td { border: 1px solid #333; padding: 8px; text-align: left; }
         th { background: #eee; }
         </style></head><body>"
    );
    html.push_str("<h1>Listing des péremptions</h1>");
    html.push_str("<table><tr><th>Dénomination</th><th>Code</th><th>Date</th><th>Quantité</th></tr>");
    for (code, date, qt, _id) in products {
        let name = compare(code, catalogue).unwrap_or_else(|_| code.to_string());
        html.push_str(&format!(
            "<tr><td>{name}</td><td>{code}</td><td>{date}</td><td>{qt}</td></tr>"
        ));
    }
    html.push_str("</table></body></html>");
    let path = std::env::temp_dir().join("listing.html");
    fs::write(&path, html)?;
    open::that(path)?;
    Ok(())
}