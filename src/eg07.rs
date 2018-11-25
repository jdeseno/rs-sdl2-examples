/*
This is a port of lesson 6 of Will Usher's SDL tutorial to Rust. See
http://www.willusher.io/sdl2%20tutorials/2013/12/18/lesson-6-true-type-fonts-with-sdl_ttf

In this lesson we’ll see how to perform basic True Type font rendering with the
SDL_ttf extension library.

This example uses a public domain font downloaded from
http://www.publicdomainfiles.com/show_file.php?id=13501609932993
 */

extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

use sdl2::pixels::{Color};
use sdl2::event::{Event,WindowEventId};
use sdl2::rect::{Rect,Point};
use sdl2::render::TextureQuery;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    let ctx = match sdl2::init() {
        Ok(ctx) => ctx,
        Err(err) => panic!("failed to init sdl2 lib: {}", err)
    };
    let video_ctx = match ctx.video() {
        Ok(ctx) => ctx,
        Err(err) => panic!("failed to init sdl2 video: {}", err)
    };
    let ttf_ctx = match sdl2_ttf::init() {
        Ok(ctx) => ctx,
        Err(err) => panic!("failed to init sdl2 ttf: {}", err)
    };

    let window  = match video_ctx.window("eg07", SCREEN_WIDTH, SCREEN_HEIGHT)
                .position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    renderer.clear();

    let font = match ttf_ctx.load_font(&Path::new("res/cruft.ttf"), 62) {
        Ok(font) => font,
        Err(err) => panic!("failed to load font: {}", err)
    };

    let surface = match font.render("TTF fonts are cool!")
        .blended(Color::RGBA(255, 0, 0, 255)) {
            Ok(font) => font,
            Err(err) => panic!("failed to blend font: {}", err)
        };
    let image = renderer.create_texture_from_surface(&surface).unwrap();

    let TextureQuery{ width: iw, height: ih, .. } = image.query();
    let dst = Rect::from_center(Point::new(SCREEN_WIDTH as i32 / 2,
                                           SCREEN_HEIGHT as i32 / 2),
                                iw, ih);
    renderer.copy(&image, None, Some(dst));
    renderer.present();

    let mut events = ctx.event_pump().unwrap();

    // Loop until we receive a QuitEvent or press escape.
    'event : loop {
        for event in events.wait_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::Window {win_event_id, ..} => {
                    match win_event_id {
                        WindowEventId::Exposed => renderer.present(),
                        _ => (),
                    }
                }
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => break 'event,
                        _ => continue,
                    }
                }
                _               => continue
            }
        }
    }
}

