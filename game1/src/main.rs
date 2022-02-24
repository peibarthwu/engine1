use engine2d::types::*;
use engine2d::image::Image;

use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() {

    struct Assets {
        img: Image,
        colors: [Color;6]
    }
    
    struct State {
        w: usize,
        y: usize,
        color: usize,
    }
    
    let assets = Assets {
        img: Image::from_file(std::path::Path::new("content/spritesheet.png")),
        colors: [
            Color(255, 0, 0, 255),
            Color(255, 255, 0, 255),
            Color(0, 255, 0, 255),
            Color(0, 255, 255, 255),
            Color(0, 0, 255, 255),
            Color(255, 0, 255, 255),
        ]
    };

    const WIDTH: usize = 320;
    const HEIGHT: usize = 240;
    
    let state = State {
        w: WIDTH,
        y: HEIGHT,
        color: 0,
    };

    fn update(now_keys: &[bool], state: &mut State, assets:&Assets) {
        // We can actually handle events now that we know what they all are.
        if now_keys[VirtualKeyCode::Up as usize] {
            state.color = (state.color + 1) % assets.colors.len();
        }
        if now_keys[VirtualKeyCode::Down as usize] {
            // What is this if doing?
            state.color = if state.color == 0 {
                assets.colors.len() - 1
            } else {
                state.color - 1
            };
        }
        if now_keys[VirtualKeyCode::Left as usize] && state.w > 0 {
            state.w = if state.w < 4 { 0 } else { state.w - 4 };
        }
        if now_keys[VirtualKeyCode::Right as usize] && state.w < WIDTH - 1 {
            state.w = (state.w + 4).min(WIDTH - 1);
        }
        // Exercise for the reader: Tie y to mouse movement
        state.y = (state.y + 3) % HEIGHT;
    }

    fn render2d(assets: &Assets, state: &State, fb2d: &mut Image) {
        fb2d.clear(Color(128, 64, 64, 255));
        fb2d.bitblt(
            &assets.img,
            Rect {
                pos: Vec2i { x: 0, y: 16 },
                sz: Vec2i { x: 16, y: 16 },
            },
            Vec2i {
                x: fb2d.sz.x / 2,
                y: fb2d.sz.y / 2,
            },
        );
        // Then draw our new line:
        fb2d.hline(
            WIDTH / 2 - state.w / 2,
            WIDTH / 2 + state.w / 2,
            state.y,
            assets.colors[state.color],
        );
    }

    engine2d::main::go(state, assets, update, render2d);
    // engine2d::main::main();
    println!("Hello, world!");
}
