#![windows_subsystem = "windows"]

use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Mod};
use sdl3::video::WindowFlags;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl_context = sdl3::init().expect("failed to initialize SDL");

    let video_subsystem = sdl_context
        .video()
        .expect("failed to initialize video subsystem");

    let window = video_subsystem
        .window("censors", 120, 120)
        .position_centered()
        .set_window_flags((WindowFlags::ALWAYS_ON_TOP | WindowFlags::BORDERLESS).as_u32())
        .build()
        .expect("failed to build window");

    let mut canvas = window.into_canvas();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("failed to obtain SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(key),
                    keymod,
                    ..
                } => {
                    let step = if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) {
                        8
                    } else {
                        24
                    };

                    let window = canvas.window_mut();

                    if keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD) {
                        let (mut w, mut h) = window.size();

                        match key {
                            Keycode::Up => h -= step as u32,
                            Keycode::Down => h += step as u32,
                            Keycode::Left => w -= step as u32,
                            Keycode::Right => w += step as u32,
                            _ => {}
                        }

                        window.set_size(w, h).expect("failed to set window size");
                    } else {
                        let (mut x, mut y) = window.position();

                        match key {
                            Keycode::Up => y -= step,
                            Keycode::Down => y += step,
                            Keycode::Left => x -= step,
                            Keycode::Right => x += step,
                            _ => {}
                        }

                        window.set_position(
                            sdl3::video::WindowPos::Positioned(x),
                            sdl3::video::WindowPos::Positioned(y),
                        );
                    }
                }
                _ => {}
            }
        }

        sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
