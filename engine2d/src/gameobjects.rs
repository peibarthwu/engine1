use super::types::*;
use super::image::Image;

// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct Item {
    pub name: String, // E.g. "Antechamber"
    pub desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    pub sheetpos: Rect, //make this a usize
    pub roomloca: Vec2i,
    pub img: Image,
    pub collider: Rect,
    pub frames: Vec<Rect>,
    pub cur_frame: usize,
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
    pub collider: Rect,
}

impl Sprite {
    pub fn moveself(&mut self, mut dx: i32, mut dy: i32, room: &Room) -> () {
        for item in room.items.iter() {
            println!("{:?}", self.collider.touches(item.collider));
                // match self.collider.rect_displacement(item.collider) {
                //     None => {},
                //     Some(overlap) => {
                //         println!("overlap");
                //         if(overlap.0 > 0){
                //             dx = 0;
                //         }
                //         if(overlap.1 > 0){
                //             dy = 0;
                //         }
                //     },
                // }
            
            
            
            if (item.collider.touches(self.collider)){
                dx = 0;
                dy = 0;
            }
        }
        self.collider.pos.x += dx;
        self.collider.pos.y += dy;

        self.cur_pos.x += dx;
        self.cur_pos.y += dy;
    }
    //collisions look at greatest x point of one and lest of the other for x overlap
    // do the same with left overlap
    //if both overlap them collision
}

pub trait Animation {
    fn anim(&mut self){}
}

impl Animation for Item {
    fn anim(&mut self){
        if(self.frames.len() == 0){
            return;
        }
        if( self.cur_frame < self.frames.len()){
            self.cur_frame += 1;
        }
        else{
            self.cur_frame = 0;
        }
        
    }
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

pub struct Assets {
    pub img: Image,
    pub colors: [Color;6]
}

pub struct State {
    pub w: usize,
    pub h: usize,
    pub color: usize,
    pub room: Room,
    pub sprite: Sprite,
}