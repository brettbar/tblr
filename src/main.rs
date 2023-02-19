mod player;

use crate::player::player_hub;
use crate::player::player_hub::Renderable;
use raylib_ffi::*;
use std::ffi::CString;

fn main() {
    println!("Hello, world!");

    let screen_width = 1200;
    let screen_height = 800;

    unsafe {
        let title = CString::new("my rust window").unwrap();
        let _text = CString::new("Hello, World!").unwrap(); // Prefixed as unused for now

        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };

        InitWindow(screen_width, screen_height, title.as_ptr());

        let mut players: Vec<player_hub::Player> = vec![
            player_hub::Player::new(
                "Bob",
                Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            ),
            player_hub::Player::new(
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
                x: players[0].Transform.x - (players[0].Transform.width as f32 / 2.),
                y: players[0].Transform.y - (players[0].Transform.height as f32 / 2.),
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
            if IsKeyDown(KeyboardKey_KEY_W.try_into().unwrap()) {
                players[0].Transform.y -= 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_A.try_into().unwrap()) {
                players[0].Transform.x -= 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_S.try_into().unwrap()) {
                players[0].Transform.y += 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_D.try_into().unwrap()) {
                players[0].Transform.x += 2.;
            }

            camera.target = Vector2 {
                x: players[0].Transform.x + (players[0].Transform.width / 2.),
                y: players[0].Transform.y + (players[0].Transform.height / 2.),
            };

            // Draw
            BeginDrawing();
            {
                BeginMode2D(camera);
                {
                    ClearBackground(white);

                    for player in players.iter() {
                        player.Draw();
                    }
                }
                EndMode2D();
            }
            EndDrawing();
        }
    }
}
