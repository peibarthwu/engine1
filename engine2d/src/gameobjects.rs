use super::types::*;
use super::image::Image;
use super::text::*;

const RADIUS: usize = 5;

#[derive(Clone)]
pub struct Item {
    pub name: String, // E.g. "Antechamber"
    //pub desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    //pub desc: Vec<Textbox>, 
    //pub desc: Textbox,
    pub desc: String,
    pub sheetpos: Rect, //make this a usize
    pub roomloca: Vec2i,
    pub img: Image,
    pub colliders: Vec<Rect>,
    pub frames: Vec<Rect>,
    pub cur_frame: usize,
    pub text_num: usize,
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
    fn anim(&mut self, fc: i32){}
}

impl Animation for Item {
    fn anim(&mut self, fc: i32){
        if self.frames.len() == 0{
            return;
        }
        if (fc % 10) == 0 {
            if self.cur_frame < self.frames.len()-1{
                self.cur_frame += 1;
            }
            else{
                self.cur_frame = 0;
            }
        }
        
        
    }
}

// #[derive(PartialEq, Eq, Clone)]
#[derive(Clone)]
pub struct Room {
    pub name: String, // E.g. "Antechamber"
    //pub desc: Vec<Textbox>, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    //pub desc: Textbox,
    pub desc: String,
    // pub txtbx_target: usize,
    pub doors: Vec<Door>,
    pub items: Vec<Item>,
    pub img: Image,
    pub floor: Rect,
    pub text_num: usize,
}

// #[derive(PartialEq, Eq, Clone, Debug)]
#[derive(Clone)]
pub struct Door {
    pub collider: Rect,
    pub target: usize, //where it goes
    pub spawn_pos: Vec2i,
}

pub struct Assets {
    pub img: Image,
    pub colors: [Color;6]
}

pub struct State {
    pub w: usize,
    pub h: usize,
    pub fc: i32,
    pub color: usize,
    pub room: usize,
    pub rooms: Vec<Room>,
    // pub textbox: Textbox,
    pub textbox: usize,
    pub textboxes: Vec<Textbox>,
    pub sprite: Sprite,
    pub inventory: Vec<String>,
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
            for rect in item.colliders.iter() {
                if new_collider.touches(*rect){
                    println!("{:?}", item.name);
                    //self.textbox = item.text_num;
                    if item.name == "Key"{
                        println!("You got the key");
                        // item.roomloca =  Vec2i { x: self.sprite.cur_pos.x as i32 + 10, y: self.sprite.cur_pos.x as i32 + 20};
                        // item.collider.sz = Vec2i { x: 0, y: 0 };
                        self.inventory.push(item.name.clone());
                    }
                    if item.name == "Diary" && self.inventory.contains(&"Key".to_string()){
                        println!("It's not polite to read someone else's diary. GAME OVER.");
                    }
                    dx = 0;
                    dy = 0;
                }
                //self.textbox = self.room;
            }   
        }

        self.sprite.cur_pos.x += dx;
        self.sprite.cur_pos.y += dy;
        self.sprite.collider.pos.x += dx;
        self.sprite.collider.pos.y += dy;
    }

    pub fn interact(&mut self) -> () {

        let new_collider = Rect {
            pos: Vec2i { x: self.sprite.collider.pos.x as i32 - RADIUS as i32 /2, y: self.sprite.collider.pos.y as i32 - RADIUS as i32/2},
            sz: Vec2i { x: self.sprite.collider.sz.x as i32 + RADIUS as i32, y: self.sprite.collider.sz.y as i32 + RADIUS as i32},
        };

        for item in self.rooms[self.room].items.iter_mut() {
            for rect in item.colliders.iter() {
                if new_collider.touches(*rect){
                    println!("{:?}", item.name);
                    self.textbox = item.text_num;
                    if item.name == "Key"{
                        println!("You got the key");
                        item.roomloca =  Vec2i { x: 10, y: 10};
                        self.inventory.push(item.name.clone());
                    }
                    if item.name == "Diary" && self.inventory.contains(&"Key".to_string()){
                        println!("It's not polite to read someone else's diary. GAME OVER.");
                        self.textbox= 16;
                        item.roomloca =  Vec2i { x: 10, y: 10};
                    }
                   
                } 
            }   
        }
       
        for door in self.rooms[self.room].doors.iter() {
            if self.sprite.collider.touches(door.collider){
                self.room = door.target;
                //get offset of collider
                let offset = self.sprite.collider.pos.y - self.sprite.cur_pos.y;
                self.sprite.cur_pos = door.spawn_pos;
                self.sprite.collider.pos = Vec2i {x: door.spawn_pos.x, y: door.spawn_pos.y + offset};
            }
        }
    }
}
