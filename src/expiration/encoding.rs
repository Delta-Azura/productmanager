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


use std::collections::HashMap;
use encoding_rs::WINDOWS_1252;
use anyhow::Result;
use anyhow::Context;



pub type Catalogue = HashMap<String, String>;

pub fn load(csv_path: &str) -> Result<Catalogue> {
    let mut map = HashMap::new();
    // defining the delimiters depending on each file 
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_path(csv_path)?;
    // cloning to keep the value without borrowing
    let headers = reader.byte_headers()?.clone();
    // decode: converts Latin-1 (Windows-1252) bytes into a UTF-8 String,
    // KEEPING the accented characters (é, è, à...) instead of mangling them.
    let decode = |b: &[u8]| WINDOWS_1252.decode(b).0.into_owned();
    // idx: finds a column by its name in the header and returns its position.
    // .trim() strips surrounding whitespace; eq_ignore_ascii_case = case-insensitive match.
    let idx = |n: &str| headers.iter().position(|h| decode(h).trim().eq_ignore_ascii_case(n));
    let c = idx("Code produit").expect("Code not found");
    let d = idx("Désignation").expect("Designation not found");
    for rec in reader.byte_records() {
        let rec = rec?;
        let code = decode(&rec[c]).trim().to_string();
        if !code.is_empty() {
            map.insert(code, decode(&rec[d]).trim().to_string());
        }
    }
    Ok(map)

}

pub fn compare(code: &str, products: &Catalogue) -> Result<String> {
    let name = products.get(code).context("Failed to get name for this code")?.clone();
    Ok(name)
}