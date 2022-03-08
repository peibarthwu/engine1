use super::types::*;
use super::image::Image;
use super::text::*;

pub struct Item {
    pub name: String, // E.g. "Antechamber"
    //pub desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    pub desc: Vec<Textbox>, 
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


pub trait Animation {
    fn anim(&mut self){}
}

impl Animation for Item {
    fn anim(&mut self){
        if(self.frames.len() == 0){
            return;
        }
        if( self.cur_frame < self.frames.len()-1){
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
    pub desc: Vec<Textbox>, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    pub doors: Vec<Door>,
    pub items: Vec<Item>,
    pub img: Image,
    pub floor: Rect,
}

// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct Door {
    pub collider: Rect,
    pub target: usize, //where it goes
}

pub struct Assets {
    pub img: Image,
    pub colors: [Color;6]
}

pub struct State {
    pub w: usize,
    pub h: usize,
    pub color: usize,
    pub room: usize,
    pub rooms: Vec<Room>,
    //pub textbox: usize,
    //pub textboxes: Vec<Textbox>,
    pub sprite: Sprite,
    pub inventory: Vec<Item>,
}

impl State {
    pub fn update(&mut self, mut dx: i32, mut dy: i32) -> () {
        let new_collider = Rect {
            pos: Vec2i { x: self.sprite.collider.pos.x as i32 + dx, y: self.sprite.collider.pos.y as i32 + dy},
            sz: self.sprite.collider.sz,
        };
       
        if self.rooms[self.room].floor.contains(new_collider) == false {
            dx = 0;
            dy = 0;
        }

        for item in self.rooms[self.room].items.iter() {
            if (new_collider.touches(item.collider)){
                dx = 0;
                dy = 0;
                println!("{:?}", item.name);
                //println!("{:?}", item.desc);
                

                if (item.name == "Key"){
                    println!("You got the key");
                }
            }
        }

        for door in self.rooms[self.room].doors.iter() {
            if (self.sprite.collider.touches(door.collider)){
                self.room = door.target;
            }
        }

        self.sprite.cur_pos.x += dx;
        self.sprite.cur_pos.y += dy;
        self.sprite.collider.pos.x += dx;
        self.sprite.collider.pos.y += dy;
    }
    //collisions look at greatest x point of one and lest of the other for x overlap
    // do the same with left overlap
    //if both overlap them collision
}
