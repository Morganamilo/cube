use crate::components::transform::Transform;

use nalgebra::Matrix4;

const DEFAULT_FOV: f32 = 70.0;
const DEFAULT_ASPECT_RATIO: f32 = 16.0 / 9.0;

pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub asxspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            transform: Transform::default(),
            fov: DEFAULT_FOV,
            asxspect_ratio: DEFAULT_ASPECT_RATIO,
            near: 0.1,
            far: 100.0,
        }
    }
}

impl Camera {
    pub fn projection(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.asxspect_ratio, self.fov, self.near, self.far)
    }
}
