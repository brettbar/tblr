use raylib_ffi::*;
use std::ffi::CString;

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
            name,
            color,
            transform: Rectangle {
                x: 0.,
                y: 0.,
                width: 200.,
                height: 200.,
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
                CString::new(self.name).unwrap().into_raw(),
                self.transform.x as i32 + (self.transform.width / 2.) as i32,
                self.transform.y as i32 + (self.transform.height / 2.) as i32,
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
