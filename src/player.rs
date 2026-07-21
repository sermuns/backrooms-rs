use macroquad::prelude::*;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

pub struct Player {
    pub yaw: f32,
    pub pitch: f32,
    pub position: Vec3,
    front: Vec3,
    right: Vec3,
    up: Vec3,
}

impl Player {
    pub fn new(yaw: f32, pitch: f32, position: Vec3, world_up: Vec3) -> Self {
        let front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        let right = front.cross(world_up).normalize();

        let up = right.cross(front).normalize();

        Self {
            yaw,
            pitch,
            position,
            front,
            right,
            up,
        }
    }

    pub fn move_forwards(&mut self) {
        self.position += self.front * MOVE_SPEED;
    }
    pub fn move_backwards(&mut self) {
        self.position -= self.front * MOVE_SPEED;
    }
    pub fn move_left(&mut self) {
        self.position -= self.right * MOVE_SPEED;
    }
    pub fn move_right(&mut self) {
        self.position += self.right * MOVE_SPEED;
    }
    pub fn move_up(&mut self) {
        self.position += self.up * MOVE_SPEED;
    }

    pub fn update(&mut self, delta: f32, mouse_delta: Vec2) {
        self.yaw += mouse_delta.x * delta * LOOK_SPEED;
        self.pitch += mouse_delta.y * delta * -LOOK_SPEED;

        self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
        self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

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
}
