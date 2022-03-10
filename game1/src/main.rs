use engine2d::types::*;
use engine2d::image::Image;
use engine2d::text::*;
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
        state.update(0,-1);
    }
    if now_keys[VirtualKeyCode::Down as usize] && (state.sprite.cur_pos.y + state.sprite.sheetpos.sz.y) < (HEIGHT) as i32 {
        state.update(0,1);

    }
    if now_keys[VirtualKeyCode::Left as usize] && state.sprite.cur_pos.x >= 0 {
        state.update(-1,0);
    }
    if now_keys[VirtualKeyCode::Right as usize] && (state.sprite.cur_pos.x + state.sprite.sheetpos.sz.x) < (WIDTH) as i32 {
        state.update(1,0);
    }
    // Exercise for the reader: Tie y to mouse movement
}

fn render2d(assets: &Assets, state: &mut State, fb2d: &mut Image) {
    fb2d.clear(Color(128, 64, 64, 255));
    state.fc += 1;
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
    }

    // move sprite
    fb2d.bitblt(
        &state.sprite.img,
        state.sprite.sheetpos,
        state.sprite.cur_pos,
    );


    // // textbox
    // fb2d.bitblt(
    //     &state.textbox.img,
    //     state.textbox.frames[state.textbox.cur_frame],
    //     state.textbox.roomloca,
    // );

    fb2d.bitblt(
        &state.textboxes[state.textbox].img,
        state.textboxes[state.textbox].sheetpos,
        state.textboxes[state.textbox].roomloca,
    );
    for txt in state.textboxes[state.textbox].txt.iter(){
        fb2d.bitblt(
            &txt.font,
            txt.frames[txt.cur_frame],
            txt.roomloca,
        );
    }

    // // add textbox and text
    // for desc in state.rooms[state.room].desc.iter() {
    //     //for txtbx in desc.txtbx.iter() {
    //         fb2d.bitblt(
    //         &desc.img,
    //         desc.frames[desc.cur_frame],
    //         desc.roomloca,
    //     );
    //     for txt in desc.txt.iter(){
    //         fb2d.bitblt(
    //             &txt.font,
    //             txt.frames[txt.cur_frame],
    //             txt.roomloca,
    //         );
    //     }
    // }


}

