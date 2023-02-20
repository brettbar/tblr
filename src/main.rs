mod player;

use crate::player::player_hub;
use crate::player::player_hub::Renderable;
use crate::player::player_hub::Position;
use raylib_ffi::*;
use std::ffi::CString;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
const GREY: Color = Color {
    r: 44,
    g: 44,
    b: 44,
    a: 255,
};
const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

fn main() {
    let screen_width = 1200;
    let screen_height = 800;

    unsafe {
        let title = CString::new("my rust window").unwrap();

        SetConfigFlags(ConfigFlags_FLAG_WINDOW_RESIZABLE);
        InitWindow(screen_width, screen_height, title.as_ptr());

        let mut players: Vec<player_hub::Player> = vec![
            player_hub::Player::new("Bob", RED),
            player_hub::Player::new("Sally", GREEN),
        ];

        let copy_vec = players.to_vec();
        for (i, player) in players.iter_mut().enumerate() {
            if i > 0 {
                let x = (copy_vec[i-1].transform.x as i32 + i as i32) << 8;
                let y = (copy_vec[i-1].transform.y as i32 + i as i32) << 8;
                player.set_position(x as f32, y as f32)
            }
        }

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

            // These look backwards but its to keep the camera "fixed" on player 1
            if IsKeyDown(KeyboardKey_KEY_S.try_into().unwrap()) {
                players[0].transform.y += 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_D.try_into().unwrap()) {
                players[0].transform.x += 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_W.try_into().unwrap()) {
                players[0].transform.y -= 2.;
            }
            if IsKeyDown(KeyboardKey_KEY_A.try_into().unwrap()) {
                players[0].transform.x -= 2.;
            }

            camera.target = Vector2 {
                x: players[0].transform.x + (players[0].transform.width / 2.),
                y: players[0].transform.y + (players[0].transform.height / 2.),
            };
            camera.offset = Vector2 {
                x: GetScreenWidth() as f32 / 2.,
                y: GetScreenHeight() as f32 / 2.,
            };

            // Draw
            BeginDrawing();
            {
                BeginMode2D(camera);
                {
                    ClearBackground(GREY);


                    for player in players.iter() {
                        player.draw();
                    }
                }
                EndMode2D();
            }
            EndDrawing();


            // Handle collisions 
            let p1 = players[0].transform;
            let p2 = players[1].transform;
            if CheckCollisionRecs(p1, p2) {
                // Gather collistion vector/rectangle
                let cr = GetCollisionRec(p1, p2);
                println!("{0} {1}", p1.x, p2.x);
                println!("Col Rec {0} {1}", cr.x, cr.y);
                if cr.y as i32 > 256 {
                    players[0].set_position(cr.x + p1.x, cr.y + p1.y);
                } else {
                    players[0].set_position(cr.x - p1.x, cr.y - p1.y);
                }
            }
        }
    }
}
