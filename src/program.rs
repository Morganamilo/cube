use gl::types::GLuint;

use crate::shader::Shader;
use crate::util::*;

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
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}
