use engine2d::types::*;
use engine2d::image::Image;
use engine2d::text::*;
use engine2d::gameobjects::*;



use winit::event::{VirtualKeyCode};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const RADIUS: usize = 5;


fn update_state(state: &mut State, mut dx: i32, mut dy: i32) -> () {
    let new_collider = Rect {
        pos: Vec2i { x: state.sprite.collider.pos.x as i32 + dx, y: state.sprite.collider.pos.y as i32 + dy},
        sz: state.sprite.collider.sz,
    };

    if dx>0 {
        state.sprite.sheetpos.pos.x = 39;
    }
    else if dx<0{
        state.sprite.sheetpos.pos.x = 51;
    }
    else {
        state.sprite.sheetpos.pos.x = 28;
    }
   
    if state.rooms[state.room].floor.contains(new_collider) == false {
        dx = 0;
        dy = 0;
    }

    for item in state.rooms[state.room].items.iter() {
        for rect in item.colliders.iter() {
            if new_collider.touches(*rect){
                dx = 0;
                dy = 0;
            }
        }
    }
    state.sprite.cur_pos.x += dx;
    state.sprite.cur_pos.y += dy;
    state.sprite.collider.pos.x += dx;
    state.sprite.collider.pos.y += dy;
}

fn interact(state: &mut State){
    if state.loss{
        state.menuidx = 0;
        state.mode = GameMode::Menu;
        state.inventory = vec![];
        state.sprite.cur_pos = Vec2i { x: 300, y: 180 };
        state.sprite.collider.pos = Vec2i { x: 300, y: 200 };
        state.room = 0;
        state.textbox= 0;
        state.loss = false;
        return;
    }
    let new_collider = Rect {
        pos: Vec2i { x: state.sprite.collider.pos.x as i32 - RADIUS as i32 /2, y: state.sprite.collider.pos.y as i32 - RADIUS as i32/2},
        sz: Vec2i { x: state.sprite.collider.sz.x as i32 + RADIUS as i32, y: state.sprite.collider.sz.y as i32 + RADIUS as i32},
    };
    
        state.textbox = state.room;
    
    for item in state.rooms[state.room].items.iter_mut() {
        for rect in item.colliders.iter_mut() {
            if new_collider.touches(*rect){
                println!("{:?}", item.name);
                state.textbox = item.text_num[0];
                if item.name == "Bunny"{
                    state.textbox= 20;
                    item.frames[0].pos =  Vec2i { x: 100, y: 60};
                    state.inventory.push(item.name.clone());
                }
                if item.name == "Key"{
                    println!("You got the key");
                    //item.roomloca =  Vec2i { x: 10, y: 10};
                    //rect.pos =  Vec2i { x: 10, y: 10};
                    state.inventory.push(item.name.clone());
                }
                if item.name == "Diary" && state.inventory.contains(&"Key".to_string()){
                    println!("It's not polite to read someone else's diary. GAME OVER.");
                    // item.roomloca =  Vec2i { x: 10, y: 10};
                    // rect.pos =  Vec2i { x: 10, y: 10};
                    state.inventory.push(item.name.clone());
                    state.textbox= 16;
                    state.loss = true;

                }
               
            }
        }   
    }
   
    for door in state.rooms[state.room].doors.iter() {
        if state.sprite.collider.touches(door.collider){
            //check if they have the front door key
            if state.inventory.contains(&"Bunny".to_string()){
                state.room = door.target;
                //get offset of collider
                let offset = state.sprite.collider.pos.y - state.sprite.cur_pos.y;
                state.sprite.cur_pos = door.spawn_pos;
                state.sprite.collider.pos = Vec2i {x: door.spawn_pos.x, y: door.spawn_pos.y + offset};
            }
            else{
                state.textbox= 21; //print door is locked
            }
        }
    }

    state.mode = GameMode::Play;
}


