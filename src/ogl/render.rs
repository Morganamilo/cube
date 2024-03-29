use crate::ogl::camera::Camera;
use crate::ogl::color_buffer::ColorBuffer;
use crate::ogl::program::Program;
use crate::ogl::shader::Shader;
use crate::ogl::viewport::Viewport;

use nalgebra::{Matrix4, Vector3};
use sdl2::event::{Event, WindowEvent};
use sdl2::render::WindowCanvas;
use sdl2::video::gl_attr::GLAttr;
use sdl2::EventPump;
use sdl2::Sdl;
use std::cell::RefCell;
use std::ffi::{c_void, CString};

pub trait WorldObject {
    fn on_render(&mut self, _renderer: &Renderer) {}
    fn on_tick(&mut self, _event_pump: &EventPump, _renderer: &Renderer) {}
    fn on_add(&mut self, _renderer: &Renderer) {}
    fn on_event(&mut self, _event: &Event) {}
}

pub struct Renderer {
    canvas: WindowCanvas,
    sdl: Sdl,
    viewport: Viewport,
    render_objects: Vec<Box<RefCell<dyn WorldObject>>>,
    camera: Camera,
    pub program: Program,
}

impl Renderer {
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

    fn init_program() -> Program {
        let vert_shader = Shader::vert_from_cstr(
            &CString::new(include_str!("../../assets/shaders/shader.vs")).unwrap(),
        )
        .unwrap();

        let frag_shader = Shader::frag_from_cstr(
            &CString::new(include_str!("../../assets/shaders/shader.fs")).unwrap(),
        )
        .unwrap();

        Program::from_shaders(&[vert_shader, frag_shader]).unwrap()
    }

    pub fn new() -> Renderer {
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
        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let gl_attr = video.gl_attr();
        Self::configure_gl(&gl_attr);

        let viewport = Viewport::for_window(1600, 900);
        viewport.use_viewport();

        let render_objects = Vec::new();
        let mut camera = Camera::default();
        camera.transform.translate(Vector3::new(0.0, 0.0, -14.0));

        let program = Self::init_program();
        program.use_program();

        Renderer {
            sdl,
            canvas,
            viewport,
            render_objects,
            camera,
            program,
        }
    }

    pub fn add_object<O: WorldObject + 'static>(&mut self, o: O) {
        let o = Box::new(RefCell::new(o));
        self.render_objects.push(o);
    }

    fn tick(&mut self, event_pump: &EventPump) {
        for object in &self.render_objects {
            object.borrow_mut().on_tick(event_pump, self);
        }
    }

    fn render(&mut self) {
        for object in &self.render_objects {
            object.borrow_mut().on_render(self);
        }
    }

    fn event(&mut self, event: &Event) {
        for object in &mut self.render_objects {
            object.get_mut().on_event(event);
        }
    }

    pub fn set_model(&self, model: Matrix4<f32>) {
        self.program.set_mat4("model", model);
    }

    pub fn main_loop(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                self.event(&event);
                match event {
                    Event::Window {
                        win_event: WindowEvent::Resized(w, h),
                        ..
                    } => {
                        self.viewport.set_size(w, h);
                        self.viewport.use_viewport();
                    }
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }

                self.event(&event);
            }

            self.tick(&event_pump);

            let color_buffer = ColorBuffer::from_color(Vector3::new(0.5, 0.0, 0.5));
            color_buffer.use_color_buffer();
            color_buffer.clear();

            let i = 0.8;
            let pos = self.camera.transform.pos;
            self.program.set_3f("lightColor", [i, i, i]);
            self.program.set_3f("lightPos", [0.0, -10.0, -10.0]);
            self.program.set_3f("viewPos", [pos.x, pos.y, pos.z]);
            self.program.set_mat4("view", self.camera.transform.view());
            self.program
                .set_mat4("projection", self.camera.projection());

            self.render();
            self.canvas.present();
            //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
