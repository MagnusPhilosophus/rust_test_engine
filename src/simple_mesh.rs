use std::ops::Index;

use assimp::import::structs::PrimitiveType::{Line, Point};
use assimp::import::structs::SortByPrimitiveType;
use assimp::import::Importer;
use glam::*;

#[repr(C)]
struct Vertex {
    position: Vec3,
    normal: Vec3,
    tex_cords: Vec2,
}

#[repr(C)]
struct Texture {
    id: u32,
    ttype: String,
}

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture>,
}

impl Mesh {
    pub fn new(path: &str) -> Self {
        let mut mesh = Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertices: vec![],
            indices: vec![],
            textures: vec![],
        };
        // Loading model
        let mut importer = Importer::new();
        importer.triangulate(true);
        importer.flip_uvs(true);
        importer.sort_by_primitive_type(|args: &mut SortByPrimitiveType| {
            args.enable = true;
            args.remove = vec![Line, Point];
        });
        let scene = importer.read_file(path).unwrap();

        let a_mesh = scene.mesh(0).unwrap();
        for vertex in a_mesh.vertex_iter() {
            mesh.vertices.push(Vertex {position: Vec3::from(<[f32; 3]>::from(vertex)), normal: Vec3::default(), tex_cords: Vec2::default()});
        }
        for face in a_mesh.face_iter() {
            mesh.indices.push(face.index(0).clone());
            mesh.indices.push(face.index(1).clone());
            mesh.indices.push(face.index(2).clone());
        }
        unsafe {
            gl::GenBuffers(1, &mut mesh.vbo);
            gl::GenBuffers(1, &mut mesh.ebo);
            gl::GenVertexArrays(1, &mut mesh.vao);

            gl::BindVertexArray(mesh.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
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

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                (std::mem::size_of::<f32>()*3) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                (std::mem::size_of::<f32>()*6) as *const gl::types::GLvoid,
            );

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

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
