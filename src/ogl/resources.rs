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
    models: HashMap<Models, Rc<Vec<ModelBuffer>>>,
    textures: HashMap<Textures, Rc<Texture>>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            models: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_model(&mut self, model: Models) -> Result<Rc<Vec<ModelBuffer>>, tobj::LoadError> {
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

fn load_obj<P: AsRef<Path>>(p: P) -> Result<Vec<ModelBuffer>, tobj::LoadError> {
    let (models, materials) = tobj::load_obj(p.as_ref())?;

    let mut buffers = Vec::with_capacity(models.len());

    for model in models {
        println!("{}", model.name);
        let mesh = &model.mesh;

        let vao = VertexArray::new();

        let vertex_buffer = ArrayBuffer::new();
        vertex_buffer.bind();
        ArrayBuffer::buffer_data(&mesh.positions);
        ArrayBuffer::unbind();

        let element_buffer = ElementArrayBuffer::new();
        element_buffer.bind();
        ElementArrayBuffer::buffer_data(&mesh.indices);
        ElementArrayBuffer::unbind();

        let normal_buffer = ArrayBuffer::new();
        normal_buffer.bind();
        ArrayBuffer::buffer_data(&mesh.normals);
        ArrayBuffer::unbind();

        let uv_buffer = ArrayBuffer::new();
        uv_buffer.bind();
        let uvs = mesh
            .texcoords
            .chunks(2)
            .map(|uv| [uv[0], 1.0 - uv[1]])
            .collect::<Vec<_>>();
        ArrayBuffer::buffer_data(&uvs);
        ArrayBuffer::unbind();

        let material = model.mesh.material_id.map(|id| materials[id].clone());

        let model_buffer = ModelBuffer::new(
            vao,
            vertex_buffer,
            element_buffer,
            normal_buffer,
            uv_buffer,
            mesh.indices.len(),
            material,
        );

        buffers.push(model_buffer);
    }

    Ok(buffers)
}
