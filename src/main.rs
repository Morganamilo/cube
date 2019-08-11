mod components;
mod error;
mod ogl;
mod util;

use crate::components::transform::Transform;
use crate::ogl::buffer::ModelBuffer;
use crate::ogl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::ogl::color_buffer::ColorBuffer;
use crate::ogl::program::Program;
use crate::ogl::render::RenderObject;
use crate::ogl::render::Renderer;
use crate::ogl::resources::{Models, ResourceManager, Textures};
use crate::ogl::shader::Shader;
use crate::ogl::texture::Texture;
use crate::ogl::uv::UV;
use crate::ogl::vertex::Vertex;
use crate::ogl::viewport::Viewport;

use gl::types::*;
use nalgebra::{Matrix4, Point3, Vector3, Rotation3, UnitQuaternion};
use sdl2::event::{Event, WindowEvent};
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::video::gl_attr::GLAttr;
use sdl2::video::GLProfile::Core;
use sdl2::keyboard::Scancode;
use std::ffi::{c_void, CString};
use std::path::Path;
use std::rc::Rc;
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

struct ExampleObject {
    buffer: Rc<ModelBuffer>,
    texture: Rc<Texture>,
    transform: Transform,
}

impl RenderObject for ExampleObject {
    fn on_render(&mut self, renderer: &Renderer) {
        renderer.set_mvp(self.transform.model());
        self.texture.bind();
        self.buffer.draw();
        Texture::unbind();
    }

    fn on_tick(&mut self, event_pump: &EventPump, renderer: &Renderer) {
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::W) {
                self.transform.relative_translate(Vector3::new(0.0, 0.0, 0.1));
            }
            if event_pump.keyboard_state().is_scancode_pressed(Scancode::A) {
                self.transform.relative_translate(Vector3::new(-0.1, 0.0, 0.0));
            }
            if event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
                self.transform.relative_translate(Vector3::new(0.1, 0.0, 0.0));
            }
            if event_pump.keyboard_state().is_scancode_pressed(Scancode::S) {
                self.transform.relative_translate(Vector3::new(0.0, 0.0, -0.1));
            }
            if event_pump
                .keyboard_state()
                .is_scancode_pressed(Scancode::Left)
            {
                self.transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    f32::to_radians(-4.0),
                    0.0,
                ))
            }
            if event_pump
                .keyboard_state()
                .is_scancode_pressed(Scancode::Right)
            {
                self.transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    f32::to_radians(4.0),
                    0.0,
                ))
            }
            if event_pump.keyboard_state().is_scancode_pressed(Scancode::Q) {
                self.transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    0.0,
                    f32::to_radians(-4.0),
                ))
            }
            if event_pump.keyboard_state().is_scancode_pressed(Scancode::E) {
                self.transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    0.0,
                    f32::to_radians(4.0),
                ))
            }
            if event_pump
                .keyboard_state()
                .is_scancode_pressed(Scancode::Up)
            {
                self.transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    f32::to_radians(-4.0),
                    0.0,
                    0.0,
                ))
            }
            if event_pump
                .keyboard_state()
                .is_scancode_pressed(Scancode::Down)
            {
                self.transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    f32::to_radians(1.0),
                    0.0,
                    0.0,
                ))
            }
            if event_pump
                .keyboard_state()
                .is_scancode_pressed(Scancode::Equals)
            {
                self.transform.scale += Vector3::new(0.1, 0.1, 0.1);
            }
            if event_pump
                .keyboard_state()
                .is_scancode_pressed(Scancode::Minus)
            {
                self.transform.scale -= Vector3::new(0.1, 0.1, 0.1);
            }
            if event_pump.keyboard_state().is_scancode_pressed(Scancode::Space) {
                self.transform.look_at(Vector3::zeros());
            }
            if event_pump.keyboard_state().is_scancode_pressed(Scancode::U) {
                self.transform.look_at(self.transform.pos.coords + self.transform.up());
            }

    }
}

impl ExampleObject {
    fn new(manager: &mut ResourceManager) -> ExampleObject {
        let spot_mod = manager.load_model(Models::Spot).unwrap();
        let spot_tex = manager.load_texture(Textures::Spot).unwrap();
        let mut transform = Transform::default();
        transform.rot_offset = UnitQuaternion::from(Rotation3::from_euler_angles(
            f32::to_radians(180.0),
            f32::to_radians(0.0),
            f32::to_radians(0.0),
        ));

        ExampleObject {
            buffer: spot_mod,
            texture: spot_tex,
            transform,
        }
    }
}

fn main() {
    let mut renderer = Renderer::new();
    let mut manager = ResourceManager::new();
    renderer.add_object(ExampleObject::new(&mut manager));
    renderer.main_loop();
}
