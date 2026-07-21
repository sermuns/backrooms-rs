use macroquad::prelude::*;

use crate::player::Player;

mod player;
mod ui;
mod world;

fn conf() -> Conf {
    Conf {
        window_title: env!("CARGO_PKG_NAME").into(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut player = Player::new(1.18, 0.0, vec3(0.0, 1.0, 0.0), world_up);

    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut cursor_grabbed = true;
    set_cursor_grab(cursor_grabbed);
    show_mouse(false);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            cursor_grabbed = !cursor_grabbed;
            set_cursor_grab(cursor_grabbed);
            show_mouse(!cursor_grabbed);
        }

        if is_key_down(KeyCode::W) {
            player.move_forwards();
        }
        if is_key_down(KeyCode::S) {
            player.move_backwards();
        }
        if is_key_down(KeyCode::A) {
            player.move_left();
        }
        if is_key_down(KeyCode::D) {
            player.move_right();
        }
        if is_key_down(KeyCode::Space) {
            player.move_up();
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;

        last_mouse_position = mouse_position;

        if cursor_grabbed {
            player.update(delta);
        }

        clear_background(LIGHTGRAY);

        world::draw(x, position, front, up);

        ui::draw(mouse_position, cursor_grabbed);

        next_frame().await
    }
}
