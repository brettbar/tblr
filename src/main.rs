use raylib_ffi::*;
use std::{
    borrow::{Borrow, BorrowMut},
    ffi::{c_char, CString},
};

#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    color: Color,
    transform: Rectangle,
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

trait Renderable {
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

fn main() {
    println!("Hello, world!");

    let screen_width = 1200;
    let screen_height = 800;

    unsafe {
        let title = CString::new("my rust window").unwrap();
        let text = CString::new("Hello, World!").unwrap();

        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };

        InitWindow(screen_width, screen_height, title.as_ptr());

        let mut players: Vec<Player> = vec![
            Player::new(
                "Bob",
                Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            ),
            Player::new(
                "Sally",
                Color {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 255,
                },
            ),
        ];

        let mut camera = Camera2D {
            target: Vector2 {
                x: players[0].transform.x - (players[0].transform.width as f32 / 2.),
                y: players[0].transform.y - (players[0].transform.height as f32 / 2.),
            },
            offset: Vector2 {
                x: screen_width as f32 / 2.,
                y: screen_height as f32 / 2.,
            },
            rotation: 0.,
            zoom: 1.,
        };

        SetTargetFPS(60);

        while !WindowShouldClose() {
            // Update

            if IsKeyDown(KeyboardKey_KEY_W) {
                players[0].transform.y -= 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_A) {
                players[0].transform.x -= 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_S) {
                players[0].transform.y += 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_D) {
                players[0].transform.x += 2.;
            }

            camera.target = Vector2 {
                x: players[0].transform.x,
                y: players[0].transform.y,
            };

            // Draw
            BeginDrawing();
            {
                BeginMode2D(camera);
                {
                    ClearBackground(white);

                    for player in players.iter() {
                        player.draw();
                    }
                }
                EndMode2D();
            }
            EndDrawing();
        }
    }
}
