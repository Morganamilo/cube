use gl::types::*;
use nalgebra::{Scalar, Vector3};

pub fn attrib_pointer<T: Scalar>() {
    unsafe {
        gl::EnableVertexAttribArray(2); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            2,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            std::mem::size_of::<Vector3<T>>() as GLint, // stride (byte offset between consecutive attributes)
            0 as *const GLvoid,                         // offset of the first component
        );
    }
}
