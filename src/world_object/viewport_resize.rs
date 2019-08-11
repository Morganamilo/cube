use crate::ogl::render::{Renderer, WorldObject};
use crate::ogl::viewport::Viewport;

use sdl2::event::Event;

pub struct ViewportResize {
    viewport: Viewport,
}

impl WorldObject for ViewportResize {
    fn on_add(&mut self, renderer: &Renderer) {
        self.viewport.use_viewport();
    }

    fn on_event(&mut self, evemt: &Event) {}
}

impl ViewportResize {
    fn new(x: i32, y: i32) -> ViewportResize {
        let viewport = Viewport::for_window(x, y);

        ViewportResize { viewport }
    }
}
