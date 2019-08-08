mod error;
mod program;
mod shader;
mod util;

use crate::program::Program;
use crate::shader::Shader;

use gl::types::{GLchar, GLuint};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::ffi::CString;
use std::time::Duration;

const VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut canvas = window.into_canvas().build().unwrap();

    let vert_shader =
        Shader::vert_from_cstr(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let frag_shader =
        Shader::frag_from_cstr(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (VERTICES.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            VERTICES.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    program.use_program();
    unsafe {
        gl::BindVertexArray(vao);
        gl::DrawArrays(
            gl::TRIANGLES, // mode
            0,             // starting index in the enabled arrays
            3,             // number of indices to be rendered
        );
    }

    let mut event_pump = sdl.event_pump().unwrap();
    let mut i = 0.0;
    'running: loop {
        i = (i + 0.005) % 1.0;
        println!("{}", i);

        unsafe {
            gl::ClearColor(0.3, i, i, i);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0,             // starting index in the enabled arrays
                    3,             // number of indices to be rendered
                );
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
