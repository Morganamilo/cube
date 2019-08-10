use crate::ogl::uv::UV;
use crate::ogl::vertex::Vertex;

use gl::types::*;
use std::marker::PhantomData;

pub type ArrayBuffer = Buffer<Array>;
pub type ElementArrayBuffer = Buffer<ElementArray>;

pub struct ModelBuffer {
    pub vao: VertexArray,
    pub vertices: ArrayBuffer,
    pub indices: ElementArrayBuffer,
    pub uvs: ArrayBuffer,
    pub indices_count: usize,
}

impl ModelBuffer {
    pub fn attrib_pointer(&self) {
        self.vao.bind();
        self.vertices.bind();
        Vertex::<f32>::attrib_pointer();
        ArrayBuffer::unbind();

        self.uvs.bind();
        UV::<f32>::attrib_pointer();
        ArrayBuffer::unbind();
        VertexArray::unbind();
    }

    pub fn indices_count(&self) -> usize {
        self.indices_count
    }

    pub fn draw(&self) {
        self.vao.bind();
        self.indices.bind();
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
            gl::DeleteBuffers(1, &mut self.0);
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
            gl::DeleteVertexArrays(1, &mut self.0);
        }
    }
}
