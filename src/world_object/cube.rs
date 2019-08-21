use crate::components::layout;
use crate::components::layout::Face;
use crate::components::layout::Layout;
use crate::components::transform::Transform;
use crate::ogl::buffer::ModelBuffer;
use crate::ogl::render::Renderer;
use crate::ogl::render::WorldObject;
use crate::ogl::resources::{Models, ResourceManager, Textures};
use crate::ogl::texture::Texture;

use nalgebra::{Rotation3, UnitQuaternion, Vector3};
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use std::rc::Rc;

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
        let transform = Transform::default();
        Piece { transform, model }
    }
}

pub struct Cube {
    buffer: Rc<Vec<ModelBuffer>>,
    texture: Rc<Texture>,
    pieces: [Piece; 27],
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

    fn on_tick(&mut self, event_pump: &EventPump, _renderer: &Renderer) {
        self.handle_input(event_pump);
        self.tick_turn();
        println!("solved {}", self.layout.solved());
    }
}

impl Cube {
    pub fn new(manager: &mut ResourceManager) -> Cube {
        let spot_mod = manager.load_model(Models::Cube).unwrap();
        //let spot_mod = manager.load_model(Models::Spot).unwrap();
        let spot_tex = manager.load_texture(Textures::Spot).unwrap();

        use crate::components::piece::*;
        let _pieces = [Piece::new(&[]); 27];
        let pieces = [
            /*//white layer
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
            Piece::new(&C),
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
            Piece::new(&GRY),*/

            // blue layer
            Piece::new(&BOY),
            Piece::new(&BY),
            Piece::new(&BRY),
            Piece::new(&BO),
            Piece::new(&B),
            Piece::new(&BR),
            Piece::new(&BOW),
            Piece::new(&BW),
            Piece::new(&BRW),

            // middle layer
            Piece::new(&OY),
            Piece::new(&Y),
            Piece::new(&RY),
            Piece::new(&O),
            Piece::new(&C),
            Piece::new(&R),
            Piece::new(&OW),
            Piece::new(&W),
            Piece::new(&RW),

            // green layer
            Piece::new(&GOY),
            Piece::new(&GY),
            Piece::new(&GRY),
            Piece::new(&GO),
            Piece::new(&G),
            Piece::new(&GR),
            Piece::new(&GOW),
            Piece::new(&GW),
            Piece::new(&GRW),
        ];

        let layout = Layout::new();
        let mut transform = Transform::default();
        transform.rot_offset = UnitQuaternion::from(Rotation3::from_euler_angles(
            f32::to_radians(-90.0),
            0.0,
            0.0,
        ));

        Cube {
            buffer: spot_mod,
            texture: spot_tex,
            pieces,
            turn: None,
            transform,
            layout,
        }
    }

    fn handle_input(&mut self, event_pump: &EventPump) {
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
        if kb.is_scancode_pressed(Scancode::Num1) {
            self.left();
        }
        if kb.is_scancode_pressed(Scancode::Num2) {
            self.right();
        }
        if kb.is_scancode_pressed(Scancode::Num3) {
            self.up();
        }
        if kb.is_scancode_pressed(Scancode::Num4) {
            self.down();
        }
        if kb.is_scancode_pressed(Scancode::Num5) {
            self.front();
        }
        if kb.is_scancode_pressed(Scancode::Num6) {
            self.back();
        }
        if kb.is_scancode_pressed(Scancode::X) {
            self.x();
        }

        if kb.is_scancode_pressed(Scancode::V) {
            for piece in &mut self.pieces[0..=9] {
                piece.transform.translate(Vector3::z() * 0.1);
            }
        }
        if kb.is_scancode_pressed(Scancode::M) {
            self.middle();
        }

        if kb.is_scancode_pressed(Scancode::Z) {
            self.pieces[0].transform.translate(Vector3::y() * 0.01);
        }

    }

    fn tick_turn(&mut self) {
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

    fn turn(&mut self, dir: Vector3<f32>, faces: &[&Face], speed: usize) {
        assert!(speed <= 100);
        if self.turn.is_some() {
            return;
        }

        let mut pieces = Vec::new();
        for face in faces.iter() {
            self.layout.turn(face);
            pieces.extend(&self.layout.layer(face));
        }

        let steps = 101 - speed;
        let dir = dir * f32::to_radians(90.0) / steps as f32;
        let rot = Rotation3::from_euler_angles(dir.x, dir.y, dir.z);
        let turn = Turn { pieces, rot, steps };

        self.turn = Some(turn);
    }

    pub fn front(&mut self) {
        self.turn(-Vector3::y(), &[&layout::FRONT], 80);
    }

    pub fn back(&mut self) {
        self.turn(Vector3::y(), &[&layout::BACK], 80);
    }

    pub fn up(&mut self) {
        self.turn(Vector3::z(), &[&layout::UP], 80);
    }

    pub fn down(&mut self) {
        self.turn(-Vector3::z(), &[&layout::DOWN], 80);
    }

    pub fn left(&mut self) {
        self.turn(Vector3::x(), &[&layout::LEFT], 80);
    }

    pub fn right(&mut self) {
        self.turn(-Vector3::x(), &[&layout::RIGHT], 80);
    }

    pub fn middle(&mut self) {
        self.turn(Vector3::x(), &[&layout::MIDDLE], 80);
    }

    pub fn x(&mut self) {
        self.turn(
            -Vector3::x(),
            &[
                &layout::MIDDLE.reverse(),
                &layout::LEFT.reverse(),
                &layout::RIGHT,
            ],
            80,
        );
    }
}
