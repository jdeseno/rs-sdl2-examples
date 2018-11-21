extern crate sdl2;

use std::path::Path;

use sdl2::event::{Event,WindowEventId};
use sdl2::surface::{Surface};
use sdl2::keyboard::Keycode;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx.window("eg05", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    // Load a surface.
    // Surfaces live in system RAM, so they aren't ideal for performance.
    let surface = match Surface::load_bmp(&Path::new("res/ice-troll.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load surface: {}", err)
    };

    // Convert a surface to a texture.
    // Textures can be used more efficiently by the GPU. (If one is available.)
    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(err)    => panic!("failed to convert surface: {:?}", err)
    };

    let _ = renderer.clear();
    // Display the texture.
    // Omitting the src & dst Rect arguments will cause our image to stretch across the entire buffer.
    // Try passing Some(surface.rect()) for src & dst instead of None & see how things change.
    let _ = renderer.copy(&texture, None, None);
    let _ = renderer.present();

    let mut events = ctx.event_pump().unwrap();

    // loop until we receive a QuitEvent or press escape.
    'event : loop {
        for event in events.wait_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::Window {win_event_id, ..} => {
                    match win_event_id {
                        // refresh our window, for example if it is no longer
                        // covered by other windows.
                        WindowEventId::Exposed => renderer.present(),
                        _ => (),
                    }
                }
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'event
                    }
                }
                _               => continue
            }
        }
    }
}

