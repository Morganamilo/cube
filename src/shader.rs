use crate::error::{self, Error};
use crate::util::*;

use gl::types::{GLchar, GLuint};
use std::ffi::{CStr, CString};
use std::ptr;

#[derive(Debug)]
pub struct Shader(GLuint);

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}

impl Shader {
    pub fn id(&self) -> GLuint {
        self.0
    }

    pub fn from_string(s: String, kind: GLuint) -> error::Result<Shader> {
        let s = CString::new(s).unwrap();
        Self::from_cstr(&s, kind)
    }

    pub fn vert_from_cstr(s: &CStr) -> error::Result<Shader> {
        Self::from_cstr(s, gl::VERTEX_SHADER)
    }

    pub fn frag_from_cstr(s: &CStr) -> error::Result<Shader> {
        Self::from_cstr(s, gl::FRAGMENT_SHADER)
    }

    pub fn from_cstr(s: &CStr, kind: GLuint) -> error::Result<Shader> {
        let id = unsafe { gl::CreateShader(kind) };

        unsafe {
            gl::ShaderSource(id, 1, &s.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 1 {
            Ok(Shader(id))
        } else {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }

            Err(Error::ShaderCompileFail(
                error.to_string_lossy().into_owned(),
            ))
        }
    }
}
