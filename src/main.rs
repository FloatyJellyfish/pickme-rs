use eframe::egui::{self, Color32, RichText, Ui};
use hero::*;
use rand::prelude::SliceRandom;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

mod hero;

const KEY_FILTERS: &str = "filters";
const KEY_FILE_PATH: &str = "file_path";

const DEFAULT_FILE_PATH: &str = "heroes.yaml";

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
    file_path: PathBuf,
}

impl PickMeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let file_path = Self::load_file_path(cc);
        let heroes = Self::load_heroes(&file_path);

        Self {
            heroes,
            picked: None,
            filters: Filters::load(cc),
            file_path,
        }
    }

    fn load_file_path(cc: &eframe::CreationContext<'_>) -> PathBuf {
        let storage = cc.storage.expect("Persistence feature is not enabled");
        if let Some(file_path) = storage.get_string(KEY_FILE_PATH) {
            println!("Retrieved file path from storage: {file_path}");
            PathBuf::from(file_path)
        } else {
            println!("Loading from default file path");
            PathBuf::from_str(DEFAULT_FILE_PATH).unwrap()
        }
    }

    fn load_heroes(path: &Path) -> Heroes {
        if path.exists() {
            if let Ok(file) = File::open(path) {
                if let Ok(heroes) = serde_yaml::from_reader(file) {
                    heroes
                } else {
                    println!("Could not parse heroes file, loading defaults");
                    Heroes::default()
                }
            } else {
                println!("Could not open heroes file, loading defaults");
                Heroes::default()
            }
        } else {
            println!("Heroes file does not exist, loading defaults");
            Heroes::default()
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

    fn draw_role_column(
        ui: &mut Ui,
        heroes: &mut Vec<Hero>,
        role: Role,
        filters: &Filters,
        lowest_level: u32,
    ) {
        ui.vertical(|ui| {
            ui.heading(role.to_string());
            for hero in heroes {
                Self::draw_hero_row(ui, hero, filters.is_selected(hero, lowest_level, role));
            }
        });
    }
}

impl eframe::App for PickMeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let lowest_level = self.lowest_level();
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        let file_path = FileDialog::new()
                            .set_directory("./")
                            .set_file_name("heroes.yaml")
                            .add_filter("YAML", &["yaml"])
                            .save_file();
                        if let Some(file_path) = file_path {
                            println!("Creating new heroes file: {}", file_path.to_str().unwrap());
                            self.file_path = file_path;
                            self.heroes = Heroes::default();
                        }
                    }
                    if ui.button("Open..").clicked() {
                        let file_path = FileDialog::new()
                            .add_filter("YAML", &["yaml"])
                            .set_directory("./")
                            .pick_file();
                        if let Some(file_path) = file_path {
                            println!("Setting new file path: {}", file_path.to_str().unwrap());
                            self.heroes = Self::load_heroes(&file_path);
                            self.file_path = file_path;
                        } else {
                            println!("No file selected");
                        }
                    }
                    if ui.button("Save As..").clicked() {
                        let file_path = FileDialog::new()
                            .set_directory("./")
                            .set_file_name("heroes.yaml")
                            .add_filter("YAML", &["yaml"])
                            .save_file();
                        if let Some(file_path) = file_path {
                            println!(
                                "Saving heroes into new file: {}",
                                file_path.to_str().unwrap()
                            );
                            self.file_path = file_path;
                        }
                    }
                })
            })
        });
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
                Self::draw_role_column(
                    ui,
                    self.heroes.tanks.as_mut(),
                    Role::Tank,
                    &self.filters,
                    lowest_level,
                );
                Self::draw_role_column(
                    ui,
                    self.heroes.damages.as_mut(),
                    Role::Damage,
                    &self.filters,
                    lowest_level,
                );
                Self::draw_role_column(
                    ui,
                    self.heroes.supports.as_mut(),
                    Role::Support,
                    &self.filters,
                    lowest_level,
                );
            })
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let heroes_file = File::create(&self.file_path).expect("Unable to open 'heroes.yaml'");
        serde_yaml::to_writer(heroes_file, &self.heroes).expect("Unable to save heroes to file");
        storage.set_string(
            KEY_FILTERS,
            serde_json::to_string(&self.filters).expect("Unable to serialize filters"),
        );
        storage.set_string(KEY_FILE_PATH, self.file_path.to_str().unwrap().to_string());
    }
}
