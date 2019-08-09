use gl::types::*;
use nalgebra::{Scalar, Vector3};

#[derive(Debug)]
pub struct Vertex<T: Scalar> {
    pub pos: Vector3<T>,
    pub col: Vector3<T>,
}

impl<T: Scalar> Vertex<T> {
    pub fn new(x: T, y: T, z: T, r: T, g: T, b: T) -> Vertex<T> {
        Vertex {
            pos: Vector3::new(x, y, x),
            col: Vector3::new(r, g, b),
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
                std::mem::size_of::<Vertex<T>>() as GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(),                          // offset of the first component
            );

            gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
            gl::VertexAttribPointer(
                1,         // index of the generic vertex attribute ("layout (location = 1)")
                3,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                std::mem::size_of::<Vertex<T>>() as GLint, // stride (byte offset between consecutive attributes)
                std::mem::size_of::<Vector3<T>>() as *const GLvoid, // offset of the first component
            );
        }
    }
}
