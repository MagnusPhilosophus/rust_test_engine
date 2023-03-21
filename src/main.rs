extern crate gl;
extern crate glfw;

use std::time::Instant;

use glfw::{Action, Context, Key};
mod render_gl;
use crate::render_gl::*;
use glam::*;
mod simple_mesh;
use simple_mesh::Mesh;
mod camera;
use camera::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

/*
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.002;
const ZOOM: f32 = 45.0;
const WORLD_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0); 

enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    right: Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    fn new(position: Vec3) -> Self {
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
    fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.direction, self.up)
    }
    fn process_keyboard(&mut self, direction: Direction, delta: f32) {
        let velocity = SPEED * delta;
        match direction {
            Direction::Forward => self.position += self.direction * velocity,
            Direction::Backward => self.position -= self.direction * velocity,
            Direction::Left => self.position += self.right * velocity,
            Direction::Right => self.position -= self.right * velocity,
        }
    }
    fn process_mouse(&mut self, mut xoffset: f32, mut yoffset: f32) {
        xoffset *= SENSITIVITY;
        yoffset *= SENSITIVITY;
        self.yaw += xoffset;
        self.pitch -= yoffset;
        self.pitch = self.pitch.clamp(-89.9, 89.9);
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
*/
fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            "Game",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Hidden);
    window.set_cursor_pos(WINDOW_WIDTH as f64 / 2.0, WINDOW_HEIGHT as f64 / 2.0);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        gl::Enable(gl::DEPTH_TEST);
    }

    // Load shaders from source
    let vs_path = "./src/assets/shaders/vertex.glsl";
    let fs_path = "./src/assets/shaders/fragment.glsl";
    let vs_source = load_shader_source(vs_path).unwrap();
    let fs_source = load_shader_source(fs_path).unwrap();

    // Compile the shader source code into executable code
    let vertex_shader = compile_shader(&vs_source, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(&fs_source, gl::FRAGMENT_SHADER);

    // Create a shader program and link the compiled shaders
    let shader_program = link_program(vertex_shader, fragment_shader);

    // Define the projection matrix (transforms from view space to clip space)
    let aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
    let fov = 1.0;
    let near = 0.1;
    let far = 100.0;
    let projection_matrix = Mat4::perspective_rh_gl(fov, aspect_ratio, near, far);

    // Get matrices location
    let model_location = get_uniform_location(shader_program, "model");
    let view_location = get_uniform_location(shader_program, "view");
    let projection_location = get_uniform_location(shader_program, "projection");

    // Setup delta time
    let mut old_time = Instant::now();

    let mut mesh = Mesh::new("./src/assets/teapot.obj");
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0));

    let mut forward: bool = false;
    let mut backward: bool = false;
    let mut left: bool = false;
    let mut right: bool = false;

    // Loop until the user closes the window
    while !window.should_close() {
        let delta = old_time.elapsed().as_secs_f32();
        old_time = Instant::now();
        let model_matrix = Mat4::from_translation(Vec3::new(0.0, 0.0, -30.0));

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UniformMatrix4fv(
                model_location,
                1,
                gl::FALSE,
                model_matrix.to_cols_array().as_ptr(),
            );
            gl::UniformMatrix4fv(
                view_location,
                1,
                gl::FALSE,
                camera.get_view_matrix().to_cols_array().as_ptr(),
            );
            gl::UniformMatrix4fv(
                projection_location,
                1,
                gl::FALSE,
                projection_matrix.to_cols_array().as_ptr(),
            );

        }
        mesh.draw(shader_program);
        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    //println!("{} {}", xpos - WINDOW_WIDTH as f64 / 2.0, ypos - WINDOW_HEIGHT as f64 / 2.0);
                    camera.process_mouse(xpos as f32 - WINDOW_WIDTH as f32 / 2.0, ypos as f32 - WINDOW_HEIGHT as f32 / 2.0);
                    window.set_cursor_pos(WINDOW_WIDTH as f64 / 2.0, WINDOW_HEIGHT as f64 / 2.0);
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    forward = true;
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
                    forward = false;
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    backward = true;
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    backward = false;
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    left = true;
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
                    left = false;
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    right = true;
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    right = false;
                }
                _ => {}
            }
        }
        if forward {
            camera.process_keyboard(Direction::Forward, delta);
        }
        if backward {
            camera.process_keyboard(Direction::Backward, delta);
        }
        if left {
            camera.process_keyboard(Direction::Left, delta);
        }
        if right {
            camera.process_keyboard(Direction::Right, delta);
        }
    }

    unsafe {
        gl::DeleteProgram(shader_program);
    }
}
