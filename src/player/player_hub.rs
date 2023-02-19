use raylib_ffi::*;
use std::{
    ffi::{c_char},
};

// player_hub is the factory for player creation 
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct Player {
    name: &'static str,
    color: Color,
    pub Transform: Rectangle,
}

impl Player {
    pub fn new(name: &'static str, color: Color) -> Self {
        Player {
            name: name,
            color: color,
            Transform: Rectangle {
                x: 0.,
                y: 0.,
                width: 400.,
                height: 400.,
            },
        }
    }
}

#[allow(non_snake_case)]
pub trait Renderable {
    fn Draw(&self);
}

impl Renderable for Player {
    fn Draw(&self) {
        unsafe {
            DrawRectangle(
                self.Transform.x as i32,
                self.Transform.y as i32,
                self.Transform.width as i32,
                self.Transform.height as i32,
                self.color,
            );

            DrawText(
                self.name.to_owned().as_ptr() as *const c_char,
                self.Transform.x as i32 + 200,
                self.Transform.y as i32 + 200,
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