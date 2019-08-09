mod error;
mod ogl;
mod util;

use crate::ogl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::ogl::color_buffer::ColorBuffer;
use crate::ogl::program::Program;
use crate::ogl::shader::Shader;
use crate::ogl::vertex::Vertex;
use crate::ogl::viewport::Viewport;

use gl::types::*;
use nalgebra::{Matrix4, Point3, Vector3};
use obj::{Obj, SimplePolygon};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::ffi::CString;
use std::path::Path;
use std::time::Duration;

pub fn main() {
    let vertices: [Vertex<f32>; 4] = [
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 0.0, 1.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 0.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5, 0.0, 0.0, 1.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 0.0, 1.0),
    ];

    let indices: [u32; 6] = [0, 1, 2, 1, 2, 3];

    let mut verticies = Vec::new();
    let mut indices = Vec::new();

    //let (models, materials) = tobj::load_obj(&Path::new("assets/obj/chalet.obj")).unwrap();
    let (models, materials) = tobj::load_obj(&Path::new("assets/obj/cube.obj")).unwrap();

    for model in models {
        let mesh = model.mesh;
        indices.extend(mesh.indices);
        for v in mesh.positions.chunks(3) {
            verticies.push(Vertex::new(v[0], v[1], v[2], 0.0, 0.0, 1.0));
        }
    }

    println!("{:#?}", vertices);
    println!("{:?}", indices);

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

    let vert_shader = Shader::vert_from_cstr(
        &CString::new(include_str!("../assets/shader/shader.vert")).unwrap(),
    )
    .unwrap();

    let frag_shader = Shader::frag_from_cstr(
        &CString::new(include_str!("../assets/shader/shader.frag")).unwrap(),
    )
    .unwrap();

    let program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let matrix_id =
        unsafe { gl::GetUniformLocation(program.id(), CString::new("MVP").unwrap().as_ptr()) };
    // Projection matrix : 45° Field of View, 4:3 ratio, display range : 0.1 unit <-> 100 units
    let projection = Matrix4::new_perspective(f32::to_radians(45.0), 4.0 / 3.0, 0.1, 100.0);
    let view = Matrix4::face_towards(
        &Point3::new(4.0, 3.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vector3::new(0.0, 1.0, 0.0),
    );

    let model = Matrix4::repeat(1.0);
    let mvp = projection * view * model;

    let vao = VertexArray::new();
    vao.bind();

    let element_buffer = ElementArrayBuffer::new();
    element_buffer.bind();
    element_buffer.static_draw_data(&indices);
    ElementArrayBuffer::unbind();

    let vbo = ArrayBuffer::new();
    vbo.bind();
    vbo.static_draw_data(&vertices);
    Vertex::<f32>::attrib_pointer();
    ArrayBuffer::unbind();
    VertexArray::unbind();

    program.use_program();

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

        color_buffer.set_color(Vector3::new(i, i, i));
        color_buffer.use_color_buffer();
        color_buffer.clear();

        unsafe {
            gl::UniformMatrix4fv(matrix_id, 1, gl::FALSE, &mvp[0]);
        }

        vao.bind();
        element_buffer.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,            // mode
                indices.len() as GLsizei, // number of indices to be rendered
                gl::UNSIGNED_INT,
                0 as *const GLvoid, // starting index in the enabled arrays
            );
        }

        ElementArrayBuffer::unbind();
        VertexArray::unbind();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    drop(gl);
}
