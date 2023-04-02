use glam::*;
use perlin2d::PerlinNoise2D;
const ROWS: usize = 100;
const COLS: usize = 100;

pub struct Terrain {
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
}

impl Terrain {
    pub fn new() -> Self {
        let mut mesh = Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertices: vec![],
            indices: vec![],
        };
        // Loading model
        let perlin = PerlinNoise2D::new(6, 1.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, 101);
        let mut grid = [[0f64; COLS]; ROWS];
        for i in 0..ROWS{
            for j in 0..COLS{
                grid[i][j] = perlin.get_noise(i as f64, j as f64) + 0.5;
            }
        }
        for i in 0..ROWS{
            for j in 0..COLS{
                mesh.vertices.push(i as f32);
                mesh.vertices.push(grid[i][j] as f32 * 3.0);
                mesh.vertices.push(j as f32);
            }
        }

        for i in 0..(ROWS-1){
            for j in 0..(COLS-1){
                mesh.vertices.push(i as f32);
                mesh.vertices.push((grid[i][j] as f32 + 1.0) * 5.0);
                mesh.vertices.push(j as f32);
                let v0 = (i * ROWS) + j;
                let v1 = v0 + 1;
                let v2 = ((i + 1) * ROWS) + j;
                let v3 = v2 + 1;

                mesh.indices.push(v0 as u32);
                mesh.indices.push(v1 as u32);
                mesh.indices.push(v2 as u32);
                mesh.indices.push(v2 as u32);
                mesh.indices.push(v1 as u32);
                mesh.indices.push(v3 as u32);
            }
        }


        unsafe {
            gl::GenBuffers(1, &mut mesh.vbo);
            gl::GenBuffers(1, &mut mesh.ebo);
            gl::GenVertexArrays(1, &mut mesh.vao);

            gl::BindVertexArray(mesh.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                mesh.indices.as_ptr() as *const gl::types::GLvoid,
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

        mesh
    }

    pub fn draw(&mut self, shader_program: u32) {
        unsafe {
            gl::UseProgram(shader_program);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

impl Drop for Terrain {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
