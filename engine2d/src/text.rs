use super::types::*;
use super::image::Image;

#[derive(Clone)]
pub struct Text {
    //pub txtbx: Vec<Textbox>, //grab from textbox.png
    pub font: Image, // grab from font.png
    pub sheetpos: Rect, 
    pub roomloca: Vec2i, 
    pub frames: Vec<Rect>, 
    pub cur_frame: usize, 
}
#[derive(Clone)]
pub struct Textbox{
    pub txt: Vec<Text>,
    pub img: Image, //grab from textbox.png
    pub sheetpos: Rect, 
    pub roomloca: Vec2i,
    pub frames: Vec<Rect>, 
    pub cur_frame: usize, 
}

impl Textbox {
    pub fn new(words: &str) -> Self {
        Self {
            //txt:Vec::<Text>::from([Text::new(words:String))]),
            //txt: Vec::<Text>::from([Text::new(words)]),
            txt: build_text_vec(words),
            img: Image::from_file(std::path::Path::new("content/textbox.png")),
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
    pub fn new(words: &str, roomloca: Vec2i) -> Self {
        Self {
            font: Image::from_file(std::path::Path::new("content/font.png")), 
            //font: Image::from_file(std::path::Path::new("engine1/engine2d/content/font.png")), // .parent()/engine2d/content
            sheetpos: string_to_vec_rect(words),

            roomloca,
            frames: Vec::<Rect>::from([
                string_to_vec_rect(words)]),
            cur_frame: 0,
            
        }
    }

}

// 'letter_space'{ //2 pixel wide
//     char = Rect {pos: Vec2i { x: 0, y: 0 },sz: Vec2i { x: 6, y: 14 },}

// 'word_space'{ // 3 pixels
//     char = Rect {pos: Vec2i { x: 0, y: 0 },sz: Vec2i { x: 6, y: 14 },}
fn build_text_vec(words:&str) -> Vec::<Text>{
    let mut v : Vec<Text> = vec![];
    let mut roomloca: Vec2i = Vec2i { x: 37, y: 178 };

    for text_char in words.chars(){
        let s = &String::from(text_char);
        let txt = Text::new(s, roomloca);
        let pix = txt.sheetpos.sz.x;
        // check that there is room left in the line. if not, start next line
        if s == " " && roomloca.x > 230{
            roomloca = Vec2i{x: 37, y: roomloca.y+14};
        } else{
            v.push(txt);
            roomloca = Vec2i{x: roomloca.x + pix, y: roomloca.y};
            v.push(Text::new("letter_space", Vec2i{x: roomloca.x+1, y: roomloca.y}));
            roomloca = Vec2i{x: roomloca.x+1, y: roomloca.y};
        }
    }
    return v;
}
fn write_sheet_rect(char_type: &str, x_pos: i32, char_width: i32) -> Rect{
    if char_type == "upper"{
        Rect { pos: Vec2i { x: x_pos, y: 3 }, sz: Vec2i { x: char_width, y: 14 },}
    } else if char_type == "lower"{
        Rect { pos: Vec2i { x: x_pos, y: 17 }, sz: Vec2i { x: char_width, y: 14 },}
    } else {
        Rect { pos: Vec2i { x: x_pos, y: 31 }, sz: Vec2i { x: char_width, y: 14 },}
    }

}


fn string_to_vec_rect(words: &str) -> Rect{
    //for text_char in words.chars(){
        let text_char = words;
        match &text_char as &str {
            //"A"=> Rect { pos: Vec2i { x: 0, y: 3 }, sz: Vec2i { x: 6, y: 14 },},
            "A" => write_sheet_rect("upper", 0, 6),
            "B" => write_sheet_rect("upper", 9, 5),
            "C" => write_sheet_rect("upper", 16, 6),
            "D"=>  write_sheet_rect("upper", 24, 6),
            "E" => write_sheet_rect("upper", 32, 5),
            "F" => write_sheet_rect("upper", 39, 5),
            "G"=> write_sheet_rect("upper", 46, 6),
            "H" => write_sheet_rect("upper", 54, 5),
            "I" => write_sheet_rect("upper", 61, 5),
            "J"=> write_sheet_rect("upper", 68, 6),
            "K" => write_sheet_rect("upper", 76, 5),
            "L" => write_sheet_rect("upper", 83, 5),
            "M"=> write_sheet_rect("upper", 90, 7),

            "N" => write_sheet_rect("upper", 99, 6),
            "O" => write_sheet_rect("upper", 107, 6),
            "P"=> write_sheet_rect("upper", 115, 5),
            "Q" => write_sheet_rect("upper", 122, 6),
            "R" => write_sheet_rect("upper", 130, 5),
            "S"=> write_sheet_rect("upper", 137, 5),
            "T" => write_sheet_rect("upper", 144, 5),
            "U" => write_sheet_rect("upper", 151, 5),
            "V"=> write_sheet_rect("upper", 158, 5),
            "W" => write_sheet_rect("upper", 165, 7),
            "X" => write_sheet_rect("upper", 174, 5),
            "Y" => write_sheet_rect("upper", 181, 5),
            "Z" => write_sheet_rect("upper", 188, 4),

            "a" => write_sheet_rect("lower", 0, 4),
            "b" => write_sheet_rect("lower", 6, 4),
            "c" => write_sheet_rect("lower", 12, 4),
            "d" => write_sheet_rect("lower", 18, 4),
            "e" => write_sheet_rect("lower", 24, 4),
            "f" => write_sheet_rect("lower", 30, 4),
            "g" => write_sheet_rect("lower", 36, 4),
            "h" => write_sheet_rect("lower", 42, 4),
            "i" => write_sheet_rect("lower", 48, 1),
            "j" => write_sheet_rect("lower", 51, 3),
            "k" => write_sheet_rect("lower", 56, 4),
            "l" => write_sheet_rect("lower", 62, 1),
            "m" => write_sheet_rect("lower", 65, 7),

            "n" => write_sheet_rect("lower", 74, 5),
            "o" => write_sheet_rect("lower", 81, 4),
            "p" => write_sheet_rect("lower", 87, 4),
            "q" => write_sheet_rect("lower", 93, 5),
            "r" => write_sheet_rect("lower", 100, 5),
            "s" => write_sheet_rect("lower", 107, 5),
            "t" => write_sheet_rect("lower", 114, 3),
            "u" => write_sheet_rect("lower", 119, 4),
            "v" => write_sheet_rect("lower", 125, 5),
            "w" => write_sheet_rect("lower", 132, 5),
            "x" => write_sheet_rect("lower", 139, 5),
            "y" => write_sheet_rect("lower", 146, 4),
            "z" => write_sheet_rect("lower", 152, 4),

            "0" => write_sheet_rect("other", 0, 4),
            "1" => write_sheet_rect("other", 6, 2),
            "2" => write_sheet_rect("other", 10, 4),
            "3" => write_sheet_rect("other", 16, 4),
            "4" => write_sheet_rect("other", 22, 4),
            "5" => write_sheet_rect("other", 28, 4),
            "6" => write_sheet_rect("other", 34, 4),
            "7" => write_sheet_rect("other", 40, 4),
            "8" => write_sheet_rect("other", 46, 4),
            "9" => write_sheet_rect("other", 52, 4),

            "!" => write_sheet_rect("other", 58, 1),
            "." => write_sheet_rect("other", 61, 1),
            "?" => write_sheet_rect("other", 64, 4),
            "," => write_sheet_rect("other", 70, 2),
            "(" => write_sheet_rect("other", 74, 3),
            ")" => write_sheet_rect("other", 79, 3),
            "[" => write_sheet_rect("other", 84, 2),
            "]" => write_sheet_rect("other", 88, 2),
            "'" => write_sheet_rect("other", 92, 2),
            ":" => write_sheet_rect("other", 96, 1),

            ";" => write_sheet_rect("other", 99, 2),
            "/" => write_sheet_rect("other", 103, 4),
            "$" => write_sheet_rect("other", 109, 5),
            "%" => write_sheet_rect("other", 116, 10),
            "-" => write_sheet_rect("other", 128, 3),
            "+" => write_sheet_rect("other", 133, 5),
            "=" => write_sheet_rect("other", 140, 4),
            "&" => write_sheet_rect("other", 146, 7),
            "{" => write_sheet_rect("other", 155, 4), //left quote
            "}" => write_sheet_rect("other", 161, 4), //right quote
            "<" => write_sheet_rect("other", 167, 4),
            ">" => write_sheet_rect("other", 173, 4),
            
            "letter_space" => write_sheet_rect("other", 179, 1), // blank space between letters
            " " => write_sheet_rect("other", 179, 3),
            _=> write_sheet_rect("other", 64, 4), // if not valid char, put ?
        }
    }
//}