use std::fmt::Display;

use serde::{Deserialize, Serialize};

const TANKS: [&str; 12] = [
    "D.va",
    "Doomfist",
    "Junker Queen",
    "Mauga",
    "Orisa",
    "Ramatra",
    "Reinhardt",
    "Roadhog",
    "Sigma",
    "Winston",
    "Wrecking Ball",
    "Zarya",
];

const DAMAGES: [&str; 17] = [
    "Ashe",
    "Bastion",
    "Cassidy",
    "Echo",
    "Genji",
    "Hanzo",
    "Junkrat",
    "Mei",
    "Pharah",
    "Reaper",
    "Sojourn",
    "Soldier: 76",
    "Sombra",
    "Symmetra",
    "Törbjorn",
    "Venture",
    "Widowmaker",
];

const SUPPORTS: [&str; 10] = [
    "Ana",
    "Baptiste",
    "Brigitte",
    "Illari",
    "Kiriko",
    "Lifeweaver",
    "Lúcio",
    "Mercy",
    "Moira",
    "Zenyatta",
];

#[derive(PartialEq, Clone, Copy)]
pub enum Role {
    Tank,
    Damage,
    Support,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Tank => write!(f, "Tank"),
            Role::Damage => write!(f, "Damage"),
            Role::Support => write!(f, "Support"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Heroes {
    pub tanks: Vec<Hero>,
    pub damages: Vec<Hero>,
    pub supports: Vec<Hero>,
}

impl Default for Heroes {
    fn default() -> Self {
        let mut tanks = Vec::new();
        for tank in TANKS {
            tanks.push(Hero::new(tank))
        }
        let mut damages = Vec::new();
        for damage in DAMAGES {
            damages.push(Hero::new(damage))
        }
        let mut supports = Vec::new();
        for support in SUPPORTS {
            supports.push(Hero::new(support))
        }
        Self {
            tanks,
            damages,
            supports,
        }
    }
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

impl PartialEq for Hero {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
