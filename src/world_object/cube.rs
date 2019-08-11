use crate::components::transform::Transform;
use crate::ogl::buffer::ModelBuffer;
use crate::ogl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::ogl::color_buffer::ColorBuffer;
use crate::ogl::program::Program;
use crate::ogl::render::Renderer;
use crate::ogl::render::WorldObject;
use crate::ogl::resources::{Models, ResourceManager, Textures};
use crate::ogl::shader::Shader;
use crate::ogl::texture::Texture;
use crate::ogl::uv::UV;
use crate::ogl::vertex::Vertex;
use crate::ogl::viewport::Viewport;

use gl::types::*;
use nalgebra::{Matrix4, Point3, Rotation3, UnitQuaternion, Vector3};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::video::gl_attr::GLAttr;
use sdl2::video::GLProfile::Core;
use sdl2::EventPump;
use std::ffi::{c_void, CString};
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

//corners
const BRW: [usize; 4] = [26, 16, 47, 73];
const BOW: [usize; 4] = [28, 31, 18, 1];
const BOY: [usize; 4] = [21, 34, 5, 67];
const BRY: [usize; 4] = [23, 54, 3, 72];
const GRY: [usize; 4] = [39, 52, 10, 71];
const GRW: [usize; 4] = [46, 49, 13, 70];
const GOW: [usize; 4] = [44, 29, 11, 69];
const GOY: [usize; 4] = [41, 36, 8, 68];

//edges
const BY: [usize; 3] = [22, 4, 56];
const BR: [usize; 3] = [25, 55, 60];
const BW: [usize; 3] = [27, 17, 57];
const BO: [usize; 3] = [20, 32, 0];
const GO: [usize; 3] = [43, 37, 59];
const GY: [usize; 3] = [40, 9, 75];
const GR: [usize; 3] = [38, 50, 74];
const GW: [usize; 3] = [45, 12, 58];
const OW: [usize; 3] = [19, 30, 64];
const OY: [usize; 3] = [35, 7, 65];
const RY: [usize; 3] = [53, 2, 76];
const RW: [usize; 3] = [48, 14, 63];

//centers
const W: [usize; 2] = [15, 79];
const O: [usize; 2] = [33, 77];
const Y: [usize; 2] = [6, 66];
const R: [usize; 2] = [51, 61];
const B: [usize; 2] = [24, 78];
const G: [usize; 2] = [42, 62];

#[derive(Clone)]
struct Piece {
    transform: Transform,
    model: &'static [usize],
}

impl Piece {
    fn new(model: &'static [usize]) -> Piece {
        Piece {
            transform: Transform::default(),
            model: model,
        }
    }
}

pub struct Cube {
    buffer: Rc<Vec<ModelBuffer>>,
    texture: Rc<Texture>,
    transform: Transform,
    pieces: [Piece; 26],
}

impl WorldObject for Cube {
    fn on_render(&mut self, renderer: &Renderer) {
        self.texture.bind();

        for peiece in &self.pieces {
            renderer.set_model(peiece.transform.model());
            println!("{:?}", peiece.transform);
            for &model in peiece.model {
                self.buffer[model].draw(renderer);
            }
        }

        Texture::unbind();
    }

    fn on_tick(&mut self, event_pump: &EventPump, renderer: &Renderer) {
        let kb = &event_pump.keyboard_state();
        if kb.is_scancode_pressed(Scancode::W) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_translate(Vector3::z() * 0.1);
            }
        }
        if kb.is_scancode_pressed(Scancode::A) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_translate(-Vector3::x() * 0.1);
            }
        }
        if kb.is_scancode_pressed(Scancode::D) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_translate(Vector3::x() * 0.1);
            }
        }
        if kb.is_scancode_pressed(Scancode::S) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_translate(-Vector3::z() * 0.1);
            }
        }
        if kb.is_scancode_pressed(Scancode::Left) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    f32::to_radians(-4.0),
                    0.0,
                ))
            }
        }
        if kb.is_scancode_pressed(Scancode::Right) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    f32::to_radians(4.0),
                    0.0,
                ))
            }
        }
        if kb.is_scancode_pressed(Scancode::Q) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    0.0,
                    f32::to_radians(-4.0),
                ))
            }
        }
        if kb.is_scancode_pressed(Scancode::E) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    0.0,
                    f32::to_radians(4.0),
                ))
            }
        }
        if kb.is_scancode_pressed(Scancode::Up) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    f32::to_radians(-4.0),
                    0.0,
                    0.0,
                ))
            }
        }
        if kb.is_scancode_pressed(Scancode::Down) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    f32::to_radians(4.0),
                    0.0,
                    0.0,
                ))
            }
        }
        if kb.is_scancode_pressed(Scancode::Equals) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.scale += Vector3::repeat(0.02);
            }
        }
        if kb.is_scancode_pressed(Scancode::Minus) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.scale -= Vector3::repeat(0.02);
            }
        }
        if kb.is_scancode_pressed(Scancode::Space) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.look_at(Vector3::zeros());
            }
        }
        if kb.is_scancode_pressed(Scancode::U) {
            for transform in self.pieces.iter_mut().map(|p| &mut p.transform) {
                transform.look_at(self.transform.pos.coords - Vector3::y());
            }
        }
        if kb.is_scancode_pressed(Scancode::T) {
            for transform in self.pieces[17..=25].iter_mut().map(|p| &mut p.transform) {
 transform.relative_rotate_euler(Rotation3::from_euler_angles(
                    0.0,
                    0.0,
                    f32::to_radians(-4.0),
                ))
            }
        }

    }
}

impl Cube {
    pub fn new(manager: &mut ResourceManager) -> Cube {
        let spot_mod = manager.load_model(Models::Cube).unwrap();
        //let spot_mod = manager.load_model(Models::Spot).unwrap();
        let spot_tex = manager.load_texture(Textures::Spot).unwrap();
        let mut transform = Transform::default();
        transform.translate(Vector3::z() * 2.0);
        transform.rot_offset = UnitQuaternion::from(Rotation3::from_euler_angles(
            f32::to_radians(180.0),
            f32::to_radians(0.0),
            f32::to_radians(0.0),
        ));

        let pieces = [
            //white layer
            Piece::new(&BOW),
            Piece::new(&BW),
            Piece::new(&BRW),
            Piece::new(&OW),
            Piece::new(&W),
            Piece::new(&RW),
            Piece::new(&GOW),
            Piece::new(&GW),
            Piece::new(&GRW),
            //middle
            Piece::new(&BO),
            Piece::new(&B),
            Piece::new(&BR),
            Piece::new(&O),
            Piece::new(&R),
            Piece::new(&GO),
            Piece::new(&G),
            Piece::new(&GR),
            //yelow layer
            Piece::new(&BOY),
            Piece::new(&BY),
            Piece::new(&BRY),
            Piece::new(&OY),
            Piece::new(&Y),
            Piece::new(&RY),
            Piece::new(&GOY),
            Piece::new(&GY),
            Piece::new(&GRY),
        ];

        Cube {
            buffer: spot_mod,
            texture: spot_tex,
            transform,
            pieces,
        }
    }
}
