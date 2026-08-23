// Promochecker aims to be a graphical application to manage promotions as well as expiration of products
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

use ProductManager::{opendb, writedb, sort, remove, removepromo, sortpromo, writepromo};
use iced::widget::{button, column, row, text, text_input, container};
use iced::Length;
use iced::{Element, Task};
use rusqlite::Connection;
use chrono::{Local, NaiveDate};


struct App {
    conn: Connection,
    code: String,
    date: String, 
    qt: String,
    products: Vec<(String, String, u32, i64)>, 
    status: Option<String>,
    tabs: Tabs,
    promoproducts: Vec<(String, String, Option<u32>, i64)>

}

#[derive(Debug, Clone)]
pub enum Message {
    Add,
    Remove(i64),
    //Remove(String),
    CodeChanged(String), 
    DateChanged(String),
    QtChanged(String),
    DisplayPromo,
    SwitchTab(Tabs),
    AddPromo,
    RemovePromo(i64)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tabs {
    Peremptions, 
    Promotions,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let conn = opendb().expect("Impossible to open database");
        let products = sort(&conn).unwrap_or_default();
        let promoproducts = sortpromo(&conn).unwrap_or_default(); 
        let app = Self {
            conn,
            code: String::new(),
            date: String::new(),
            qt: String::new(),
            products,
            status: None,
            tabs: Tabs::Peremptions,
            promoproducts, 

        };
        (app, Task::none())
    
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        
        match message {
            Message::CodeChanged(v) => self.code = v, 
            Message::DateChanged(v) => self.date = v,
            Message::QtChanged(v) => self.qt = v,
            Message::SwitchTab(o) => self.tabs = o,
            Message::DisplayPromo => {}
            Message::Add => {
                self.status = None;
                if let Ok(qt) = self.qt.parse::<u32>() {
                    match writedb(&self.conn, &self.code, &self.date, qt) {
                        Ok(_) => {}
                        Err(e) => {
                            self.status = Some(format!("Ajout impossible: {e:#}"));
                            // in order to not leave the insertion fields empty
                            return Task::none();
                        }
                    }
                    match sort(&self.conn) {
                        Ok(list) => self.products = list,
                        Err(e) => self.status = Some(format!("Impossible de lire dans la base de donnée {e:#}")),

                    }
                    self.code.clear();
                    self.date.clear();
                    self.qt.clear();
                } else {
                    self.status = Some("Impossible de parser la quantité indiquée".to_string());
                }
            }
            Message::AddPromo => {
                self.status = None;
                if let Ok(qt) = self.qt.parse::<u32>() {
                    match writepromo(&self.conn, &self.code, &self.date, Some(qt)) {
                        Ok(_) => {}
                        Err(e) => {
                            self.status = Some(format!("Ajout impossible: {e:#}"));
                            // in order to not leave the insertion fields empty
                            return Task::none();
                        }
                    }
                    match sortpromo(&self.conn) {
                        Ok(list) => self.promoproducts = list,
                        Err(e) => self.status = Some(format!("Impossible de lire dans la base de donnée {e:#}")),

                    }
                    self.code.clear();
                    self.date.clear();
                    self.qt.clear();
                } else {
                    self.status = Some("Impossible de parser la quantité indiquée".to_string());
                }
            }
            Message::Remove(id) => {
                self.status = None;
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
            Message::RemovePromo(id) => {
                self.status = None;
                match removepromo(&self.conn, id) {
                    Ok(_) => {}
                    Err(e) => {
                        self.status = Some(format!("Une erreur s'est produite: {e:#}"));
                    }                    
                } 
                match sortpromo(&self.conn) {
                    Ok(list) => self.promoproducts = list,
                    Err(e) => self.status = Some(format!("Impossible de lire la base de donnée: {e}")),
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let tab = row![
            button("Promotions").on_press(Message::SwitchTab(Tabs::Promotions)).style(if self.tabs == Tabs::Promotions {
                button::primary } else {
                    button::secondary
                }),
            button("Peremptions").on_press(Message::SwitchTab(Tabs::Peremptions)).style(if self.tabs == Tabs::Peremptions {
                button::primary } else {
                    button::secondary
                }),
        ].spacing(10);
        match self.tabs {
            Tabs::Peremptions => {
                let input = row![
                    text_input("Code", &self.code).on_input(Message::CodeChanged).width(Length::FillPortion(1)),
                    text_input("Date", &self.date).on_input(Message::DateChanged).width(Length::FillPortion(1)),
                    text_input("Quantité", &self.qt).on_input(Message::QtChanged).width(Length::FillPortion(1)),
                    button("Ajouter").on_press(Message::Add),
                ].spacing(10);
                let mut list = column![].spacing(20);
                for (code, date, qt, id) in &self.products {
                    let today = Local::now().date_naive();
                    let d = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
                    let mut days = ( d - today).num_days();
                    let mut title = if days < 0 {
                        format!("Produit prérimé depuis {} jours", -days)
                    } else if days == 0 {
                        format!("Produit périmé aujourd'hui")
                    } else {
                        format!("Périme sous {days} jours")
                    };
                    let mut color = if days < 7 {
                        iced::Color::from_rgb(0.9, 0.3, 0.3)
                    } else if days < 30 {
                        iced::Color::from_rgb(0.9, 0.6, 0.2)
                    } else {
                        iced::Color::WHITE
                    };
                    let line = row![
                        text(title).color(color).size(25),
                        text(format!("{code}")).size(15).width(Length::FillPortion(1)),
                        text(format!("{date}")).size(15).width(Length::FillPortion(1)),
                        text(format!("x{qt}")).size(15).width(Length::FillPortion(1)),
                        button("Supprimer").on_press(Message::Remove(*id)),
                    ].spacing(10);
                    let card = container(line)
                        .padding(12)
                        .width(Length::Fill)
                        .style(container::rounded_box);
                    list = list.push(card);
                }
                let mut content = column![tab, input, list].spacing(20).padding(20);
                if let Some(msg) = &self.status {
                    content = content.push(
                        text(msg).color(iced::Color::from_rgb(0.9, 0.2, 0.2))
                    )
                }
                return content.into()
            }
            Tabs::Promotions => {
                let input = row![
                    text_input("Code", &self.code).on_input(Message::CodeChanged).width(Length::FillPortion(1)),
                    text_input("Date", &self.date).on_input(Message::DateChanged).width(Length::FillPortion(1)),
                    text_input("Quantité", &self.qt).on_input(Message::QtChanged).width(Length::FillPortion(1)),
                    button("Ajouter").on_press(Message::AddPromo),
                ]
                .spacing(10);
                let mut list = column![].spacing(20);
                for (code, date, qt, id) in &self.promoproducts {
                    let qt = qt.unwrap_or_else(|| 0);
                    let line = row![
                        text(format!("{code}")).size(18).width(Length::FillPortion(1)),
                        text(format!("{date}")).size(15).width(Length::FillPortion(1)),
                        text(format!("x{qt}")).size(15).width(Length::FillPortion(1)),
                        button("Supprimer").on_press(Message::RemovePromo(*id)),
                    ].spacing(10);
                    let card = container(line)
                        .padding(12)
                        .width(Length::Fill)
                        .style(container::rounded_box);
                    list = list.push(card);
                }
                let mut content = column![tab, input, list].spacing(20).padding(20);
                if let Some(msg) = &self.status {
                    content = content.push(
                        text(msg).color(iced::Color::from_rgb(0.9, 0.2, 0.2))
                    )
                }
                return content.into()
            }
        };
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("ProductManager")
        .run()
}
