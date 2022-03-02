use super::types::*;
use super::image::Image;

pub struct Text {
    pub words: String,
    pub txtbx: Vec<Textbox>, //grab from textbox.png
    pub font: Image, // grab from font.png
    pub sheetpos: Rect, // depends on the character
    pub roomloca: Vec2i, // for textbox, this is fixed for now. need to figure out for characters
    pub frames: Vec<Rect>, // where the text will be on the screen?
    pub cur_frame: usize, // idk what this is... magic number zero for now...
}

pub struct Textbox{
    pub img: Image, //grab from textbox.png
    pub sheetpos: Rect, // depends on the character
    pub roomloca: Vec2i, // for textbox, this is fixed for now. need to figure out for characters
    pub frames: Vec<Rect>, // where the text will be on the screen?
    pub cur_frame: usize, // idk what this is... magic number zero for now...
}

impl Textbox {
    pub fn new(img: Image) -> Self {
        Self {
            img, 
            sheetpos: Rect {
                pos: Vec2i { x: 0, y: 0 },
                sz: Vec2i { x: 266, y: 60 },
                },
            roomloca: Vec2i { x: 26, y: 175 },
            frames: Vec::<Rect>::from([
                Rect {
                pos: Vec2i { x: 0, y: 0 },
                sz: Vec2i { x: 266, y: 60 },
                }
                ]),
            cur_frame: 0,
        }
    }

}

impl Text {
    pub fn new(words: String) -> Self {
        Self {
            words,
            txtbx: Vec::<Textbox>::from([Textbox::new(Image::from_file(std::path::Path::new("content/textbox.png")))]),
            //([Textbox::new(Image::from_file(std::path::Path::new("content/textbox.png")))]),
            font: Image::from_file(std::path::Path::new("content/font.png")), 
            //font: Image::from_file(std::path::Path::new("engine1/engine2d/content/font.png")), // .parent()/engine2d/content
            sheetpos: Rect {
                pos: Vec2i { x: 0, y: 0 },
                sz: Vec2i { x: 6, y: 14 },
                },
            roomloca: Vec2i { x: 37, y: 178 },
            frames: Vec::<Rect>::from([
                Rect {
                pos: Vec2i { x: 0, y: 0 },
                sz: Vec2i { x: 6, y: 14 },
                }
                ]),
            cur_frame: 0,
            
        }
    }

}
