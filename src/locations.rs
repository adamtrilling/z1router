use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Quest {
    First,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Board {
    Overworld,
    // Level1,
    // Level2,
    // Level3,
    // Level4,
    // Level5,
    // Level6,
    // Level7,
    // Level8,
    // Level9
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    quest: Quest,
    board: Board,
    x: u8,
    y: u8,
}

impl Location {
    pub fn new(quest: Quest, board: Board, x: u8, y: u8) -> Self {
        Self {
            quest: quest,
            board: board,
            x: x,
            y: y,
        }
    }

    pub fn start(quest: Quest) -> Self {
        Self {
            quest: quest,
            board: Board::Overworld,
            x: 7,
            y: 0,
        }
    }

    pub fn ow1q(x: u8, y: u8) -> Self {
        Self {
            quest: Quest::First,
            board: Board::Overworld,
            x: x,
            y: y,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ({}, {})", self.board, self.y, self.x)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Path {
    cost: u32,
    pub from: Location,
    pub to: Location,
    provides: Option<Item>,
    requires: Vec<Constraint>,
}

impl Path {
    pub fn walk(from: Location, to: Location) -> Path {
        Self {
            from: from.clone(),
            to: to.clone(),
            provides: None,
            requires: vec![],
            cost: 1,
        }
    }

    pub fn scroll(from: Location, to: Location) -> Path {
        Self {
            from: from.clone(),
            to: to.clone(),
            provides: None,
            requires: vec![Constraint::ScreenScroll],
            cost: 1,
        }
    }

    pub fn get_item(loc: Location, item: Item, needs: Vec<Constraint>) -> Path {
        Self {
            from: loc.clone(),
            to: loc.clone(),
            provides: Some(item.clone()),
            requires: needs,
            cost: 1,
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.provides.clone() {
            Some(item) => write!(f, "get {:?} at {}", item, self.from),
            None => write!(f, "from {} to {}", self.from, self.to),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Rupees(u8),
    WoodSword,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Constraint {
    ScreenScroll,
    Bomb(u8),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Route {
    pub paths: Vec<Path>,
    pub location: Location,
    inventory: Vec<Item>,
}

impl Route {
    pub fn new() -> Self {
        Self {
            location: Location::start(Quest::First),
            paths: vec![],
            inventory: vec![],
        }
    }

    pub fn next_paths(&self) -> Vec<Path> {
        first_quest_paths()
            .iter()
            .filter(|p| p.from == self.location)
            .map(|p| p.clone())
            .collect()
    }
}

pub fn first_quest_paths() -> Vec<Path> {
    let paths = vec![
        Path::get_item(Location::ow1q(7, 0), Item::WoodSword, vec![]),
        Path::walk(Location::ow1q(7, 0), Location::ow1q(7, 1)),
        Path::walk(Location::ow1q(7, 0), Location::ow1q(6, 0)),
        Path::walk(Location::ow1q(7, 0), Location::ow1q(8, 0)),
        Path::get_item(
            Location::ow1q(6, 0),
            Item::Rupees(50),
            vec![Constraint::Bomb(1)],
        ),
        Path::walk(Location::ow1q(6, 0), Location::ow1q(5, 0)),
    ];

    paths
}
