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

use promochecker::{opendb, writedb, sort, input};
use iced::widget::{button, column, row, text, text_input};
use iced::{Element, Task};
use rusqlite::Connection;

struct App {
    conn: Connection,
    code: String,
    date: String, 
    qt: String,
    products: Vec<(String, String, u32)>  
}

#[derive(Debug, Clone)]
pub enum Message {
    Add,
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
                        Ok(_) => {println!("ok");}
                        Err(e) => eprintln!("{e:#}"),
                    }
                    self.products = sort(&self.conn).unwrap_or_default();
                    self.code.clear();
                    self.date.clear();
                    self.qt.clear();
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
        let mut list = column![].spacing(5);
        for (code, date, qt) in &self.products {
            list = list.push(text(format!("{code} {date} x{qt}")))
        }
        column![input, list].spacing(20).padding(20).into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("PromoChecker")
        .run()
}
