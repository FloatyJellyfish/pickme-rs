use eframe::egui::{self, Color32, RichText, Ui};
use hero::*;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

mod hero;

const KEY_FILTERS: &str = "filters";

const HEROES_FILE_PATH: &str = "heroes.yaml";

#[derive(Serialize, Deserialize)]
struct Filters {
    tank: bool,
    damage: bool,
    support: bool,
    favourite: bool,
    lowest: bool,
}

impl Filters {
    /// # Panics
    /// Panics if `persistence` feature of eframe isn't enabled, or there was an error deserializing filters
    fn load(cc: &eframe::CreationContext<'_>) -> Self {
        let storage = cc.storage.expect("Persistence feature is not enabled");
        match storage.get_string(KEY_FILTERS) {
            None => Self::default(),
            Some(string) => serde_json::from_str(&string).expect("Unable to deserialize filters"),
        }
    }

    fn is_selected(&self, hero: &Hero, lowest_level: u32, role: Role) -> bool {
        (hero.level == lowest_level || !self.lowest)
            && (hero.favourite || !self.favourite)
            && ((self.tank && role == Role::Tank)
                || (self.support && role == Role::Support)
                || (self.damage && role == Role::Damage))
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            tank: true,
            damage: true,
            support: true,
            favourite: false,
            lowest: false,
        }
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
    filters: Filters,
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
            filters: Filters::load(cc),
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

    fn draw_hero_row(ui: &mut Ui, hero: &mut Hero, selected: bool) {
        ui.horizontal(|ui| {
            if ui.button("⬆").clicked() {
                hero.level_up();
            }
            let star = if hero.favourite { "★" } else { "☆" };
            if ui
                .button(RichText::new(star).color(Color32::YELLOW))
                .clicked()
            {
                hero.toggle_favourite();
            }
            if selected {
                ui.label(RichText::new(hero.to_string()).strong());
            } else {
                ui.label(RichText::new(hero.to_string()));
            }
        });
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
                        if self.filters.tank {
                            all_heroes.append(&mut self.heroes.tanks.clone());
                        }
                        if self.filters.damage {
                            all_heroes.append(&mut self.heroes.damages.clone());
                        }
                        if self.filters.support {
                            all_heroes.append(&mut self.heroes.supports.clone());
                        }
                        all_heroes.retain(|hero| !self.filters.favourite || hero.favourite);
                        all_heroes
                            .retain(|hero| !self.filters.lowest || hero.level == lowest_level);
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
                    ui.checkbox(&mut self.filters.tank, "Tank");
                    ui.checkbox(&mut self.filters.damage, "Damage");
                    ui.checkbox(&mut self.filters.support, "Support");
                    ui.checkbox(&mut self.filters.favourite, "Favourite");
                    ui.checkbox(&mut self.filters.lowest, "Lowest");
                });
            });
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Tank");
                    for tank in self.heroes.tanks.iter_mut() {
                        Self::draw_hero_row(
                            ui,
                            tank,
                            self.filters.is_selected(tank, lowest_level, Role::Tank),
                        );
                    }
                });
                ui.vertical(|ui| {
                    ui.heading("Damage");
                    for damage in self.heroes.damages.iter_mut() {
                        Self::draw_hero_row(
                            ui,
                            damage,
                            self.filters.is_selected(damage, lowest_level, Role::Damage),
                        );
                    }
                });
                ui.vertical(|ui| {
                    ui.heading("Support");
                    for support in self.heroes.supports.iter_mut() {
                        Self::draw_hero_row(
                            ui,
                            support,
                            self.filters
                                .is_selected(support, lowest_level, Role::Support),
                        );
                    }
                });
            })
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let heroes_file = File::create(HEROES_FILE_PATH).expect("Unable to open 'heroes.yaml'");
        serde_yaml::to_writer(heroes_file, &self.heroes).expect("Unable to save heroes to file");
        storage.set_string(
            KEY_FILTERS,
            serde_json::to_string(&self.filters).expect("Unable to serialize filters"),
        );
    }
}