fn main() {

     let house = Item {
         name: String::from("House"),
         desc: String::from("A modern house"),
         //desc: Textbox::new("A modern house"),
         sheetpos: Rect {
                     pos: Vec2i { x: 0, y: 0 },
                     sz: Vec2i { x: 180, y: 110 },
                 },
         roomloca: Vec2i { x: 83, y: 47 },
         img: Image::from_file(std::path::Path::new("content/house.png")),
         collider: Rect {
             pos: Vec2i { x: 83, y: 47},
             sz: Vec2i { x: 180, y: 97 },
            },
        frames: Vec::<Rect>::from([
            Rect {
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i { x: 180, y: 110 },
        }
        ]),
        cur_frame: 0,
        text_num: 4,
     };

    let tree = Item {
        name: String::from("Tree"),
        desc: String::from("That's a nice tree."),
        //desc: Textbox::new("That's a nice tree."),
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
        text_num: 3,
    };

    // let key = Item {
    //     name: String::from("Key"),
    //     //desc: String::from("I wonder what this opens..."),
    //     desc: Textbox::new("I wonder what this opens..."),
    //     sheetpos: Rect {
    //                 pos: Vec2i { x: 37, y: 40 },
    //                 sz: Vec2i { x: 3, y: 7 },
    //             },
    //     roomloca: Vec2i { x: 300, y: 10 },
    //     img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
    //     collider: Rect {
    //         pos: Vec2i { x: 100, y: 10 },
    //         sz: Vec2i { x: 3, y: 7 },
    //     },
    //     frames: vec![Rect {
    //         pos: Vec2i { x: 37, y: 40 },
    //         sz: Vec2i { x: 3, y: 7 },
    //     }],
    //     cur_frame: 0,
    //     text_num: 8,
    // };

    // let couch = Item {
    //     name: String::from("Couch"),
    //     //desc: String::from("Just a couch"),
    //     desc: Textbox::new("Just a couch"),
    //     sheetpos: Rect {
    //                 pos: Vec2i { x: 134, y: 0 },
    //                 sz: Vec2i { x: 69, y: 23 },
    //             },
    //     roomloca: Vec2i { x: 200, y: 100 },
    //     img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
    //     collider: Rect {
    //         pos: Vec2i { x: 200, y: 100 },
    //         sz: Vec2i { x: 69, y: 23 },
    //     },
    //     frames: vec![Rect {
    //         pos: Vec2i { x: 134, y: 0 },
    //         sz: Vec2i { x: 69, y: 23 },
    //     }],
    //     cur_frame: 0,
    //     text_num: 7,
    // };

    // let shelf = Item {
    //     name: String::from("Shelf"),
    //     //desc: String::from("I don't know any of these books"),
    //     desc: Textbox::new("I don't know any of these books"),
    //     sheetpos: Rect {
    //                 pos: Vec2i { x: 52, y: 25 },
    //                 sz: Vec2i { x: 41, y: 35 },
    //             },
    //     roomloca: Vec2i { x: 53, y: 89 },
    //     img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
    //     collider: Rect {
    //         pos: Vec2i { x: 53, y: 89 },
    //         sz: Vec2i { x: 1, y: 1 },
    //     },
    //     frames: vec![Rect {
    //         pos: Vec2i { x: 52, y: 25 },
    //         sz: Vec2i { x: 41, y: 35 },
    //     }],
    //     cur_frame: 0,
    //     text_num: 6,
    // };

    // let shrub = Item {
    //     name: String::from("Shrub"),
    //     desc: Textbox::new(""),
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
    //     text_num: 5,

    // };
    // let dresser = Item {
    //     name: String::from("Dresser"),
    //     //desc: String::from("There's nothing in this."),
    //     desc: Textbox::new("There's nothing in this."),
    //     sheetpos: Rect {
    //                 pos: Vec2i { x: 190, y: 28 },
    //                 sz: Vec2i { x: 31, y: 19 },
    //             },
    //     roomloca: Vec2i { x: 53, y: 100 },
    //     img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
    //     collider: Rect {
    //         pos: Vec2i { x: 53, y: 100 },
    //         sz: Vec2i { x: 1, y: 1 },
    //     },
    //     frames: vec![Rect {
    //         pos: Vec2i { x: 190, y: 28 },
    //         sz: Vec2i { x: 31, y: 19 },
    //     }],
    //     cur_frame: 0,
    //     text_num: 9,
    // };

    // let diary = Item {
    //     name: String::from("Diary"),
    //     desc: Textbox::new("Idk whose diary this is. It is locked."),
    //     sheetpos: Rect {
    //                 pos: Vec2i { x: 97, y: 104 },
    //                 sz: Vec2i { x: 8, y: 10 },
    //             },
    //     roomloca: Vec2i { x: 600, y: 22 },
    //     img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
    //     collider: Rect {
    //         pos: Vec2i { x: 136, y: 22 },
    //         sz: Vec2i { x: 8, y: 10 },
    //     },
    //     frames: vec![Rect {
    //         pos: Vec2i { x: 97, y: 104 },
    //         sz: Vec2i { x: 8, y: 10 },
    //     }],
    //     cur_frame: 0,
    //     text_num: 10,
    // };

    let hallway_door1 = Door {
        collider: Rect {
            pos: Vec2i { x: 51, y: 133 },
            sz: Vec2i { x: 6, y: 50 },
        },
        target: 1,
        spawn_pos: Vec2i { x: 100, y: 157 },
    };

    let hallway_door2 = Door {
        collider: Rect {
            pos: Vec2i { x: 264, y: 157 },
            sz: Vec2i { x: 6, y: 50 },
        },
        target: 0,
        spawn_pos: Vec2i { x: 100, y: 157 },
    };

    let hallway = Room {
        name: String::from("Hallway"),
        //desc: Textbox::new("ughh"),
        desc: String::from("A hallway?"),
        //items: Vec::<Item>::from([dresser, diary]),
        items: Vec::<Item>::from([]),
        img: Image::from_file(std::path::Path::new("content/hallway.png")),
        doors: Vec::<Door>::from([hallway_door1, hallway_door2]),
        floor: Rect {
            pos: Vec2i { x: 52, y: 119 },
            sz: Vec2i { x: 217, y: 92 },
        },
        text_num: 2,
    };

    let livingroom_door = Door {
        collider: Rect {
            pos: Vec2i { x: 264, y: 138},
            sz: Vec2i { x: 6, y: 20 },
        },
        target: 2,
        spawn_pos: Vec2i { x: 100, y: 157 },
    };

    let livingroom = Room {
        name: String::from("Living Room"),
        //desc: Textbox::new("ughh"),
        desc: String::from("Huh... this looks like a living room."),
        //items: Vec::<Item>::from([key, couch]),
        items: Vec::<Item>::from([]),
        img: Image::from_file(std::path::Path::new("content/room3.png")),
        doors: Vec::<Door>::from([livingroom_door]),
        floor: Rect {
            pos: Vec2i { x: 52, y: 119 },
            sz: Vec2i { x: 217, y: 92 },
        },
        text_num: 1,
    };

    let door = Door {
        collider: Rect {
            pos: Vec2i { x: 146, y: 111 },
            sz: Vec2i { x: 6, y: 50 },
        },
        target: 1,
        spawn_pos: Vec2i { x: 146, y: 111 },
    };

    let yard = Room {
        name: String::from("Front Yard"),
        //desc: Textbox::new("A mysterious field"),
        desc: String::from("A mysterious field"),
        items: Vec::<Item>::from([tree, house]),
        //items: Vec::<Item>::from([tree, house, shrub]),
        img: Image::from_file(std::path::Path::new("content/grass.png")),
        doors: Vec::<Door>::from([door]),
        floor: Rect {
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i { x: 320, y: 240 },
        },
        text_num: 0,
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
            pos: Vec2i { x: 0, y: 27 },
            sz: Vec2i {x: 12, y: 10},
        },
    };

    
    let state = State {garlic soap
        
        w: WIDTH,
        h: HEIGHT,
        fc: 0,
        color: 0,
        room: 0,
        rooms: vec![yard, livingroom, hallway],
        textbox: 0,
        textboxes: vec![Textbox::new("A mysterious field"), Textbox::new("Livingroom"), 
                        Textbox::new("A hallway"), Textbox::new("This is a nice tree"), 
                        Textbox::new("A modern house")],
        sprite: sprite,
        inventory: vec![],
    };

    engine2d::main::go(state, assets, update, render2d);
    // engine2d::main::main();
    println!("Hello, world!");
}
