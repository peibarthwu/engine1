#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add<Vec2i> for Vec2i {
    type Output = Self;

    fn add(self, other: Vec2i) -> <Self as std::ops::Add<Vec2i>>::Output {
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    pub pos: Vec2i,
    pub sz: Vec2i,
}

impl Rect {
    pub fn contains(&self, other: Rect) -> bool {
        let br = self.pos + self.sz;
        let obr = other.pos + other.sz;
        self.pos.x <= other.pos.x && self.pos.y <= other.pos.y && obr.x <= br.x && obr.y <= br.y
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

unsafe impl vulkano::format::Pixel for Color {
    fn ensure_accepts(
        format: vulkano::format::Format,
    ) -> std::result::Result<(), vulkano::format::IncompatiblePixelsType> {
        // TODO: Be more strict: accept only if the format has a matching AcceptsPixels impl.
        if format.size().map_or(false, |x| {
            x % std::mem::size_of::<Self>() as vulkano::DeviceSize == 0
        }) {
            Ok(())
        } else {
            Err(vulkano::format::IncompatiblePixelsType)
        }
    }
    fn rate(format: vulkano::format::Format) -> u32 {
        (format.size().expect("this format cannot accept pixels")
            / std::mem::size_of::<Self>() as vulkano::DeviceSize) as u32
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Item {
    pub name: String, // E.g. "Antechamber"
    pub desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    pub sheetpos: Rect,
    pub roomloca: Vec2i,
}

pub struct Tile {
    pub sheetpos: Rect,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Room {
    pub name: String, // E.g. "Antechamber"
    pub desc: String, // E.g. "Dark wood paneling covers the walls.  The gilded northern doorway lies open."
    pub doors: Vec<Door>,
    floor: Vec<Item>,
    items: Vec<Item>
}



#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Door {
    target: String, // More about this in a minute
    triggers:Vec<String>, // e.g. "go north", "north"
    message: Option<String> // What message, if any, to print when the doorway is traversed
    // Any other info about the door would go here
}