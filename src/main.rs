use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    camera::visibility::RenderLayers,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

const BACKROOMS_YELLOWS: &[Color] = &[
    Color::srgb_u8(228, 230, 168),
    Color::srgb_u8(225, 226, 187),
    Color::srgb_u8(214, 222, 167),
    Color::srgb_u8(195, 203, 110),
    Color::srgb_u8(198, 197, 139),
];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_world))
        .add_systems(Update, (grab_mouse, move_player))
        .run();
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

const FOV_RADIANS: f32 = 90.0_f32.to_radians();

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::default(),
        children![(
            WorldModelCamera,
            Camera3d::default(),
            Projection::from(PerspectiveProjection {
                fov: FOV_RADIANS,
                ..default()
            }),
        ),],
    ));
}

const WIDTH_ROOMS: usize = 16;
const HEIGHT_ROOMS: usize = 16;
const ROOM_SIZE_METERS: usize = 16;
const HALF_ROOM_SIZE_METERS: f32 = ROOM_HEIGHT_METERS as f32 / 2.;
const ROOM_HEIGHT_METERS: f32 = 2.0;

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_plane = meshes.add(Plane3d::new(
        Vec3::Y,
        Vec2::new(
            (WIDTH_ROOMS * ROOM_SIZE_METERS) as f32,
            (HEIGHT_ROOMS * ROOM_SIZE_METERS) as f32,
        ),
    ));
    let ground_material = materials.add(BACKROOMS_YELLOWS[1]);
    commands.spawn((
        Mesh3d(ground_plane.clone()),
        MeshMaterial3d(ground_material.clone()),
    ));

    let ceiling_plane = meshes.add(Plane3d::new(
        -Vec3::Y,
        Vec2::new(
            (WIDTH_ROOMS * ROOM_SIZE_METERS) as f32,
            (HEIGHT_ROOMS * ROOM_SIZE_METERS) as f32,
        ),
    ));
    commands.spawn((
        Mesh3d(ceiling_plane),
        MeshMaterial3d(ground_material),
        Transform::from_xyz(0., ROOM_HEIGHT_METERS, 0.),
    ));

    let wall = meshes.add(Plane3d::new(
        Vec3::Z,
        Vec2::new(ROOM_SIZE_METERS as f32, ROOM_HEIGHT_METERS),
    ));
    let wall_material = materials.add(BACKROOMS_YELLOWS[3]);

    for row in 0..WIDTH_ROOMS {
        for col in 0..HEIGHT_ROOMS {
            let dx = row as isize - WIDTH_ROOMS as isize / 2;
            let dz = col as isize - HEIGHT_ROOMS as isize / 2;

            let translation = Vec3::new(
                (dx * ROOM_SIZE_METERS as isize) as f32 + HALF_ROOM_SIZE_METERS,
                0.,
                (dz * ROOM_SIZE_METERS as isize) as f32 + HALF_ROOM_SIZE_METERS,
            );

            commands.spawn((
                SpotLight {
                    color: BACKROOMS_YELLOWS[0],
                    // shadow_maps_enabled: true,
                    ..default()
                },
                Transform::from_translation(translation.with_y(ROOM_HEIGHT_METERS - 1.))
                    .looking_at(translation.with_z(0.), Vec3::Y),
                RenderLayers::default(),
            ));

            for (offset, angle) in [
                (
                    Vec3::new(HALF_ROOM_SIZE_METERS, ROOM_HEIGHT_METERS / 2.0, 0.0),
                    FRAC_PI_2,
                ),
                (
                    Vec3::new(-HALF_ROOM_SIZE_METERS, ROOM_HEIGHT_METERS / 2.0, 0.0),
                    -FRAC_PI_2,
                ),
                (
                    Vec3::new(0.0, ROOM_HEIGHT_METERS / 2.0, HALF_ROOM_SIZE_METERS),
                    0.0,
                ),
                (
                    Vec3::new(0.0, ROOM_HEIGHT_METERS / 2.0, -HALF_ROOM_SIZE_METERS),
                    PI,
                ),
            ] {
                commands.spawn((
                    Mesh3d(wall.clone()),
                    MeshMaterial3d(wall_material.clone()),
                    Transform {
                        translation: translation + offset,
                        rotation: Quat::from_rotation_y(angle),
                        ..default()
                    },
                ));
            }
        }
    }
}

fn grab_mouse(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let mouse_delta = accumulated_mouse_motion.delta;

    if mouse_delta != Vec2::ZERO {
        let delta_yaw = -mouse_delta.x * camera_sensitivity.x;
        let delta_pitch = -mouse_delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }

    const WALK_SPEED: f32 = 0.1;

    let forward_xz = WALK_SPEED * (*transform.forward()).with_y(0.);
    if key.pressed(KeyCode::KeyW) {
        transform.translation += forward_xz;
    } else if key.pressed(KeyCode::KeyS) {
        transform.translation -= forward_xz;
    }

    let right_xz = WALK_SPEED * (*transform.right()).with_y(0.);
    if key.pressed(KeyCode::KeyD) {
        transform.translation += right_xz;
    } else if key.pressed(KeyCode::KeyA) {
        transform.translation -= right_xz;
    }
}
