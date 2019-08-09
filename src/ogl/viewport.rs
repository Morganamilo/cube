use nalgebra::Vector4;

pub struct Viewport(Vector4<i32>);

impl Viewport {
    pub fn for_window(w: i32, h: i32) -> Viewport {
        Viewport(Vector4::new(0, 0, w, h))
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.0.z = w;
        self.0.w = h;
    }

    pub fn use_viewport(&self) {
        unsafe {
            gl::Viewport(self.0.x, self.0.y, self.0.z, self.0.w);
        }
    }
}
