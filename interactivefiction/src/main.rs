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
        state.sprite.sheetpos.pos.x = 10;
    }
    else if dx<0{
        state.sprite.sheetpos.pos.x = 20;
    }
    else {
        state.sprite.sheetpos.pos.x = 0;
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
        state.sprite.cur_pos = Vec2i { x: 300, y: 180 };
        state.sprite.collider.pos = Vec2i { x: 300, y: 200 };
        state.room = 0;
        state.textbox= 0;
        state.loss = false;
        return;
    }
    
    let new_collider = Rect {
        pos: Vec2i { x: state.sprite.collider.pos.x as i32 - RADIUS as i32/2, y: state.sprite.collider.pos.y as i32 - RADIUS as i32/2},
        sz:  Vec2i { x: state.sprite.collider.sz.x as i32 + RADIUS as i32, y: state.sprite.collider.sz.y as i32 + RADIUS as i32},
    };
        
    for item in state.rooms[state.room].items.iter_mut() {
        println!("item");

        for rect in item.colliders.iter_mut() {
            if new_collider.touches(*rect){
                println!("touches");
                // if state.textbox != item.text_num[item.text_num.len()-1] {
                //     state.textbox += 1;
                // }
                if item.name == "FinalStar"{
                    state.room = 4;
                    state.textbox = 7;
                    state.mode = GameMode::Animation;
                }
                
                state.textbox = item.text_num[0];
                println!("{:?}", item.name);
            }
        }   
    }
   
    for door in state.rooms[state.room].doors.iter() {
        if state.sprite.collider.touches(door.collider){
            state.room = door.target;
            let offset = state.sprite.collider.pos.y - state.sprite.cur_pos.y;
            state.textbox = state.room;
            state.sprite.cur_pos = door.spawn_pos;
            state.sprite.collider.pos = Vec2i {x: door.spawn_pos.x, y: door.spawn_pos.y + offset};
        }
    }

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
                interact(state);

            }
            // if now_keys[VirtualKeyCode::Space as usize] && !prev_keys[VirtualKeyCode::Space as usize] && (state.sprite.cur_pos.x + state.sprite.sheetpos.sz.x) < (WIDTH) as i32 {
            //     // interact(state);
            //     // state.mode = GameMode::Animation;

            // }
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
            if now_keys[VirtualKeyCode::Space as usize] && !prev_keys[VirtualKeyCode::Space as usize]{
                state.mode = GameMode::Play;
            }
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
            if state.fc % 11 == 0 && state.animidx < assets.anim_frames.len() as i32 - 1{
                state.animidx += 1;
                fb2d.bitblt(
                    &assets.anim_frames[state.animidx as usize],
                    Rect {
                        pos: Vec2i { x: 0, y: 0 },
                        sz: Vec2i { x: 320, y: 240 },
                    },
                    Vec2i {
                        x: 0,
                        y: 0,
                    },
                );
            } else{
                fb2d.bitblt(
                    &assets.anim_frames[state.animidx as usize],
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
            // ...
        }
        GameMode::Transition => {
            // ...
        }
    }


    

}

