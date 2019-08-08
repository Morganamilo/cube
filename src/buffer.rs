use gl::types::*;

pub struct ArrayBuffer(GLuint);

impl ArrayBuffer {
    pub fn new() -> ArrayBuffer {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        ArrayBuffer(vbo)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * ::std::mem::size_of::<T>()) as GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
        }
    }
}

impl Drop for ArrayBuffer {
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
