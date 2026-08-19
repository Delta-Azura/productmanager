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

use promochecker::{opendb, writedb, sort, remove};
use iced::widget::{button, column, row, text, text_input, space};
use iced::Length;
use iced::{Element, Task};
use rusqlite::Connection;

struct App {
    conn: Connection,
    code: String,
    date: String, 
    qt: String,
    products: Vec<(String, String, u32, i64)>, 
    status: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Add,
    Remove(i64),
    //Remove(String),
    CodeChanged(String), 
    DateChanged(String),
    QtChanged(String),
}


impl App {
    pub fn new() -> (Self, Task<Message>) {
        let conn = opendb().expect("Impossible to open database");
        let products = sort(&conn).unwrap_or_default(); 
        let app = Self {
            conn,
            code: String::new(),
            date: String::new(),
            qt: String::new(),
            products,
            status: None,
        };
        (app, Task::none())
    
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CodeChanged(v) => self.code = v, 
            Message::DateChanged(v) => self.date = v,
            Message::QtChanged(v) => self.qt = v, 
            Message::Add => {
                if let Ok(qt) = self.qt.parse::<u32>() {
                    match writedb(&self.conn, &self.code, &self.date, qt) {
                        Ok(_) => {}
                        Err(e) => {
                            self.status = Some(format!("Quantité invalide {e:#}"));
                        }
                    }
                    match sort(&self.conn) {
                        Ok(list) => self.products = list,
                        Err(e) => self.status = Some(format!("Impossible d'écrire dans la base de donnée {e:#}")),

                    }
                    self.code.clear();
                    self.date.clear();
                    self.qt.clear();
                } else {
                    self.status = Some("Impossible de parser la quantité indiquée".to_string());
                }
            }
            Message::Remove(id) => { 
                match remove(&self.conn, id) {
                    Ok(_) => {}
                    Err(e) => {
                        self.status = Some(format!("Une erreur s'est produite: {e:#}"));
                    }                    
                } 
                match sort(&self.conn) {
                    Ok(list) => self.products = list,
                    Err(e) => self.status = Some(format!("Impossible de lire la base de donnée: {e}")),
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let input = row![
            text_input("Code", &self.code).on_input(Message::CodeChanged),
            text_input("Date", &self.date).on_input(Message::DateChanged),
            text_input("Quantité", &self.qt).on_input(Message::QtChanged),
            button("Ajouter").on_press(Message::Add),
        ]
        .spacing(10);
        let mut list = column![].spacing(20);
        for (code, date, qt, id) in &self.products {
            let line = row![
                text(format!("Id : {id} Code Produit : {code} Date : {date} x{qt}")).width(Length::Fill),
                button("Supprimer").on_press(Message::Remove(*id)),

            ].spacing(30);
            list = list.push(line);
        }
        let mut content = column![input, list].spacing(20).padding(20);
        if let Some(msg) = &self.status {
            content = content.push(
                text(msg).color(iced::Color::from_rgb(0.9, 0.2, 0.2))
            )
        }
        content.into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("PromoChecker")
        .run()
}
