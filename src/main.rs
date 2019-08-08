mod error;
mod program;
mod shader;
mod util;
mod vertex;
mod buffer;

use crate::program::Program;
use crate::shader::Shader;
use crate::vertex::Vertex;
use crate::buffer::{ArrayBuffer, VertexArray};

use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ffi::CString;
use std::time::Duration;

use nalgebra::Vector3;

const VERTICES: [f32; 54] = [
    // positions      // colors
    -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 0.0, 1.0, 0.5, 0.5, -0.5, 0.0, 0.0, 1.0,
    0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 0.0, 1.0,
];

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
        i = (i + 0.005) % 1.0;
        println!("{}", i);

        unsafe {
            gl::ClearColor(0.3, i, i, i);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            vao.bind();
            gl::DrawArrays(
                gl::TRIANGLES,                   // mode
                0,                               // starting index in the enabled arrays
                (vertices.len() * 3) as GLsizei, // number of indices to be rendered
            );
            VertexArray::unbind();
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

    drop(gl);
}
