mod buffer;
mod error;
mod program;
mod shader;
mod util;
mod vertex;
mod viewport;
mod color_buffer;

use crate::buffer::{ArrayBuffer, VertexArray};
use crate::program::Program;
use crate::shader::Shader;
use crate::vertex::Vertex;
use crate::viewport::Viewport;
use crate::color_buffer::ColorBuffer;

use gl::types::*;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::ffi::CString;
use std::time::Duration;
use nalgebra::Vector3;

pub fn main() {
    let vertices: [Vertex<f32>; 3] = [
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 0.0, 1.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 0.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5, 0.0, 0.0, 1.0),
    ];

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut canvas = window.into_canvas().build().unwrap();

    let mut viewport = Viewport::for_window(900, 700);
    viewport.use_viewport();

    let mut color_buffer = ColorBuffer::from_color(Vector3::new(1.0, 1.0, 1.0));
    color_buffer.use_color_buffer();
    color_buffer.clear();

    let vert_shader =
        Shader::vert_from_cstr(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let frag_shader =
        Shader::frag_from_cstr(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let vbo = ArrayBuffer::new();
    vbo.bind();
    vbo.static_draw_data(&vertices);
    ArrayBuffer::unbind();

    let vao = VertexArray::new();
    vao.bind();

    vbo.bind();
    Vertex::<f32>::attrib_pointer();
    ArrayBuffer::unbind();
    VertexArray::unbind();

    program.use_program();
    vao.bind();
    unsafe {
        gl::DrawArrays(
            gl::TRIANGLES,             // mode
            0,                         // starting index in the enabled arrays
            vertices.len() as GLsizei, // number of indices to be rendered
        );
    }
    VertexArray::unbind();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut i = 0.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.set_size(w, h);
                    viewport.use_viewport();
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        i = (i + 0.005) % 1.0;

        unsafe {
            color_buffer.set_color(Vector3::new(i, i, i));
            color_buffer.use_color_buffer();
            color_buffer.clear();

            vao.bind();
            gl::DrawArrays(
                gl::TRIANGLES,                   // mode
                0,                               // starting index in the enabled arrays
                (vertices.len() * 3) as GLsizei, // number of indices to be rendered
            );
            VertexArray::unbind();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    drop(gl);
}
