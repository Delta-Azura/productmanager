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

use ProductManager::{opendb, writedb, sort, remove, removepromo, sortpromo, writepromo, load, compare, html};
use iced::widget::{button, column, row, text, text_input, container};
use iced::Length;
use iced::{Element, Task};
use rusqlite::Connection;
use chrono::{Local, NaiveDate};
use iced::widget::pick_list;
use std::fmt;
use std::collections::HashMap;
use ProductManager::expiration::encoding::Catalogue;


struct App {
    conn: Connection,
    code: String,
    date: String, 
    qt: String,
    products: Vec<(String, String, u32, i64)>, 
    status: Option<String>,
    tabs: Tabs,
    search: String,
    promoproducts: Vec<(String, String, Option<u32>, i64)>,
    filter: Filter,
    datesearch: bool,
    datestart: String,
    dateend: String,
    catalogue: Catalogue,
    name: String,

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
    RemovePromo(i64), 
    ChoseFilter(Filter),
    Search(String),
    DateSearch(bool),
    QueryDateChangedstart(String),
    QueryDateChangedend(String),
    CodeLoaded(String),
    Print,
    

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tabs {
    Peremptions, 
    Promotions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    Month,
    ThreeMonth,
    All,
}



impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Filter::Month => "Périme dans 1 mois",
            Filter::ThreeMonth => "Périme dans 3 mois",
            Filter::All => "Tout afficher"
        };
        write!(f, "{text}")
    }
}


impl App {
    pub fn new() -> (Self, Task<Message>) {
        let conn = opendb().expect("Impossible to open database");
        let products = sort(&conn).unwrap_or_default();
        let promoproducts = sortpromo(&conn).unwrap_or_default(); 
        let datesearch = false;
        let (catalogue, status) = match load("/var/cache/export_codification.csv") {
            Ok(catalogue) => { (catalogue, None) }
            Err(e) => { (Catalogue::new(), Some(format!("Catalogue non chargé: {e:#}"))) }
        };
        let app = Self {
            name: String::new(),
            conn,
            catalogue, 
            status,
            datesearch,
            code: String::new(),
            date: String::new(),
            qt: String::new(),
            search: String::new(),
            datestart: String::new(),
            dateend: String::new(),
            products,
            tabs: Tabs::Peremptions,
            promoproducts,
            filter: Filter::All,
        };
        (app, Task::none())
    
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CodeChanged(v) => self.code = v,
                
                
            Message::CodeLoaded(v) => {
                match compare(&self.code, &self.catalogue) {
                    Ok(name) => {
                        self.name = name.clone();
                        println!("{}", name)
                    }
                    Err(e) => { 
                        self.status = Some(format!("Une erreur s'est produite: {e:#}"));
                    }
                }
            }
            Message::Print => {
                if let Err(e) = html(&self.products, &self.catalogue) {
                    self.status = Some(format!("Impression impossible: {e:#}"));
                }
            }
            Message::DateChanged(v) => self.date = v,
            Message::QtChanged(v) => self.qt = v,
            Message::SwitchTab(o) => self.tabs = o,
            Message::ChoseFilter(o) => self.filter = o, 
            Message::DisplayPromo => {}
            Message::Search(v) => self.search = v,
            Message::DateSearch(v) => self.datesearch = v,
            Message::QueryDateChangedstart(v) => self.datestart = v,
            Message::QueryDateChangedend(v) => self.dateend = v,
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
            button("Imprimer le listing").on_press(Message::Print),
        ].spacing(10);
        match self.tabs {
            Tabs::Peremptions => {
                let querydate = if self.datesearch == true {
                    row![
                        text_input("Date de début : JJ/MM/AAAA", &self.datestart).on_input(Message::QueryDateChangedstart),
                        text_input("Date de fin : JJ/MM/AAAA", &self.dateend).on_input(Message::QueryDateChangedend),
                    ].spacing(10)
                } else {
                    row![]
                };

                let search_bar = text_input("Rechercher un produit...", &self.search).on_input(Message::Search);
                let input = row![
                    text_input("Code", &self.code).on_input(Message::CodeChanged).width(Length::FillPortion(1)),
                    text_input("Date", &self.date).on_input(Message::DateChanged).width(Length::FillPortion(1)),
                    text_input("Quantité", &self.qt).on_input(Message::QtChanged).width(Length::FillPortion(1)),
                    button("Ajouter").on_press(Message::Add),
                    if self.datesearch == true {
                        button("Rechercher par date").on_press(Message::DateSearch(false))
                    } else {
                        button("Rechercher par date").on_press(Message::DateSearch(true))
                    },
                ].spacing(10);
                let start = NaiveDate::parse_from_str(&self.datestart, "%d/%m/%Y").ok();
                let end = NaiveDate::parse_from_str(&self.dateend, "%d/%m/%Y").ok();
                let options = [Filter::Month, Filter::ThreeMonth, Filter::All];
                let menu = pick_list(
                    options,
                    Some(self.filter),
                    Message::ChoseFilter,
                );
                let limit = match self.filter {
                    Filter::Month => 30, 
                    Filter::ThreeMonth => 90,
                    Filter::All => i64::MAX,
                };
                let mut list = column![].spacing(20);
                for (code, date, qt, id) in &self.products {
                    let name = compare(code, &self.catalogue).unwrap_or_else(|_| code.to_string());
                    let q = self.search.to_lowercase();
                    if !self.search.is_empty() && !code.contains(&self.search) && !name.to_lowercase().contains(&self.search) {
                        continue;
                    }
                    let today = Local::now().date_naive();
                    let d = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
                    let mut days = ( d - today).num_days();
                    if self.datesearch == true {
                        if let Some(start) = start {
                            if d < start { continue; }
                        }

                        if let Some(end) = end {
                            if d > end { continue; }
                        }
                    }
                    let mut color = if days < 7 {
                        iced::Color::from_rgb(0.9, 0.3, 0.3)
                    } else if days < 30 {
                        iced::Color::from_rgb(0.9, 0.6, 0.2)
                    } else {
                        iced::Color::WHITE
                    };
                    if days > limit {
                        continue;
                    }
                    let line = row![
                        text(format!("{name}")).size(15).width(Length::FillPortion(1)),
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
                let mut content = column![tab, input, querydate, search_bar, menu, list].spacing(20).padding(20);
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
