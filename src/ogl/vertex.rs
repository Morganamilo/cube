use gl::types::*;
use nalgebra::{Scalar, Vector3};

#[derive(Debug)]
pub struct Vertex<T: Scalar> {
    pub pos: Vector3<T>,
}

impl<T: Scalar> Vertex<T> {
    pub fn new(x: T, y: T, z: T) -> Vertex<T> {
        Vertex {
            pos: Vector3::new(x, y, z),
        }
    }

    pub fn attrib_pointer() {
        unsafe {
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0,         // index of the generic vertex attribute ("layout (location = 0)")
                3,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                std::mem::size_of::<Vector3<T>>() as GLint, // stride (byte offset between consecutive attributes)
                0 as *const GLvoid,                         // offset of the first component
            );
        }
    }
}
