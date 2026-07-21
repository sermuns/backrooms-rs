use macroquad::prelude::*;

pub fn draw(mouse_position: Vec2, grabbed: bool) {
    set_default_camera();
    draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

    draw_text(
        format!("X: {} Y: {}", mouse_position.x, mouse_position.y).as_str(),
        10.0,
        48.0 + 18.0,
        30.0,
        BLACK,
    );
    draw_text(
        format!("Press <TAB> to toggle mouse grab: {grabbed}").as_str(),
        10.0,
        48.0 + 42.0,
        30.0,
        BLACK,
    );
}
