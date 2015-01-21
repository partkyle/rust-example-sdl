extern crate sdl2;

use sdl2::video::{Window, WindowPos, OPENGL, FULLSCREEN};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::event::poll_event;
use sdl2::event::Event::{KeyDown, Quit, Window};
use sdl2::event::WindowEventId::{Enter, Leave};
use sdl2::keycode::KeyCode;
use sdl2::rect::Rect;

use std::mem::swap;

use sdl2::video::FullscreenType::{FTDesktop, FTOff};

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match Window::new("rust-sdl2 demo: Video", WindowPos::PosCentered, WindowPos::PosCentered, 600, 480, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let debugColor = Color::RGB(255, 0, 255);
    let mut colors = [Color::RGB(255, 0, 0), Color::RGB(0, 0, 255)];

    let mut grid: [[uint; 5]; 5] = [[0, 0, 0, 0, 0],
                                    [0, 0, 0, 0, 0],
                                    [0, 0, 0, 0, 0],
                                    [0, 0, 0, 0, 0],
                                    [0, 0, 0, 0, 0]];

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            if x == y {
                *cell = 1u;
            }
        }
    }

    loop {
        match poll_event() {
            Quit(_) => break,
            KeyDown(_, window, key, _, _, _) => {
                match key {
                    KeyCode::Escape => {
                        break;
                    },
                    KeyCode::F => {
                        let flags = window.get_flags();
                        if (flags & FULLSCREEN) == FULLSCREEN {
                            window.set_fullscreen(FTOff);
                        } else {
                            window.set_fullscreen(FTDesktop);
                        }
                    },
                    _ => {}
                }
            }
            sdl2::event::Event::Window(_, _, event, _, _) => {
              match event {
                sdl2::event::WindowEventId::Enter => {
                  colors[0] = Color::RGB(255, 255, 255);
                  colors[1] = Color::RGB(0, 0, 0);
                },
                sdl2::event::WindowEventId::Leave => {
                  colors[0] = Color::RGB(0, 0, 0);
                  colors[1] = Color::RGB(255, 255, 255);
                }
                _ => {}
              }
            }
            _ => {}
        }

        let _ = renderer.set_draw_color(debugColor);
        let _ = renderer.clear();

        let (windowWidth, windowHeight) = renderer.get_output_size().unwrap();

        let (rows, cols) = (5, 5);

        let mut width: i32 = (windowWidth as i32) / cols;
        let mut height: i32 = (windowHeight as i32) / rows;

        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let r = Rect{x: (x as i32) * width, y: (y as i32) * height, w: width, h: height};

                println!("{}", cell);
                renderer.set_draw_color(colors[*cell]);
                let _ = renderer.fill_rect(&r);
            }
        }

        renderer.present();
    }

    sdl2::quit();
}

