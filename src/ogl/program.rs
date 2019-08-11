use crate::ogl::shader::Shader;
use crate::util::*;

use std::ffi::CString;

use gl::types::*;
use nalgebra::Matrix4;

#[derive(Debug)]
pub struct Program(GLuint);

impl Program {
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        // continue with error handling here
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 1 {
            for shader in shaders {
                unsafe {
                    gl::DetachShader(program_id, shader.id());
                }
            }

            Ok(Program(program_id))
        } else {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            Err(error.to_string_lossy().into_owned())
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.0
    }

    pub fn get_uniform_location<S: Into<String>>(&self, s: S) -> GLint {
        let s = CString::new(s.into()).unwrap();

        unsafe { gl::GetUniformLocation(self.0, s.as_ptr()) }
    }

    pub fn set_3f<S: Into<String>>(&self, name: S, f: [f32; 3]) {
        let id = self.get_uniform_location(name);
        unsafe {
            gl::Uniform3f(id, f[0], f[1], f[2]);
        }
    }

    pub fn set_1f<S: Into<String>>(&self, name: S, f: f32) {
        let id = self.get_uniform_location(name);
        unsafe {
            gl::Uniform1f(id, f);
        }
    }

    pub fn set_mat4<S: Into<String>>(&self, name: S, x: Matrix4<f32>) {
        let id = self.get_uniform_location(name);
        unsafe {
            gl::UniformMatrix4fv(id, 1, gl::FALSE, &x[0]);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}
