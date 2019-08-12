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
use crate::components::layout::Layout;

use gl::types::*;
use nalgebra::{Matrix4, Point3, Rotation3, UnitQuaternion, Vector3};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::video::gl_attr::GLAttr;
use sdl2::video::GLProfile::Core;
use sdl2::EventPump;
use std::ffi::{c_void, CString};
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

struct Turn {
    pieces: Vec<usize>,
    rot: Rotation3<f32>,
    steps: usize,
}

#[derive(Clone, Copy)]
struct Piece {
    transform: Transform,
    model: &'static [usize],
}

impl Piece {
    fn new(model: &'static [usize]) -> Piece {
        let mut transform = Transform::default();

        Piece {
            transform,
            model: model,
        }
    }
}

pub struct Cube {
    buffer: Rc<Vec<ModelBuffer>>,
    texture: Rc<Texture>,
    pieces: [Piece; 26],
    turn: Option<Turn>,
    transform: Transform,
    layout: Layout,
}

impl WorldObject for Cube {
    fn on_render(&mut self, renderer: &Renderer) {
        self.texture.bind();

        for peiece in &self.pieces {
            renderer.set_model(self.transform.model() * peiece.transform.model());
            for &model in peiece.model {
                self.buffer[model].draw(renderer);
            }
        }

        Texture::unbind();
    }

    fn on_tick(&mut self, event_pump: &EventPump, renderer: &Renderer) {
        let kb = &event_pump.keyboard_state();
        if kb.is_scancode_pressed(Scancode::W) {
            self.transform.translate(Vector3::z() * 0.1);
        }
        if kb.is_scancode_pressed(Scancode::A) {
            self.transform.translate(-Vector3::x() * 0.1);
        }
        if kb.is_scancode_pressed(Scancode::D) {
            self.transform.translate(Vector3::x() * 0.1);
        }
        if kb.is_scancode_pressed(Scancode::S) {
            self.transform.translate(-Vector3::z() * 0.1);
        }
        if kb.is_scancode_pressed(Scancode::Left) {
            self.transform.rotate_euler(Rotation3::from_euler_angles(
                0.0,
                f32::to_radians(-4.0),
                0.0,
            ))
        }
        if kb.is_scancode_pressed(Scancode::Right) {
            self.transform.rotate_euler(Rotation3::from_euler_angles(
                0.0,
                f32::to_radians(4.0),
                0.0,
            ))
        }
        if kb.is_scancode_pressed(Scancode::Q) {
            self.transform.rotate_euler(Rotation3::from_euler_angles(
                0.0,
                0.0,
                f32::to_radians(-4.0),
            ))
        }
        if kb.is_scancode_pressed(Scancode::E) {
            self.transform.rotate_euler(Rotation3::from_euler_angles(
                0.0,
                0.0,
                f32::to_radians(4.0),
            ))
        }
        if kb.is_scancode_pressed(Scancode::Up) {
            self.transform.rotate_euler(Rotation3::from_euler_angles(
                f32::to_radians(-4.0),
                0.0,
                0.0,
            ))
        }
        if kb.is_scancode_pressed(Scancode::Down) {
            self.transform.rotate_euler(Rotation3::from_euler_angles(
                f32::to_radians(4.0),
                0.0,
                0.0,
            ))
        }
        if kb.is_scancode_pressed(Scancode::Equals) {
            self.transform.scale += Vector3::repeat(0.02);
        }
        if kb.is_scancode_pressed(Scancode::Minus) {
            self.transform.scale -= Vector3::repeat(0.02);
        }
        if kb.is_scancode_pressed(Scancode::Space) {
            self.transform.look_at(Vector3::zeros());
        }
        if kb.is_scancode_pressed(Scancode::U) {
            self.transform
                .look_at(self.transform.pos.coords - Vector3::y());
        }
        if kb.is_scancode_pressed(Scancode::T) {
            self.front();
        }
        if kb.is_scancode_pressed(Scancode::Y) {
            self.up();
        }

        self.turn();
    }
}

impl Cube {
    pub fn new(manager: &mut ResourceManager) -> Cube {
        let spot_mod = manager.load_model(Models::Cube).unwrap();
        //let spot_mod = manager.load_model(Models::Spot).unwrap();
        let spot_tex = manager.load_texture(Textures::Spot).unwrap();

        use crate::components::piece::*;
        let pieces = [Piece::new(&[]); 26];
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

        let layout = Layout::new();
        let transform = Transform::default();

        Cube {
            buffer: spot_mod,
            texture: spot_tex,
            pieces,
            turn: None,
            transform,
            layout,
        }
    }

    pub fn set_turn(&mut self, t: Turn) {
        self.turn.get_or_insert(t);
    }

    pub fn turn(&mut self) {
        let turn = match &mut self.turn {
            Some(turn) => turn,
            None => return,
        };

        for &piece in &turn.pieces {
            let piece = &mut self.pieces[piece];
            piece.transform.rotate_euler(turn.rot);
        }

        turn.steps -= 1;

        if turn.steps == 0 {
            self.turn = None;
        }
    }

    pub fn front(&mut self) {
        if self.turn.is_some() {
            return;
        }

        let rot = Rotation3::from_euler_angles(0.0, 0.0, f32::to_radians(-90.0 / 20.0));
        let turn = Turn {
            pieces: Vec::from(&self.layout.front()[..]),
            rot,
            steps: 20,
        };

        self.layout.turn_front();
        self.set_turn(turn);
    }

    pub fn up(&mut self) {
        if self.turn.is_some() {
            return;
        }

        let rot = Rotation3::from_euler_angles(0.0, f32::to_radians(-90.0 / 20.0), 0.0);
        let turn = Turn {
            pieces: Vec::from(&self.layout.up()[..]),
            rot,
            steps: 20,
        };

        self.layout.turn_up();
        self.set_turn(turn);
    }
}
