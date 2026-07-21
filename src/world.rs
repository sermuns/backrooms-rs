use macroquad::prelude::*;

pub fn draw(x: f32, position: Vec3, front: Vec3, up: Vec3) {
    set_camera(&Camera3D {
        position: position,
        up: up,
        target: position + front,
        ..Default::default()
    });

    draw_grid(20, 1., BLACK, GRAY);

    draw_line_3d(
        vec3(x, 0.0, x),
        vec3(5.0, 5.0, 5.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
    );

    draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), GREEN);
    draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
    draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);
}