fn main() {

    let door1 = Door {
        collider: Rect {
            pos: Vec2i { x: 310, y: 0 },
            sz: Vec2i { x: 10, y: 240 },
        },
        target: 1,
        spawn_pos: Vec2i { x: 5, y: 130 },
    };

    let star1 = Item {
        name: String::from("Star"),
        desc: String::from(""),
        sheetpos: Rect {
                    pos: Vec2i { x: 3, y: 71 },
                    sz: Vec2i { x: 3, y: 3 },
                },
        roomloca: Vec2i { x: 160, y: 160 },
        img: Image::from_file(std::path::Path::new("content/assets.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 160, y: 160 },
            sz: Vec2i { x: 3, y: 3 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 3, y: 71 },
            sz: Vec2i { x: 3, y: 3 },
        },
        Rect {
            pos: Vec2i { x: 7, y: 71 },
            sz: Vec2i { x: 3, y: 3 },
        },
        Rect {
            pos: Vec2i { x: 11, y: 71 },
            sz: Vec2i { x: 3, y: 3 },
        }],
        cur_frame: 0,
        text_num: vec![4],
    };

    let stars = Item {
        name: String::from("Stars"),
        desc: String::from(""),
        sheetpos: Rect {
                    pos: Vec2i { x: 0, y: 76 },
                    sz: Vec2i { x: 320, y: 24 },
                },
        roomloca: Vec2i { x: 0, y: 0 },
        img: Image::from_file(std::path::Path::new("content/assets.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i { x: 0, y: 0 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 0, y: 76 },
            sz: Vec2i { x: 320, y: 24 },
        },
        Rect {
            pos: Vec2i { x: 0, y: 101 },
            sz: Vec2i { x: 320, y: 24 },
        },
        Rect {
            pos: Vec2i { x: 11, y: 126 },
            sz: Vec2i { x: 3, y: 3 },
        }],
        cur_frame: 0,
        text_num: vec![0],
    };

    let scene1 = Room {
        name: String::from("1"),
        desc: String::from(""),
        items: Vec::<Item>::from([star1, stars.clone()]),
        img: Image::from_file(std::path::Path::new("content/street.png")),
        doors: Vec::<Door>::from([door1]),
        floor: Rect {
            pos: Vec2i { x: 0, y: 145 },
            sz: Vec2i { x: 320, y: 55 },
        },
        text_num: vec![0],
    };

    let door2 = Door {
        collider: Rect {
            pos: Vec2i { x: 316, y: 0 },
            sz: Vec2i { x: 4, y: 240 },
        },
        target: 2,
        spawn_pos: Vec2i { x: 5, y: 130 },
    };

    let sign = Item {
        name: String::from("Pizza Sign"),
        desc: String::from(""),
        sheetpos: Rect {
                    pos: Vec2i { x: 0, y: 40 },
                    sz: Vec2i { x: 64, y: 14 },
                },
        roomloca: Vec2i { x: 18, y: 78 },
        img: Image::from_file(std::path::Path::new("content/assets.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 200, y: 100 },
            sz: Vec2i { x: 0, y: 0 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 0, y: 40 },
            sz: Vec2i { x: 64, y: 14 },
        },
        Rect {
            pos: Vec2i { x: 0, y: 55 },
            sz: Vec2i { x: 64, y: 14 },
        }],
        cur_frame: 0,
        text_num: vec![0],
    };

    let star2 = Item {
        name: String::from("Star"),
        desc: String::from(""),
        sheetpos: Rect {
                    pos: Vec2i { x: 3, y: 71 },
                    sz: Vec2i { x: 3, y: 3 },
                },
        roomloca: Vec2i { x: 65, y: 160 },
        img: Image::from_file(std::path::Path::new("content/assets.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 65, y: 160 },
            sz: Vec2i { x: 3, y: 3 },
        }],
        frames: vec![Rect {
            pos: Vec2i { x: 3, y: 71 },
            sz: Vec2i { x: 3, y: 3 },
        },
        Rect {
            pos: Vec2i { x: 7, y: 71 },
            sz: Vec2i { x: 3, y: 3 },
        },
        Rect {
            pos: Vec2i { x: 11, y: 71 },
            sz: Vec2i { x: 3, y: 3 },
        }],
        cur_frame: 0,
        text_num: vec![5],
    };

    let star3 = Item {
        name: String::from("Star"),
        desc: String::from(""),
        sheetpos:  Rect {
            pos: Vec2i { x: 7, y: 71 },
            sz: Vec2i { x: 3, y: 3 },
        },
        roomloca: Vec2i { x: 234, y: 160 },
        img: Image::from_file(std::path::Path::new("content/assets.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 234, y: 160 },
            sz: Vec2i { x: 3, y: 3 },
        }],
        frames: vec![
            Rect {
                pos: Vec2i { x: 7, y: 71 },
                sz: Vec2i { x: 3, y: 3 },
            },
            Rect {
                pos: Vec2i { x: 3, y: 71 },
                sz: Vec2i { x: 3, y: 3 },
            },
            Rect {
                pos: Vec2i { x: 11, y: 71 },
                sz: Vec2i { x: 3, y: 3 },
            }],
        cur_frame: 0,
        text_num: vec![6],
    };

    let scene2 = Room {
        name: String::from("1"),
        desc: String::from(""),
        items: Vec::<Item>::from([sign, stars.clone(), star2, star3]),
        img: Image::from_file(std::path::Path::new("content/3.png")),
        doors: Vec::<Door>::from([door2]),
        floor: Rect {
            pos: Vec2i { x: 0, y: 145 },
            sz: Vec2i { x: 320, y: 55 },
        },
        text_num: vec![2],
    };

    let door3 = Door {
        collider: Rect {
            pos: Vec2i { x: 316, y: 0 },
            sz: Vec2i { x: 4, y: 240 },
        },
        target: 3,
        spawn_pos: Vec2i { x: 5, y: 130 },
    };


    let scene3 = Room {
        name: String::from("1"),
        desc: String::from(""),
        items: Vec::<Item>::from([stars.clone()]),
        img: Image::from_file(std::path::Path::new("content/2.png")),
        doors: Vec::<Door>::from([door3]),
        floor: Rect {
            pos: Vec2i { x: 0, y: 145 },
            sz: Vec2i { x: 320, y: 55 },
        },
        text_num: vec![0],
    };

    let final_star = Item {
        name: String::from("FinalStar"),
        desc: String::from(""),
        sheetpos:  Rect {
            pos: Vec2i { x: 7, y: 71 },
            sz: Vec2i { x: 3, y: 3 },
        },
        roomloca: Vec2i { x: 234, y: 160 },
        img: Image::from_file(std::path::Path::new("content/assets.png")),
        colliders: vec![Rect {
            pos: Vec2i { x: 234, y: 160 },
            sz: Vec2i { x: 3, y: 3 },
        }],
        frames: vec![
            Rect {
                pos: Vec2i { x: 7, y: 71 },
                sz: Vec2i { x: 3, y: 3 },
            },
            Rect {
                pos: Vec2i { x: 3, y: 71 },
                sz: Vec2i { x: 3, y: 3 },
            },
            Rect {
                pos: Vec2i { x: 11, y: 71 },
                sz: Vec2i { x: 3, y: 3 },
            }],
        cur_frame: 0,
        text_num: vec![6],
    };

    let door4 = Door {
        collider: Rect {
            pos: Vec2i { x: 162, y: 11 },
            sz: Vec2i { x: 20, y: 40 },
        },
        target: 4,
        spawn_pos: Vec2i { x: 90, y: 63 },
    };

    let scene4 = Room {
        name: String::from("1"),
        desc: String::from(""),
        items: Vec::<Item>::from([stars.clone(), final_star]),
        img: Image::from_file(std::path::Path::new("content/door.png")),
        doors: Vec::<Door>::from([door4]),
        floor: Rect {
            pos: Vec2i { x: 0, y: 145 },
            sz: Vec2i { x: 320, y: 55 },
        },
        text_num: vec![0],
    };

    let scene5 = Room {
        name: String::from("House"),
        desc: String::from(""),
        items: Vec::<Item>::from([stars.clone()]),
        img: Image::from_file(std::path::Path::new("content/room.png")),
        doors: Vec::<Door>::from([]),
        floor: Rect {
            pos: Vec2i { x: 70, y: 164 },
            sz: Vec2i { x: 177, y: 20 },
        },
        text_num: vec![0],
    };
    
    let assets = Assets {
        menuimg: vec![Image::from_file(std::path::Path::new("content/space.png")),
        Image::from_file(std::path::Path::new("content/title.png"))
        ],
        anim_frames: vec![
            Image::from_file(std::path::Path::new("content/frame1.png")),
            Image::from_file(std::path::Path::new("content/frame2.png")),
            Image::from_file(std::path::Path::new("content/frame3.png")), //there is no frame 4 lol
            Image::from_file(std::path::Path::new("content/frame5.png")),
            Image::from_file(std::path::Path::new("content/frame6.png")),
            Image::from_file(std::path::Path::new("content/frame7.png")),
            Image::from_file(std::path::Path::new("content/frame8.png")), //there is no 9 either
            Image::from_file(std::path::Path::new("content/frame10.png")),
            Image::from_file(std::path::Path::new("content/frame11.png")),
            Image::from_file(std::path::Path::new("content/frame12.png")),
            Image::from_file(std::path::Path::new("content/frame13.png")),
            Image::from_file(std::path::Path::new("content/frame14.png")),
            Image::from_file(std::path::Path::new("content/frame15.png")),
            Image::from_file(std::path::Path::new("content/frame16.png")),
            Image::from_file(std::path::Path::new("content/frame17.png")),
            Image::from_file(std::path::Path::new("content/frame18.png")),
            Image::from_file(std::path::Path::new("content/frame19.png")),
            Image::from_file(std::path::Path::new("content/frame20.png")),
            Image::from_file(std::path::Path::new("content/frame21.png")),
            Image::from_file(std::path::Path::new("content/frame22.png")),
        ],
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
            pos: Vec2i { x: 0, y: 0 },
            sz: Vec2i { x: 11, y: 37 },
        },
        img: Image::from_file(std::path::Path::new("content/assets.png")),
        cur_pos: Vec2i { x: 0, y: 143 },
        collider: Rect {
            pos: Vec2i { x: 0, y: 170 },
            sz: Vec2i {x: 12, y: 10},
        },
    };

    
    let state = State {
        w: WIDTH,
        h: HEIGHT,
        fc: 0,
        color: 0,
        room: 0,
        rooms: vec![scene1, scene2, scene3, scene4, scene5],
        textbox: 0,
        textboxes: vec![Textbox::new("The whole town is asleep. I wish I didn't take melatonin so often. I think I'm immune to it now."),  //room1
                        Textbox::new("Look, there are more stars here."), //room2
                        Textbox::new("I hope everything is okay..."), //room3
                        Textbox::new("I hope the world isn't disintegrating. I read somewhere it's possible."), //room4
                        Textbox::new("> Hello little star. What are you doing down here?"), //4: star1
                        Textbox::new("> Why are the stars all the way down here?"), //5: star2
                        Textbox::new("> Is there something wrong with the sky?"), //6: star2
                        Textbox::new("I guess I'm just too tired. Goodnight"), //7: star2
                        ], 
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