fn update(now_keys: &[bool], prev_keys: &[bool], state: &mut State, assets:&Assets) {
    // We can actually handle events now that we know what they all are.
    match state.mode {
        GameMode::Play => {
            if now_keys[VirtualKeyCode::Up as usize] && state.sprite.cur_pos.y >= 0 {
                update_state(state, 0, -1);
            }
            if now_keys[VirtualKeyCode::Down as usize] && (state.sprite.cur_pos.y + state.sprite.sheetpos.sz.y) < (HEIGHT) as i32 {
                update_state(state, 0, 1);
        
            }
            if now_keys[VirtualKeyCode::Left as usize] && state.sprite.cur_pos.x >= 0 {
                update_state(state, -1, 0);
            }
            if now_keys[VirtualKeyCode::Right as usize] && (state.sprite.cur_pos.x + state.sprite.sheetpos.sz.x) < (WIDTH) as i32 {
                update_state(state, 1, 0);
            }
            if now_keys[VirtualKeyCode::Space as usize] && !prev_keys[VirtualKeyCode::Space as usize] && (state.sprite.cur_pos.x + state.sprite.sheetpos.sz.x) < (WIDTH) as i32 {
                interact(state);
            }
            // ...
        }
        GameMode::Menu => {
            if now_keys[VirtualKeyCode::Space as usize] && !prev_keys[VirtualKeyCode::Space as usize]{
                if state.menuidx < assets.menuimg.len() as i32 -1{
                    state.menuidx += 1;
                }
                else{
                    state.mode = GameMode::Play;
                }
            }
            // ...
        }
        GameMode::Animation => {
            // ...
        }
        GameMode::Transition => {
            
            // ...
        }
    }

   
    
    // Exercise for the reader: Tie y to mouse movement

}

