use eframe::egui::{self, Color32, RichText};
use hero::*;
use rand::prelude::SliceRandom;
use std::{fs::File, path::Path};

mod hero;

const KEY_TANK: &str = "tank";
const KEY_DAMAGE: &str = "damage";
const KEY_SUPPORT: &str = "support";
const KEY_FAVOURITE: &str = "favourite";
const KEY_LOWEST: &str = "lowest";

const HEROES_FILE_PATH: &str = "heroes.yaml";

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
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let heroes = if Path::new(HEROES_FILE_PATH).exists() {
            let heroes_file = File::open("heroes.yaml").expect("Unable to open 'heroes.yaml'");
            serde_yaml::from_reader(heroes_file).expect("Unable to parse heroes from yaml")
        } else {
            Heroes::default()
        };
        Self {
            heroes,
            picked: None,
            tank: get_bool_or_default(cc, KEY_TANK, true),
            damage: get_bool_or_default(cc, KEY_DAMAGE, true),
            support: get_bool_or_default(cc, KEY_SUPPORT, true),
            favourite: get_bool_or_default(cc, KEY_FAVOURITE, false),
            lowest: get_bool_or_default(cc, KEY_LOWEST, false),
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

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let heroes_file = File::create(HEROES_FILE_PATH).expect("Unable to open 'heroes.yaml'");
        serde_yaml::to_writer(heroes_file, &self.heroes).expect("Unable to save heroes to file");
        storage.set_string(KEY_TANK, self.tank.to_string());
        storage.set_string(KEY_DAMAGE, self.damage.to_string());
        storage.set_string(KEY_SUPPORT, self.support.to_string());
        storage.set_string(KEY_FAVOURITE, self.favourite.to_string());
        storage.set_string(KEY_LOWEST, self.lowest.to_string());
    }
}

fn get_bool_or_default(cc: &eframe::CreationContext<'_>, key: &str, default: bool) -> bool {
    cc.storage
        .expect("Persistence feature not enabled")
        .get_string(key)
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(default)
}
