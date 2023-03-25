use glam::*;

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.002;
const _ZOOM: f32 = 45.0;
const WORLD_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0); 

pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

pub struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    right: Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let right = WORLD_UP.cross(direction).normalize();
        let up = direction.cross(right);

        Self {
            position: position,
            direction: direction,
            up: up,
            right: right,
            yaw: YAW,
            pitch: PITCH,
        }
    }
    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.direction, self.up)
    }
    pub fn process_keyboard(&mut self, direction: Direction, delta: f32) {
        let velocity = SPEED * delta;
        match direction {
            Direction::Forward => self.position += self.direction * velocity,
            Direction::Backward => self.position -= self.direction * velocity,
            Direction::Left => self.position += self.right * velocity,
            Direction::Right => self.position -= self.right * velocity,
            Direction::Up => self.position += self.up * velocity,
            Direction::Down => self.position -= self.up * velocity,
        }
    }
    pub fn process_mouse(&mut self, mut xoffset: f32, mut yoffset: f32) {
        xoffset *= SENSITIVITY;
        yoffset *= SENSITIVITY;
        self.pitch = self.pitch.clamp(-89.9, 89.9);
        self.yaw += xoffset;
        self.pitch -= yoffset;
        self.update();
    }
    fn update(&mut self) {
        self.direction.x = self.yaw.cos() * self.pitch.cos();
        self.direction.y = self.pitch.sin();
        self.direction.z = self.yaw.sin() * self.pitch.cos();
        self.direction = self.direction.normalize();
        self.right = WORLD_UP.cross(self.direction).normalize();
        self.up = self.direction.cross(self.right);
    }
}
