use crate::ogl::buffer::{ArrayBuffer, ElementArrayBuffer};
use crate::ogl::buffer::ModelBuffer;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Model {
    Cube,
    Spot,
}

pub struct ResourceManager {
    models: HashMap<Model, Rc<ModelBuffer>>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            models: HashMap::new(),
        }
    }

    pub fn load_obj(&mut self, model: Model) -> Result<Rc<ModelBuffer>, tobj::LoadError> {
        match self.models.entry(model) {
            Entry::Occupied(e) => Ok(Rc::clone(e.get())),
            Entry::Vacant(e) => {
                let buffers = load_obj("foo")?;
                let buffers = Rc::new(buffers);
                e.insert(Rc::clone(&buffers));
                Ok(buffers)
            }
        }
    }
}

fn load_obj<P: AsRef<Path>>(p: P) -> Result<ModelBuffer, tobj::LoadError> {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();

    let (models, materials) = tobj::load_obj(p.as_ref())?;

    for model in models {
        let mesh = model.mesh;
        indices.extend(mesh.indices);
        vertices.extend(mesh.positions);
        uvs.extend(mesh.texcoords);
    }

    for uv in uvs.chunks_mut(2) {
        uv[1] = 1.0 - uv[1];
    }

    let vertex_buffer = ArrayBuffer::new();
    vertex_buffer.bind();
    ArrayBuffer::buffer_data(&vertices);
    ArrayBuffer::unbind();

    let element_buffer = ElementArrayBuffer::new();
    element_buffer.bind();
    ElementArrayBuffer::buffer_data(&indices);
    ElementArrayBuffer::unbind();

    let uv_buffer = ArrayBuffer::new();
    uv_buffer.bind();
    ArrayBuffer::buffer_data(&uvs);
    ArrayBuffer::unbind();

    let model_buffer = ModelBuffer {
       vertices: vertex_buffer,
       indices: element_buffer,
       uvs: uv_buffer,
    };

    Ok(model_buffer)
}

