use std::fs;
use std::ffi::CString;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn load_shader_source(path: &str) -> Result<String, std::io::Error> {
    let source = fs::read_to_string(path)?;
    Ok(source)
}

pub fn compile_shader(source: &str, shader_type: gl::types::GLuint) -> gl::types::GLuint {
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
pub fn link_program(vertex_shader: gl::types::GLuint, fragment_shader: gl::types::GLuint) -> gl::types::GLuint {
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

pub fn get_uniform_location(shader_program: u32, location_name: &str) -> i32{
    let location_name = CString::new(location_name).unwrap().into_raw();
    unsafe {gl::GetUniformLocation(shader_program, location_name)}
}
