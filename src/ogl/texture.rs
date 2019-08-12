use gl::types::*;

pub struct Texture(GLuint);

impl Texture {
    pub fn new() -> Texture {
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
        }

        Texture(texture_id)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn tex_image_2d(width: u32, height: u32, data: &[u8]) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                width as GLint,
                height as GLint,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const GLvoid,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            // When MAGnifying the image (no bigger mipmap available), use LINEAR filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            // When MINifying the image, use a LINEAR blend of two mipmaps, each filtered LINEARLY too
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
            // Generate mipmaps, by the way.
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.0);
        }
    }
}
