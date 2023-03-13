extern crate gl;
extern crate glfw;

use std::time::Instant;

use glfw::{Action, Context, Key};
mod render_gl;
use crate::render_gl::*;
use glam::*;
use obj::Obj;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a windowed mode window and its OpenGL context
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

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
    }

    let model = Obj::load("./src/assets/model.obj").unwrap();
    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    // Create a Vertex Buffer Object (VBO) and Vertex Array Object (VAO) for the triangle
    let mut vbo = 0;
    let mut vao = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
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

    // Define the model matrix (transforms from model space to world space)
    let mut model_matrix = Mat4::from_translation(Vec3::new(0.0, 0.0, -5.0));

    // Define the view matrix (transforms from world space to camera/view space)
    let eye = Vec3::new(0.0, 0.0, 0.0); // camera position
    let target = Vec3::new(0.0, 0.0, -1.0); // where camera is looking
    let up = Vec3::new(0.0, 1.0, 0.0); // up vector
    let view_matrix = Mat4::look_at_rh(eye, target, up);

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
    let now = Instant::now();

    // Loop until the user closes the window
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            model_matrix = Mat4::from_translation(Vec3::new(0.0, 0.0, now.elapsed().as_secs_f32().sin()*2.0-3.0));
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
                view_matrix.to_cols_array().as_ptr(),
            );
            gl::UniformMatrix4fv(
                projection_location,
                1,
                gl::FALSE,
                projection_matrix.to_cols_array().as_ptr(),
            );

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        // Swap front and back buffers
        window.swap_buffers();
        // Poll for and process events
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            //println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    unsafe {
        gl::DeleteProgram(shader_program);
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
    }
}
