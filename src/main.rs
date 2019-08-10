mod error;
mod ogl;
mod util;

use crate::ogl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::ogl::color_buffer::ColorBuffer;
use crate::ogl::program::Program;
use crate::ogl::renderer::Renderer;
use crate::ogl::resources::{Model, ResourceManager};
use crate::ogl::shader::Shader;
use crate::ogl::texture::Texture;
use crate::ogl::uv::UV;
use crate::ogl::vertex::Vertex;
use crate::ogl::viewport::Viewport;

use gl::types::*;
use nalgebra::{Matrix4, Point3, Vector3};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::gl_attr::GLAttr;
use sdl2::video::GLProfile::Core;
use std::ffi::{c_void, CString};
use std::path::Path;
use std::time::Duration;

fn configure_gl(gl_attr: &GLAttr) {
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(8);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
        gl::DepthFunc(gl::LESS);
    }
}

fn main() {
    let mut renderer = Renderer::new();
    renderer.main_loop();
}

fn main2() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("rust-sdl2 demo", 1600, 900)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    gl::load_with(|s| video.gl_get_proc_address(s) as *const c_void);
    let mut canvas = window.into_canvas().build().unwrap();
    let gl_attr = video.gl_attr();
    configure_gl(&gl_attr);

    let mut manager = ResourceManager::new();
    let spot = manager.load_model(Model::Spot).unwrap();

    let mut viewport = Viewport::for_window(1600, 900);
    viewport.use_viewport();

    let mut color_buffer = ColorBuffer::from_color(Vector3::new(1.0, 1.0, 1.0));
    color_buffer.use_color_buffer();
    color_buffer.clear();

    let vert_shader = Shader::vert_from_cstr(
        &CString::new(include_str!("../assets/shaders/shader.vert")).unwrap(),
    )
    .unwrap();

    let frag_shader = Shader::frag_from_cstr(
        &CString::new(include_str!("../assets/shaders/shader.frag")).unwrap(),
    )
    .unwrap();

    let program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let matrix_id =
        unsafe { gl::GetUniformLocation(program.id(), CString::new("MVP").unwrap().as_ptr()) };
    // Projection matrix : 45° Field of View, 4:3 ratio, display range : 0.1 unit <-> 100 units
    let projection = Matrix4::new_perspective(16.0 / 9.0, f32::to_radians(45.0), 0.1, 100.0);
    let view = Matrix4::look_at_rh(
        &Point3::new(-1.0, 0.8, -2.5),
        &Point3::new(0.0, 0.0, 0.0),
        &Vector3::new(0.0, 1.0, 0.0),
    );

    let model = Matrix4::identity();
    let mvp = projection * view * model;

    spot.attrib_pointer();

    program.use_program();

    //texture
    let img = image::open("assets/textures/spot_texture.png")
        .unwrap()
        .to_rgb();
    let width = img.width();
    let height = img.height();
    let data = img.into_vec();

    let texture = Texture::new();
    texture.bind();
    Texture::tex_image_2d(width, height, &data);

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

        color_buffer.set_color(Vector3::new(0.5, i, 0.5));
        color_buffer.use_color_buffer();
        color_buffer.clear();

        unsafe {
            gl::UniformMatrix4fv(matrix_id, 1, gl::FALSE, &mvp[0]);
        }

        spot.draw();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