fn render2d(assets: &Assets, state: &mut State, fb2d: &mut Image) {
    fb2d.clear(Color(0, 0, 0, 255));
    state.fc += 1;
    match state.mode {
        GameMode::Play => {
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
                item.anim(state.fc);
            }
        
            // move sprite
            fb2d.bitblt(
                &state.sprite.img,
                state.sprite.sheetpos,
                state.sprite.cur_pos,
            );
        
        
            //textbox
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
        }
        GameMode::Menu => {
            fb2d.bitblt(
                &assets.menuimg[state.menuidx as usize],
                Rect {
                    pos: Vec2i { x: 0, y: 0 },
                    sz: Vec2i { x: 320, y: 240 },
                },
                Vec2i {
                    x: 0,
                    y: 0,
                },
            );
        }
        GameMode::Animation => {
            // ...
        }
        GameMode::Transition => {
            // ...
        }
    }


    

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
         colliders: vec![Rect {
             pos: Vec2i { x: 83, y: 47},
             sz: Vec2i { x: 128, y: 32 },
            },
            Rect {
                pos: Vec2i { x: 83, y: 79},
                sz: Vec2i { x: 180, y: 74 },
               }
            ],
        frames: Vec::<Rect>::from([
            Rect {
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i { x: 180, y: 110 },
        }
        ]),
        cur_frame: 0,
        text_num: vec![5],
     };

    let tree = Item {
        name: String::from("Tree"),
        desc: String::from("That's a nice tree."),
        //desc: Textbox::new("That's a nice tree."),
        sheetpos: Rect {
                    pos: Vec2i { x: 0, y: 119 }, // h = 32
                    sz: Vec2i { x: 49, y: 51 },
                },
        roomloca: Vec2i { x: 10, y: 60 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 10, y: 60 },
            sz: Vec2i { x: 49, y: 32 },
            },
            Rect {
                pos: Vec2i { x: 26, y: 92 },
                sz: Vec2i { x: 11, y: 19 },
            }
            ],
        frames: Vec::<Rect>::from([
            Rect {
            pos: Vec2i { x: 0, y: 119 },
            sz: Vec2i { x: 49, y: 51 },
            },
            Rect {
                pos: Vec2i { x: 51, y: 119 },
                sz: Vec2i { x: 49, y: 51 },
            }
        ]),
        cur_frame: 0,
        text_num: vec![6],
    };

    let tree1 = Item {
        name: String::from("Tree"),
        desc: String::from("That's a nice tree."),
        sheetpos: Rect {
                    pos: Vec2i { x: 0, y: 119 },
                    sz: Vec2i { x: 49, y: 51 },
                },
        roomloca: Vec2i { x: 30, y: 100 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 30, y: 100 },
            sz: Vec2i { x: 49, y: 32 },
            },
            Rect {
                pos: Vec2i { x: 46, y: 132 },
                sz: Vec2i { x: 11, y: 19 },
            }
        ],
        frames: Vec::<Rect>::from([
            Rect {
            pos: Vec2i { x: 0, y: 119 },
            sz: Vec2i { x: 49, y: 51 },
            },
            Rect {
                pos: Vec2i { x: 51, y: 119 },
                sz: Vec2i { x: 49, y: 51 },
            }
        ]),
        cur_frame: 0,
        text_num: vec![7],
    };

    let fence = Item {
        name: String::from("Fence"),
        desc: String::from("A pixel fence"),
        //desc: Textbox::new("That's a nice tree."),
        sheetpos: Rect {
                    pos: Vec2i { x: 0, y: 0 }, 
                    sz: Vec2i { x: 320, y: 22 },
                },
        roomloca: Vec2i { x: 0, y: 218 },
        img: Image::from_file(std::path::Path::new("content/fence.png")),
        colliders: vec![Rect {
            pos: Vec2i {x: 0, y: 218 },
            sz: Vec2i { x: 320, y: 22 },
            }],
        frames: Vec::<Rect>::from([
            Rect {
                pos: Vec2i { x: 0, y: 0 },
                sz: Vec2i { x: 320, y: 22 },
            }]),
        cur_frame: 0,
        text_num: vec![19],
    };

    let key = Item {
        name: String::from("Key"),
        desc: String::from("I wonder what this opens..."),
        sheetpos: Rect {
                    pos: Vec2i { x: 37, y: 40 },
                    sz: Vec2i { x: 3, y: 7 },
                },
        roomloca: Vec2i { x: 100, y: 150 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 100, y: 150 },
            sz: Vec2i { x: 3, y: 7 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 37, y: 40 },
            sz: Vec2i { x: 3, y: 7 },
        }],
        cur_frame: 0,
        text_num: vec![10],
    };

    let couch = Item {
        name: String::from("Couch"),
        desc: String::from("Just a couch"),
        sheetpos: Rect {
                    pos: Vec2i { x: 134, y: 0 },
                    sz: Vec2i { x: 69, y: 23 },
                },
        roomloca: Vec2i { x: 200, y: 100 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 200, y: 100 },
            sz: Vec2i { x: 69, y: 23 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 134, y: 0 },
            sz: Vec2i { x: 69, y: 23 },
        }],
        cur_frame: 0,
        text_num: vec![14],
    };

    let shelf = Item {
        name: String::from("Shelf"),
        desc: String::from("I don't know any of these books"),
        sheetpos: Rect {
                    pos: Vec2i { x: 52, y: 25 },
                    sz: Vec2i { x: 41, y: 32 },
                },
        roomloca: Vec2i { x: 53, y: 89 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 53, y: 89 },
            sz: Vec2i {  x: 41, y: 32 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 52, y: 25 },
            sz: Vec2i { x: 41, y: 32 },
        }],
        cur_frame: 0,
        text_num: vec![8],
    };

    let shrub = Item {
        name: String::from("Shrub"),
        desc: String::from(""),
        sheetpos: Rect {
                    pos: Vec2i { x: 0, y: 69 },
                    sz: Vec2i { x: 68, y: 14 },
                },
        roomloca: Vec2i { x: 181, y: 150 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 181, y: 150},
            sz: Vec2i { x: 68, y: 14 },
        }],
        frames: Vec::<Rect>::from([
            Rect {
                pos: Vec2i { x: 0, y: 69 },
                sz: Vec2i { x: 68, y: 14 },
            }
        ]),
        cur_frame: 0,
        text_num: vec![9],
    };

    let bunny = Item {
        name: String::from("Bunny"),
        desc: String::from(""),
        sheetpos: Rect {
                    pos: Vec2i { x: 69, y: 60 },
                    sz: Vec2i { x: 28, y: 28 },
                },
        roomloca: Vec2i { x: 280, y: 100 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 280, y: 100 },
            sz: Vec2i { x: 28, y: 28},
        }],
        frames: Vec::<Rect>::from([
            Rect {
                pos: Vec2i { x: 69, y: 60 },
                sz: Vec2i { x: 28, y: 28 },
            },
        ]),
        cur_frame: 0,
        text_num: vec![9],
    };

    let dresser = Item {
        name: String::from("Dresser"),
        desc: String::from("There's nothing in this."),
        sheetpos: Rect {
                    pos: Vec2i { x: 190, y: 28 },
                    sz: Vec2i { x: 31, y: 19 },
                },
        roomloca: Vec2i { x: 53, y: 100 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 53, y: 100 },
            sz: Vec2i { x: 31, y: 19 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 190, y: 28 },
            sz: Vec2i { x: 31, y: 19 },
        }],
        cur_frame: 0,
        text_num: vec![15],
    };

    let diary = Item {
        name: String::from("Diary"),
        desc: String::from("Idk whose diary this is. It is locked."),
        sheetpos: Rect {
                    pos: Vec2i { x: 97, y: 104 },
                    sz: Vec2i { x: 8, y: 10 },
                },
        roomloca: Vec2i { x: 220, y: 110 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 220, y: 110},
            sz: Vec2i { x: 8, y: 10 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 97, y: 104 },
            sz: Vec2i { x: 8, y: 10 },
        }],
        cur_frame: 0,
        text_num: vec![11],
    };

    let hallway_door1 = Door {
        collider: Rect {
            pos: Vec2i { x: 51, y: 133 },
            sz: Vec2i { x: 6, y: 50 },
        },
        target: 1, //living room
        spawn_pos: Vec2i { x: 224, y: 138 },
    };

    let hallway_door2 = Door {
        collider: Rect {
            pos: Vec2i { x: 264, y: 157 },
            sz: Vec2i { x: 6, y: 50 },
        },
        target: 0, //outside
        spawn_pos: Vec2i { x: 130, y: 157 },
    };

    let hallway_door3 = Door {
        collider: Rect {
            pos: Vec2i { x: 164, y: 81 },
            sz: Vec2i { x: 25, y: 38 },
        },
        target: 3, //bedroom1
        spawn_pos: Vec2i { x: 112, y: 110 },
    };

    let hallway_door4 = Door {
        collider: Rect {
            pos: Vec2i { x: 239, y: 81 },
            sz: Vec2i { x: 25, y: 38 },
        },
        target: 5, //kitchen
        spawn_pos: Vec2i { x: 141, y: 100 },
    };

    let table = Item {
        name: String::from("Table"),
        desc: String::from("It looks like someone just had dinner."),
        sheetpos: Rect {
                    pos: Vec2i { x: 153, y: 87 },
                    sz: Vec2i { x: 61, y: 28 },
                },
        roomloca: Vec2i { x: 128, y: 151 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 128, y: 160},
            sz: Vec2i { x: 61, y: 18 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 153, y: 87 },
            sz: Vec2i { x: 61, y: 28 },
        }],
        cur_frame: 0,
        text_num: vec![17],
    };

    let shelf2 = Item {
        name: String::from("Shelf"),
        desc: String::from("I don't know any of these books."),
        sheetpos: Rect {
                    pos: Vec2i { x: 94, y: 23 },
                    sz: Vec2i { x: 31, y: 34 },
                },
        roomloca: Vec2i { x: 203, y: 89 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 203, y: 89},
            sz: Vec2i { x: 30, y: 34 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 94, y: 23 },
            sz: Vec2i { x: 31, y: 34 },
        }],
        cur_frame: 0,
        text_num: vec![18],
    };

    let hallway = Room {
        name: String::from("Hallway"),
        desc: String::from("There are so many doors."),
        items: Vec::<Item>::from([dresser, table, shelf2]),
        img: Image::from_file(std::path::Path::new("content/hallway2.png")),
        doors: Vec::<Door>::from([hallway_door1, hallway_door2, hallway_door3, hallway_door4]),
        floor: Rect {
            pos: Vec2i { x: 52, y: 108 },
            sz: Vec2i { x: 217, y: 76 },
        },
        text_num: vec![2],
    };

    let bedroom_door1 = Door {
        collider: Rect {
            pos: Vec2i { x: 112, y: 152 },
            sz: Vec2i { x: 22, y: 5 },
        },
        target: 2,
        spawn_pos: Vec2i { x: 164, y: 101 },
    };

    let bedroom_door2 = Door {
        collider: Rect {
            pos: Vec2i { x: 194, y: 152 },
            sz: Vec2i { x: 22, y: 5 }
        },
        target: 5,
        spawn_pos: Vec2i { x: 95, y: 100 },
    };

    let bed2 = Item {
        name: String::from("Bed"),
        //desc: String::from("There's nothing in this."),
        desc: String::from("There's nothing in this."),
        sheetpos: Rect {
                    pos: Vec2i { x: 130, y: 25 },
                    sz: Vec2i { x: 20, y: 39 },
                },
        roomloca: Vec2i { x: 159, y: 82 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 159, y: 82 },
            sz: Vec2i { x: 18, y: 39 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 130, y: 25 },
            sz: Vec2i { x: 20, y: 39 },
        }],
        cur_frame: 0,
        text_num: vec![13],
    };

    

    let bed1 = Item {
        name: String::from("Bed"),
        desc: String::from("There's nothing in this."),
        sheetpos: Rect {
                    pos: Vec2i { x: 0, y: 24 },
                    sz: Vec2i { x: 36, y: 40 },
                },
        roomloca: Vec2i { x: 69, y: 102 },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 69, y: 102 },
            sz: Vec2i { x: 34, y: 40 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 0, y: 24 },
            sz: Vec2i { x: 36, y: 40  },
        }],
        cur_frame: 0,
        text_num: vec![12],
    };


    let bedroom1 = Room {
        name: String::from("Bedroom"),
        desc: String::from("This is my bedroom"),
        items: Vec::<Item>::from([bed1]),
        img: Image::from_file(std::path::Path::new("content/bedroom1.png")),
        doors: Vec::<Door>::from([bedroom_door1]),
        floor: Rect {
            pos: Vec2i { x: 69, y: 102 },
            sz: Vec2i { x: 86, y: 54 },
        },
        text_num: vec![3],
    };

    let bedroom2 = Room {
        name: String::from("Bedroom"),
        desc: String::from("Idk whose room this is"),
        items: Vec::<Item>::from([bed2, diary]),
        img: Image::from_file(std::path::Path::new("content/bedroom2.png")),
        doors: Vec::<Door>::from([bedroom_door2]),
        floor: Rect {
            pos: Vec2i { x: 159, y: 102 },
            sz: Vec2i { x: 86, y: 54 },
        },
        text_num: vec![4],
    };

    let kitchen_door = Door {
        collider: Rect {
            pos: Vec2i { x: 95, y: 150 },
            sz: Vec2i { x: 20, y: 5 },
        },
        target: 2,
        spawn_pos: Vec2i { x: 242, y: 130 },
    };

    let kitchen_door2 = Door {
        collider: Rect {
            pos: Vec2i { x: 197, y: 118 },
            sz: Vec2i { x: 20, y: 10 },
        },
        target: 4, //bedroom2
        spawn_pos: Vec2i { x: 164, y: 101 },
    };

    let kitchen  = Room {
        name: String::from("Kitchen"),
        desc: String::from("Idk whose room this is"),
        items: Vec::<Item>::from([]),
        img: Image::from_file(std::path::Path::new("content/kitchen.png")),
        doors: Vec::<Door>::from([kitchen_door,kitchen_door2]),
        floor: Rect {
            pos: Vec2i { x: 86, y: 111 }, //-45 +23
            sz: Vec2i { x: 128, y: 47 },
        },
        text_num: vec![22],
    };

    let livingroom_door = Door {
        collider: Rect {
            pos: Vec2i { x: 264, y: 138},
            sz: Vec2i { x: 6, y: 20 },
        },
        target: 2,
        spawn_pos: Vec2i { x: 71, y: 110 },
    };

    let livingroom = Room {
        name: String::from("Living Room"),
        desc: String::from("ughh"),
        items: Vec::<Item>::from([key, couch, shelf]),
        img: Image::from_file(std::path::Path::new("content/livingroom.png")),
        doors: Vec::<Door>::from([livingroom_door]),
        floor: Rect {
            pos: Vec2i { x: 52, y: 107 },
            sz: Vec2i { x: 217, y: 96 },
        },
        text_num: vec![1],
    };

    let door = Door {
        collider: Rect {
            pos: Vec2i { x: 146, y: 111 },
            sz: Vec2i { x: 6, y: 50 },
        },
        target: 2,
        spawn_pos: Vec2i { x: 244, y: 122 },
    };

    let yard = Room {
        name: String::from("Front Yard"),
        desc: String::from("A mysterious field. Press space to use doors."),
        items: Vec::<Item>::from([tree, tree1, house, shrub, fence, bunny]),
        img: Image::from_file(std::path::Path::new("content/grass.png")),
        doors: Vec::<Door>::from([door]),
        floor: Rect {
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i { x: 320, y: 240 },
        },
        text_num: vec![0],
    };
    
    let assets = Assets {
        menuimg: vec![Image::from_file(std::path::Path::new("content/space.png")),
        Image::from_file(std::path::Path::new("content/title.png"))
        ],
        anim_frames: vec![],
        colors: [
            Color(255, 0, 0, 255),
            Color(255, 255, 0, 255),
            Color(0, 255, 0, 255),
            Color(0, 255, 255, 255),
            Color(0, 0, 255, 255),
            Color(255, 0, 255, 255),
        ]
    };

    let sprite = Sprite {
        sheetpos: Rect {
            pos: Vec2i { x: 28, y: 178 },
            sz: Vec2i { x: 12, y: 37 },
        },
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        cur_pos: Vec2i { x: 300, y: 180 },
        collider: Rect {
            pos: Vec2i { x: 300, y: 200 },
            sz: Vec2i {x: 12, y: 17},
        },
    };

    
    let state = State {
        
        w: WIDTH,
        h: HEIGHT,
        fc: 0,
        color: 0,
        room: 0,
        rooms: vec![yard, livingroom, hallway, bedroom1, bedroom2, kitchen],
        textbox: 0,
        textboxes: vec![Textbox::new("What a mysterious field... Press SPACE to use doors and interact."), //yard
                        Textbox::new("A livingroom? "), //livingroom
                        Textbox::new("Ughh... there are so many doors"), //hallway
                        Textbox::new("This almost looks like my bedroom. But I've never been here before."), //bedroom1
                        Textbox::new("I don't know whose room this is..."), //bedroom2
                        Textbox::new("A modern house. I wonder who lives here."), //house
                        Textbox::new("That's a nice tree."), //tree
                        Textbox::new("That's a tree."), //tree1
                        Textbox::new("I don't know any of these books"), //shelf
                        Textbox::new("My favorite shrubs."), //shrub
                        Textbox::new("I got another key! I wonder what this opens..."), //key
                        Textbox::new("I don't know whose diary this is. It is locked."), //diary
                        Textbox::new("Someone made the bed."), //bed1
                        Textbox::new("I'm not sure who sleeps here..."), //bed2
                        Textbox::new("Just a couch. I don't feel like sitting down."), //couch
                        Textbox::new("There's nothing in this. I wonder why they have this dresser if they just keep it empty."), //dresser
                        Textbox::new("It's not polite to read someone else's diary. GAME OVER. Press space to restart."), // diary end game
                        Textbox::new("It looks like someone just had dinner"), // table
                        Textbox::new("These are all my favorite books! I think I would get along with whoever owns them."), // shelf
                        Textbox::new("A pixel fence."), // fence
                        Textbox::new("This shrub almost looks like my old rabbit. It gave me a key! I wonder what this opens."), // 20: bunny
                        Textbox::new("The house is locked."), // 
                        Textbox::new("Just a kitchen.")], // 
        sprite: sprite,
        inventory: vec![],
        mode: GameMode::Menu,
        menuidx: 0,
        animidx: 0,
        loss: false,
    };

    engine2d::main::go(state, assets, update, render2d);
    // engine2d::main::main();
    println!("Hello, world!");
}
