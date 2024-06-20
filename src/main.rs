use eframe::egui::{self, Color32, RichText};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Heroes {
    pub tanks: Vec<Hero>,
    pub damages: Vec<Hero>,
    pub supports: Vec<Hero>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub favourite: bool,
}

impl Hero {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            level: 1,
            favourite: false,
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
    }

    pub fn toggle_favourite(&mut self) {
        self.favourite = !self.favourite;
    }
}

impl Display for Hero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.level, self.name)
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Pick Me",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(PickMeApp::new(cc))),
    )
}

struct PickMeApp {
    heroes: Heroes,
    picked: Option<String>,
    tank: bool,
    damage: bool,
    support: bool,
    favourite: bool,
    lowest: bool,
}

impl PickMeApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let heroes_file = File::open("heroes.yaml").expect("Unable to open 'heroes.yaml'");
        let heroes: Heroes =
            serde_yaml::from_reader(heroes_file).expect("Unable to parse heroes from yaml");
        Self {
            heroes,
            picked: None,
            tank: true,
            damage: true,
            support: true,
            favourite: false,
            lowest: false,
        }
    }

    fn lowest_level(&self) -> u32 {
        let mut min = u32::MAX;
        for hero in self.heroes.tanks.iter() {
            if hero.level < min {
                min = hero.level;
            }
        }
        for hero in self.heroes.damages.iter() {
            if hero.level < min {
                min = hero.level;
            }
        }
        for hero in self.heroes.supports.iter() {
            if hero.level < min {
                min = hero.level;
            }
        }
        min
    }
}

impl eframe::App for PickMeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let lowest_level = self.lowest_level();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Pick Me").clicked() {
                        let mut all_heroes: Vec<Hero> = Vec::new();
                        if self.tank {
                            all_heroes.append(&mut self.heroes.tanks.clone());
                        }
                        if self.damage {
                            all_heroes.append(&mut self.heroes.damages.clone());
                        }
                        if self.support {
                            all_heroes.append(&mut self.heroes.supports.clone());
                        }
                        all_heroes.retain(|hero| !self.favourite || hero.favourite);
                        all_heroes.retain(|hero| !self.lowest || hero.level == lowest_level);
                        self.picked = Some(
                            all_heroes
                                .choose(&mut rand::thread_rng())
                                .expect("No heroes to choose from :(")
                                .name
                                .clone(),
                        );
                    }
                    if let Some(hero) = &self.picked {
                        ui.heading(hero);
                    } else {
                        ui.heading("<- Click me");
                    }
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.tank, "Tank");
                    ui.checkbox(&mut self.damage, "Damage");
                    ui.checkbox(&mut self.support, "Support");
                    ui.checkbox(&mut self.favourite, "Favourite");
                    ui.checkbox(&mut self.lowest, "Lowest");
                });
            });
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Tank");
                    for tank in self.heroes.tanks.iter_mut() {
                        ui.horizontal(|ui| {
                            if ui.button("⬆").clicked() {
                                tank.level_up();
                            }
                            let star = if tank.favourite { "★" } else { "☆" };
                            if ui
                                .button(RichText::new(star).color(Color32::YELLOW))
                                .clicked()
                            {
                                tank.toggle_favourite();
                            }
                            if (tank.level == lowest_level || !self.lowest)
                                && (tank.favourite || !self.favourite)
                                && self.tank
                            {
                                ui.label(RichText::new(tank.to_string()).strong());
                            } else {
                                ui.label(RichText::new(tank.to_string()));
                            }
                        });
                    }
                });
                ui.vertical(|ui| {
                    ui.heading("Damage");
                    for damage in self.heroes.damages.iter_mut() {
                        ui.horizontal(|ui| {
                            if ui.button("⬆").clicked() {
                                damage.level_up();
                            }
                            let star = if damage.favourite { "★" } else { "☆" };
                            if ui
                                .button(RichText::new(star).color(Color32::YELLOW))
                                .clicked()
                            {
                                damage.toggle_favourite();
                            }
                            if (damage.level == lowest_level || !self.lowest)
                                && (damage.favourite || !self.favourite)
                                && self.damage
                            {
                                ui.label(RichText::new(damage.to_string()).strong());
                            } else {
                                ui.label(RichText::new(damage.to_string()));
                            }
                        });
                    }
                });
                ui.vertical(|ui| {
                    ui.heading("Support");
                    for support in self.heroes.supports.iter_mut() {
                        ui.horizontal(|ui| {
                            if ui.button("⬆").clicked() {
                                support.level_up();
                            }
                            let star = if support.favourite { "★" } else { "☆" };
                            if ui
                                .button(RichText::new(star).color(Color32::YELLOW))
                                .clicked()
                            {
                                support.toggle_favourite();
                            }
                            if (support.level == lowest_level || !self.lowest)
                                && (support.favourite || !self.favourite)
                                && self.support
                            {
                                ui.label(RichText::new(support.to_string()).strong());
                            } else {
                                ui.label(RichText::new(support.to_string()));
                            }
                        });
                    }
                });
            })
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let heroes_file = File::create("heroes.yaml").expect("Unable to open 'heroes.yaml'");
        serde_yaml::to_writer(heroes_file, &self.heroes).expect("Unable to save heroes to file");
    }
}
