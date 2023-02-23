use raylib_ffi::*;
use std::ffi::CString;

// player_hub is the factory for player creation
#[derive(Clone)]
pub struct Player {
    pub name: &'static str,
    color: Color,
    pub transform: Rectangle,
    pub line_angle: f32,
    has_wang: bool,
}
const WANG_LENGTH: f32 = 250.0;
impl Player {
    pub fn new(name: &'static str, color: Color, has_wang: bool) -> Self {
        Player {
            name,
            color,
            transform: Rectangle {
                x: 0.,
                y: 0.,
                width: 100.,
                height: 100.,
            },
            line_angle: 0.,
            has_wang,
        }
    }

    pub fn rotate_line_left(&mut self, amount: f32) {
        self.line_angle -= amount;
        println!("line angle: {}, Pamount: {}", self.line_angle, amount);
    }

    pub fn rotate_line_right(&mut self, amount: f32) {
        self.line_angle += amount;
    }

    fn draw_line(&self) {
        let center_x = self.transform.x + (self.transform.width / 2.);
        let center_y = self.transform.y + (self.transform.height / 2.);

        let line_end_x = center_x + self.line_angle.cos() * WANG_LENGTH;
        let line_end_y = center_y + self.line_angle.sin() * WANG_LENGTH;

        let line_start_x = self.transform.x + (self.transform.width / 2.0);
        let line_start_y = self.transform.y + (self.transform.height / 2.0);

        unsafe {
            DrawLine(
                line_start_x as i32,
                line_start_y as i32,
                line_end_x as i32,
                line_end_y as i32,
                Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            );
        }
    }
}

pub trait Renderable {
    fn draw(&self);
}

pub trait Position {
    fn set_position(&mut self, x: f32, y: f32);
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

            if self.has_wang {
                self.draw_line();
            }
        }
    }
}

impl Position for Player {
    fn set_position(&mut self, x: f32, y: f32) {
        self.transform.x = x as f32;
        self.transform.y = y as f32;
    }
}

pub trait Movable {
    fn move_left(&mut self, distance: f32);
    fn move_right(&mut self, distance: f32);
    fn move_up(&mut self, distance: f32);
    fn move_down(&mut self, distance: f32);
}

impl Movable for Player {
    fn move_left(&mut self, distance: f32) {
        self.set_position(self.transform.x - distance, self.transform.y);
    }

    fn move_right(&mut self, distance: f32) {
        self.set_position(self.transform.x + distance, self.transform.y);
    }

    fn move_up(&mut self, distance: f32) {
        self.set_position(self.transform.x, self.transform.y - distance);
    }

    fn move_down(&mut self, distance: f32) {
        self.set_position(self.transform.x, self.transform.y + distance);
    }
}
