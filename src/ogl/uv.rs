use gl::types::*;
use nalgebra::{Scalar, Vector2};

pub fn attrib_pointer<T: Scalar>() {
    unsafe {
        gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
        gl::VertexAttribPointer(
            1,         // index of the generic vertex attribute ("layout (location = 1)")
            2,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            std::mem::size_of::<Vector2<T>>() as GLint, // stride (byte offset between consecutive attributes)
            0 as *const GLvoid,                         // offset of the first component
        );
    }
}
