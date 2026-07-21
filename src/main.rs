use macroquad::prelude::*;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

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
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up = right.cross(front).normalize();

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Space) {
            position += up * MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;

        last_mouse_position = mouse_position;

        if grabbed {
            yaw += mouse_delta.x * delta * LOOK_SPEED;
            pitch += mouse_delta.y * delta * -LOOK_SPEED;

            pitch = if pitch > 1.5 { 1.5 } else { pitch };
            pitch = if pitch < -1.5 { -1.5 } else { pitch };

            front = vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos(),
            )
            .normalize();

            right = front.cross(world_up).normalize();
            up = right.cross(front).normalize();

            x += if switch { 0.04 } else { -0.04 };
            if x >= bounds || x <= -bounds {
                switch = !switch;
            }
        }

        clear_background(LIGHTGRAY);

        world::draw(x, position, front, up);

        ui::draw(mouse_position, grabbed);

        next_frame().await
    }
}
