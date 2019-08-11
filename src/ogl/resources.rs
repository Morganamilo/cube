use crate::ogl::buffer::ModelBuffer;
use crate::ogl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::ogl::texture::Texture;

use image::ImageError;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Models {
    Cube,
    Spot,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Textures {
    Cube,
    Spot,
}

pub struct ResourceManager {
    models: HashMap<Models, Rc<ModelBuffer>>,
    textures: HashMap<Textures, Rc<Texture>>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            models: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_model(&mut self, model: Models) -> Result<Rc<ModelBuffer>, tobj::LoadError> {
        match self.models.entry(model) {
            Entry::Occupied(e) => Ok(Rc::clone(e.get())),
            Entry::Vacant(e) => {
                let buffers = match model {
                    Models::Spot => load_obj("assets/obj/spot_triangulated.obj")?,
                    Models::Cube => load_obj("assets/obj/rcube.obj")?,
                };
                let buffers = Rc::new(buffers);
                e.insert(Rc::clone(&buffers));
                Ok(buffers)
            }
        }
    }

    pub fn load_texture(&mut self, texture: Textures) -> Result<Rc<Texture>, ImageError> {
        match self.textures.entry(texture) {
            Entry::Occupied(e) => Ok(Rc::clone(e.get())),
            Entry::Vacant(e) => {
                let img = match texture {
                    Textures::Spot => image::open("assets/textures/spot_texture.png")?,
                    Textures::Cube => image::open("assets/textures/cube.bmp")?,
                };
                let img = img.to_rgb();
                let width = img.width();
                let height = img.height();
                let data = img.into_vec();

                let texture = Texture::new();
                texture.bind();
                Texture::tex_image_2d(width, height, &data);
                Texture::unbind();

                let texture = Rc::new(texture);
                e.insert(Rc::clone(&texture));
                Ok(texture)
            }
        }
    }
}

fn load_obj<P: AsRef<Path>>(p: P) -> Result<ModelBuffer, tobj::LoadError> {
    let mut vertices = Vec::<f32>::new();
    let mut indices = Vec::<u32>::new();
    let mut normals = Vec::<f32>::new();
    let mut uvs = Vec::<f32>::new();

    let (models, materials) = tobj::load_obj(p.as_ref())?;

    for model in &models {
        let mesh = &model.mesh;
        let size = vertices.len();
        indices.extend(mesh.indices.iter().map(|&i| i + size as u32));
        //indices.extend(&mesh.indices);
        vertices.extend(&mesh.positions);
        uvs.extend(&mesh.texcoords);
        normals.extend(&mesh.normals);
    }

    for uv in uvs.chunks_mut(2) {
        uv[1] = 1.0 - uv[1];
    }

    let vao = VertexArray::new();

    let vertex_buffer = ArrayBuffer::new();
    vertex_buffer.bind();
    ArrayBuffer::buffer_data(&vertices);
    ArrayBuffer::unbind();

    let element_buffer = ElementArrayBuffer::new();
    element_buffer.bind();
    ElementArrayBuffer::buffer_data(&indices);
    ElementArrayBuffer::unbind();

    let normal_buffer = ArrayBuffer::new();
    normal_buffer.bind();
    ArrayBuffer::buffer_data(&normals);
    ArrayBuffer::unbind();

    let uv_buffer = ArrayBuffer::new();
    uv_buffer.bind();
    ArrayBuffer::buffer_data(&uvs);
    ArrayBuffer::unbind();

    let material = models[0].mesh.material_id.map(|id| materials[id].clone());

    let model_buffer = ModelBuffer::new(
        vao,
        vertex_buffer,
        element_buffer,
        normal_buffer,
        uv_buffer,
        indices.len(),
        material,
    );

    Ok(model_buffer)
}
