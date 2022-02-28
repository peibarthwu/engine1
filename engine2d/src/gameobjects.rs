use super::types::*;
use super::image::Image;

// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct Item {
    pub name: String, // E.g. "Antechamber"
    pub desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    pub sheetpos: Rect,
    pub roomloca: Vec2i,
    pub img: Image,
}

// #[derive(PartialEq, Eq, Clone)]
pub struct Tile {
    pub sheetpos: Rect,
    pub img: Image,
}

pub struct Sprite {
    pub sheetpos: Rect,
    pub img: Image,
    pub cur_pos: Vec2i,
}

// #[derive(PartialEq, Eq, Clone)]
pub struct Room {
    pub name: String, // E.g. "Antechamber"
    pub desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    //pub doors: Vec<Door>,
    //pub floor: Vec<Tile>,
    pub items: Vec<Item>,
    pub img: Image,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Door {
    target: String, // More about this in a minute
    triggers:Vec<String>, // e.g. "go north", "north"
    message: Option<String> // What message, if any, to print when the doorway is traversed
    // Any other info about the door would go here
}