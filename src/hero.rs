use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
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

impl Heroes {
    pub fn migrate(&mut self) {
        let default = Self::default();
        Self::add_missing(&mut self.tanks, &default.tanks);
        Self::add_missing(&mut self.damages, &default.damages);
        Self::add_missing(&mut self.supports, &default.supports);
        Self::set_stadium(&mut self.tanks, &default.tanks);
        Self::set_stadium(&mut self.damages, &default.damages);
        Self::set_stadium(&mut self.supports, &default.supports);
    }

    /// Add any additional heroes in `new` to `old`
    fn add_missing(old: &mut Vec<Hero>, new: &[Hero]) {
        for hero in new {
            if !old.contains(hero) {
                println!("Adding new hero '{}'", hero.name);
                old.push(hero.clone());
            }
        }
        old.sort_unstable_by_key(|h| h.name.clone());
    }

    /// Set stadium enabled for any hero in `old` where it is set in `new`
    fn set_stadium(old: &mut Vec<Hero>, new: &[Hero]) {
        for hero in old {
            if let Some(new_hero) = new.iter().find(|h| hero.name == h.name) {
                if new_hero.stadium && !hero.stadium {
                    println!("Setting stadium enable for '{}'", new_hero.name);
                    hero.stadium |= new_hero.stadium;
                }
            }
        }
    }
}

impl Default for Heroes {
    fn default() -> Self {
        Self {
            tanks: vec![
                Hero::new("D.va", Role::Tank, true),
                Hero::new("Doomfist", Role::Tank, false),
                Hero::new("Hazard", Role::Tank, true),
                Hero::new("Junker Queen", Role::Tank, true),
                Hero::new("Mauga", Role::Tank, false),
                Hero::new("Orisa", Role::Tank, true),
                Hero::new("Ramatra", Role::Tank, false),
                Hero::new("Reinhardt", Role::Tank, true),
                Hero::new("Roadhog", Role::Tank, false),
                Hero::new("Sigma", Role::Tank, true),
                Hero::new("Winston", Role::Tank, true),
                Hero::new("Wrecking Ball", Role::Tank, false),
                Hero::new("Zarya", Role::Tank, true),
            ],
            damages: vec![
                Hero::new("Ashe", Role::Damage, true),
                Hero::new("Bastion", Role::Damage, false),
                Hero::new("Cassidy", Role::Damage, true),
                Hero::new("Echo", Role::Damage, false),
                Hero::new("Freya", Role::Damage, true),
                Hero::new("Genji", Role::Damage, true),
                Hero::new("Hanzo", Role::Damage, false),
                Hero::new("Junkrat", Role::Damage, true),
                Hero::new("Mei", Role::Damage, true),
                Hero::new("Pharah", Role::Damage, true),
                Hero::new("Reaper", Role::Damage, true),
                Hero::new("Sojourn", Role::Damage, true),
                Hero::new("Soldier: 76", Role::Damage, true),
                Hero::new("Sombra", Role::Damage, false),
                Hero::new("Symmetra", Role::Damage, false),
                Hero::new("Törbjorn", Role::Damage, true),
                Hero::new("Tracer", Role::Damage, true),
                Hero::new("Venture", Role::Damage, false),
                Hero::new("Widowmaker", Role::Damage, false),
            ],
            supports: vec![
                Hero::new("Ana", Role::Support, true),
                Hero::new("Baptiste", Role::Support, false),
                Hero::new("Brigitte", Role::Support, true),
                Hero::new("Illari", Role::Support, false),
                Hero::new("Juno", Role::Support, true),
                Hero::new("Kiriko", Role::Support, true),
                Hero::new("Lifeweaver", Role::Support, false),
                Hero::new("Lúcio", Role::Support, true),
                Hero::new("Mercy", Role::Support, true),
                Hero::new("Moira", Role::Support, true),
                Hero::new("Wuyang", Role::Support, false),
                Hero::new("Zenyatta", Role::Support, true),
            ],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hero {
    pub name: String,
    pub stadium: bool,
    pub role: Role,
    pub level: u32,
    pub favourite: bool,
}

impl Hero {
    pub fn new<S: Into<String>>(name: S, role: Role, stadium: bool) -> Self {
        Self {
            name: name.into(),
            level: 1,
            favourite: false,
            stadium,
            role,
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
