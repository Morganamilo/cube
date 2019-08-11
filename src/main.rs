mod components;
mod error;
mod ogl;
mod util;
mod world_object;

use crate::ogl::render::Renderer;
use crate::ogl::resources::ResourceManager;
use crate::world_object::cube::Cube;

fn main() {
    let mut renderer = Renderer::new();
    let mut manager = ResourceManager::new();
    renderer.add_object(Cube::new(&mut manager));
    renderer.main_loop();
}
