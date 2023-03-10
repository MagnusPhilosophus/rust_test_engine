extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use std::fs;
use std::ffi::CString;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
   let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Game", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
    }

     // Define the geometry of the triangle
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];

    // Create a Vertex Buffer Object (VBO) and Vertex Array Object (VAO) for the triangle
    let mut vbo = 0;
    let mut vao = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                       vertices.as_ptr() as *const gl::types::GLvoid,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                                3 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                                std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let vs_path = "./src/assets/shaders/vertex.glsl";
    let fs_path = "./src/assets/shaders/fragment.glsl";
    let vs_source = load_shader_source(vs_path).unwrap();
    let fs_source = load_shader_source(fs_path).unwrap();

    // Compile the shader source code into executable code
    let vertex_shader = compile_shader(&vs_source, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(&fs_source, gl::FRAGMENT_SHADER);

    // Create a shader program and link the compiled shaders
    let shader_program = link_program(vertex_shader, fragment_shader);

    // Loop until the user closes the window
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
         unsafe {
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        // Swap front and back buffers
        window.swap_buffers();
        // Poll for and process events
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

fn load_shader_source(path: &str) -> Result<String, std::io::Error> {
    let source = fs::read_to_string(path)?;
    Ok(source)
}

fn compile_shader(source: &str, shader_type: gl::types::GLuint) -> gl::types::GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = CString::new(source.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        panic!(
            "Failed to compile shader:\n{}\n{}",
            source,
            error.to_string_lossy()
        );
    }
    shader
}

// Link compiled shaders into a program and return a handle to the program
fn link_program(vertex_shader: gl::types::GLuint, fragment_shader: gl::types::GLuint) -> gl::types::GLuint {
    let shader_program;

    unsafe {
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for linking errors
        let mut success = gl::FALSE as gl::types::GLint;
        let mut log_length = 0;

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut log_length);

        if log_length > 0 {
            let error_msg = CString::new(vec![b' '; log_length as usize]).unwrap();
            gl::GetProgramInfoLog(shader_program, log_length, std::ptr::null_mut(), error_msg.as_ptr() as *mut gl::types::GLchar);

            println!("Shader program linking error: {:?}", error_msg);
        }
    }

    shader_program
}
