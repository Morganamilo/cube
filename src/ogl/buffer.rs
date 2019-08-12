use crate::ogl::normal;
use crate::ogl::render::Renderer;
use crate::ogl::uv;
use crate::ogl::vertex;

use gl::types::*;
use std::marker::PhantomData;
use tobj::Material;

pub type ArrayBuffer = Buffer<Array>;
pub type ElementArrayBuffer = Buffer<ElementArray>;

pub struct ModelBuffer {
    vao: VertexArray,
    vertices: ArrayBuffer,
    indices: ElementArrayBuffer,
    normals: ArrayBuffer,
    uvs: ArrayBuffer,
    indices_count: usize,
    material: Option<Material>,
}

impl ModelBuffer {
    pub fn new(
        vao: VertexArray,
        vertices: ArrayBuffer,
        indices: ElementArrayBuffer,
        normals: ArrayBuffer,
        uvs: ArrayBuffer,
        indices_count: usize,
        material: Option<Material>,
    ) -> ModelBuffer {
        let mb = ModelBuffer {
            vao,
            vertices,
            indices,
            normals,
            uvs,
            indices_count,
            material,
        };

        mb.attrib_pointer();
        mb
    }

    fn attrib_pointer(&self) {
        self.vao.bind();
        self.vertices.bind();
        vertex::attrib_pointer::<f32>();
        ArrayBuffer::unbind();

        self.normals.bind();
        normal::attrib_pointer::<f32>();
        ArrayBuffer::unbind();

        self.uvs.bind();
        uv::attrib_pointer::<f32>();
        ArrayBuffer::unbind();
        VertexArray::unbind();
    }

    pub fn draw(&self, renderer: &Renderer) {
        self.vao.bind();
        self.indices.bind();

        let program = &renderer.program;
        if let Some(material) = &self.material {
            program.set_3f("ambient", material.ambient);
            program.set_3f("diffuse", material.diffuse);
            program.set_3f("ambient", material.specular);
            program.set_1f("shininess", material.shininess);
        }

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,                 // mode
                self.indices_count as GLsizei, // number of indices to be rendered
                gl::UNSIGNED_INT,
                0 as *const GLvoid, // starting index in the enabled arrays
            );
        }
        ElementArrayBuffer::unbind();
        VertexArray::unbind();
    }
}

pub unsafe trait BufferType {
    const BUFFER_TYPE: GLuint;
}

pub struct Array;
unsafe impl BufferType for Array {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct ElementArray;
unsafe impl BufferType for ElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub struct Buffer<B: BufferType>(GLuint, PhantomData<B>);

impl<B: BufferType> Buffer<B> {
    pub fn new() -> Buffer<B> {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Buffer(vbo, PhantomData)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    pub fn buffer_data<T>(data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,                                          // target
                (data.len() * ::std::mem::size_of::<T>()) as GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const GLvoid,                          // pointer to data
                gl::STATIC_DRAW,                                         // usage
            );
        }
    }
}

impl<B: BufferType> Drop for Buffer<B> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}

pub struct VertexArray(GLuint);

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray(vao)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.0);
        }
    }
}
