use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::video::gl_attr::GLAttr;
use sdl2::video::GLProfile::Core;
use sdl2::Sdl;
use std::ffi::{c_void, CString};
use std::path::Path;
use std::time::Duration;
use crate::ogl::viewport::Viewport;

pub struct Renderer {
    canvas: WindowCanvas,
    sdl: Sdl,
    viewport: Viewport,
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
        let mut canvas = window.into_canvas().build().unwrap();
        let gl_attr = video.gl_attr();
        Self::configure_gl(&gl_attr);


        let mut viewport = Viewport::for_window(1600, 900);
        viewport.use_viewport();

        Renderer { sdl, canvas, viewport }
    }

    pub fn main_loop(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut i = 0.0;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Window {
                        win_event: WindowEvent::Resized(w, h),
                        ..
                    } => {
                        self.viewport.set_size(w, h);
                        self.viewport.use_viewport();
                    }
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
