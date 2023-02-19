use raylib_ffi::*;
use std::{
    ffi::{c_char},
};

// player_hub is the factory for player creation 
#[derive(Clone, Copy)]
pub struct Player {
    name: &'static str,
    color: Color,
    pub transform: Rectangle,
}

impl Player {
    pub fn new(name: &'static str, color: Color) -> Self {
        Player {
            name: name,
            color: color,
            transform: Rectangle {
                x: 0.,
                y: 0.,
                width: 400.,
                height: 400.,
            },
        }
    }
}

pub trait Renderable {
    fn draw(&self);
}

impl Renderable for Player {
    fn draw(&self) {
        unsafe {
            DrawRectangle(
                self.transform.x as i32,
                self.transform.y as i32,
                self.transform.width as i32,
                self.transform.height as i32,
                self.color,
            );

            DrawText(
                self.name.to_owned().as_ptr() as *const c_char,
                self.transform.x as i32 + 200,
                self.transform.y as i32 + 200,
                32,
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            );
        }
    }
}