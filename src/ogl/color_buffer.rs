use nalgebra::{Vector3, Vector4, U1, U4};

pub struct ColorBuffer {
    pub color: Vector4<f32>,
}

impl ColorBuffer {
    pub fn from_color(color: Vector3<f32>) -> ColorBuffer {
        ColorBuffer {
            color: color.fixed_resize::<U4, U1>(1.0),
        }
    }

    pub fn set_color(&mut self, color: Vector3<f32>) {
        self.color = color.fixed_resize::<U4, U1>(1.0);
    }

    pub fn use_color_buffer(&self) {
        unsafe {
            gl::ClearColor(self.color.x, self.color.y, self.color.z, 1.0);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
