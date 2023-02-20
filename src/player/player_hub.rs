use raylib_ffi::*;
use std::ffi::CString;

#[derive(Clone)]
struct UserCoordinates {
    x: i32,
    y: i32,
}

// player_hub is the factory for player creation
#[derive(Clone)]
pub struct Player {
    pub name: &'static str,
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
                width: 100.,
                height: 100.,
            },
        }
    }
}

pub trait Renderable {
    fn draw(&self);
}

pub trait Position {
    fn generate_position(&mut self, x: f32, y: f32);
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
                self.transform.x as i32 + (self.transform.width / 2.75) as i32,
                self.transform.y as i32 + (self.transform.height / 2.5) as i32,
                16,
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

impl Position for Player {
    fn generate_position(&mut self, x: f32, y: f32) {
        self.transform.x = x as f32;
        self.transform.y = y as f32;
    }
}
