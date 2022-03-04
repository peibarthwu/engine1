use engine2d::types::*;
use engine2d::image::Image;
use engine2d::text::Text;
use engine2d::gameobjects::*;



use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const TILE: usize = 22;




fn update(now_keys: &[bool], state: &mut State, assets:&Assets) {
    // We can actually handle events now that we know what they all are.
    if now_keys[VirtualKeyCode::Up as usize] && state.sprite.cur_pos.y >= 0 {
        // state.sprite.cur_pos.y -= 1;
        // state.sprite.moveself(0, -1, &state.room);
        state.update(0,-1);
    }
    if now_keys[VirtualKeyCode::Down as usize] && (state.sprite.cur_pos.y + state.sprite.sheetpos.sz.y) < (HEIGHT) as i32 {
        // What is this if doing?
        // state.sprite.moveself(0, 1, &state.room);
        state.update(0,1);

    }
    if now_keys[VirtualKeyCode::Left as usize] && state.sprite.cur_pos.x >= 0 {
        // state.w = if state.w < 4 { 0 } else { state.w - 4 };
        // state.sprite.moveself(-1, 0, &state.room);
        state.update(-1,0);

    }
    if now_keys[VirtualKeyCode::Right as usize] && (state.sprite.cur_pos.x + state.sprite.sheetpos.sz.x) < (WIDTH) as i32 {
        // state.sprite.moveself(1, 0, &state.room);
        state.update(1,0);

    }
    // Exercise for the reader: Tie y to mouse movement
}

fn render2d(assets: &Assets, state: &mut State, fb2d: &mut Image) {
    fb2d.clear(Color(128, 64, 64, 255));

    fb2d.bitblt(
        &state.rooms[state.room].img,
        Rect {
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i { x: 320, y: 240 },
        },
        Vec2i {
            x: 0,
            y: 0,
        },
    );
    
    // //add assets on top
    for item in state.rooms[state.room].items.iter_mut() {
        fb2d.bitblt(
            &item.img,
            item.frames[item.cur_frame],
            item.roomloca,
        );
        item.anim();
    }

    // move sprite
    fb2d.bitblt(
        &state.sprite.img,
        state.sprite.sheetpos,
        state.sprite.cur_pos,
    );

    // add textbox and text
    for desc in state.rooms[state.room].desc.iter() {
        for txtbx in desc.txtbx.iter() {
            fb2d.bitblt(
            &txtbx.img,
            txtbx.frames[txtbx.cur_frame],
            txtbx.roomloca,
        );
    }
}

    for desc in state.rooms[state.room].desc.iter() {
        fb2d.bitblt(
            &desc.font,
            desc.frames[desc.cur_frame],
            desc.roomloca,
        );
    }


}

fn main() {

     let house = Item {
         name: String::from("House"),
         desc: String::from("A modern house"),
         sheetpos: Rect {
                     pos: Vec2i { x: 0, y: 0 },
                     sz: Vec2i { x: 180, y: 110 },
                 },
         roomloca: Vec2i { x: 83, y: 47 },
         img: Image::from_file(std::path::Path::new("content/house.png")),
         collider: Rect {
             pos: Vec2i { x: 83, y: 47},
             sz: Vec2i { x: 180, y: 70 },
            },
        frames: Vec::<Rect>::from([
            Rect {
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i { x: 180, y: 110 },
        }
        ]),
        cur_frame: 0,
     };

     

    let tree = Item {
        name: String::from("Tree"),
        desc: String::from(""),
        sheetpos: Rect {
                    pos: Vec2i { x: 0, y: 119 },
                    sz: Vec2i { x: 49, y: 51 },
                },
        roomloca: Vec2i { x: 10, y: 60 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        collider: Rect {
            pos: Vec2i { x: 10, y: 60 },
            sz: Vec2i { x: 49, y: 51 },
        },
        frames: Vec::<Rect>::from([
            Rect {
            pos: Vec2i { x: 0, y: 119 },
            sz: Vec2i { x: 49, y: 51 },
            },
            // Rect {
            //     pos: Vec2i { x: 51, y: 119 },
            //     sz: Vec2i { x: 49, y: 51 },
            // }
        ]),
        cur_frame: 0,
    };

    // let shrub = Item {
    //     name: String::from("Shrub"),
    //     desc: String::from(""),
    //     sheetpos: Rect {
    //                 pos: Vec2i { x: 0, y: 70 },
    //                 sz: Vec2i { x: 22, y: 14 },
    //             },
    //     roomloca: Vec2i { x: 200, y: 200 },
    //     img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
    //     collider: Rect {
    //         pos: Vec2i { x: 200, y: 200 },
    //         sz: Vec2i { x: 22, y: 14 },
    //     },
    //     frames: Vec::<Rect>::from([
    //         Rect {
    //             pos: Vec2i { x: 0, y: 70 },
    //             sz: Vec2i { x: 22, y: 14 },
    //     }
    //     ]),
    //     cur_frame: 0,

    // };

    let livingroom = Room {
        name: String::from("Front Yard"),
        desc: Vec::<Text>::from([Text::new(String::from("ughh"))]),
        items: Vec::<Item>::from([]),
        img: Image::from_file(std::path::Path::new("content/room3.png")),
        doors: Vec::<Door>::from([]),
    };

    let door = Door {
        collider: Rect {
            pos: Vec2i { x: 136, y: 111 },
            sz: Vec2i { x: 26, y: 50 },
        },
        target: 1,
    };

    let yard = Room {
        name: String::from("Front Yard"),
        desc: Vec::<Text>::from([Text::new(String::from("A mysterious field"))]),
        items: Vec::<Item>::from([tree, house]),
        img: Image::from_file(std::path::Path::new("content/grass.png")),
        doors: Vec::<Door>::from([door]),
    };
    
    let assets = Assets {
        img: Image::from_file(std::path::Path::new("content/room3.png")),
        colors: [
            Color(255, 0, 0, 255),
            Color(255, 255, 0, 255),
            Color(0, 255, 0, 255),
            Color(0, 255, 255, 255),
            Color(0, 0, 255, 255),
            Color(255, 0, 255, 255),
        ]
    };

    let mut sprite = Sprite {
        sheetpos: Rect {
            pos: Vec2i { x: 28, y: 178 },
            sz: Vec2i { x: 12, y: 37 },
        },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        cur_pos: Vec2i { x: 0, y: 0 },
        collider: Rect {
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i {x: 12, y: 37},
        },
    };

    
    let state = State {
        w: WIDTH,
        h: HEIGHT,
        color: 0,
        room: 0,
        rooms: vec![yard, livingroom],
        sprite: sprite,
    };

    engine2d::main::go(state, assets, update, render2d);
    // engine2d::main::main();
    println!("Hello, world!");
}
