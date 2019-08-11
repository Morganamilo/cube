use nalgebra::{Matrix4, Point3, Rotation3, UnitQuaternion, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub pos: Point3<f32>,
    pub scale: Vector3<f32>,
    pub rot: UnitQuaternion<f32>,
    pub rot_offset: UnitQuaternion<f32>,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::new(
            Point3::origin(),
            Vector3::repeat(1.0),
            UnitQuaternion::identity(),
            UnitQuaternion::identity(),
        )
    }
}

impl Transform {
    pub fn new(
        pos: Point3<f32>,
        scale: Vector3<f32>,
        rot: UnitQuaternion<f32>,
        rot_offset: UnitQuaternion<f32>,
    ) -> Transform {
        Transform {
            pos,
            scale,
            rot,
            rot_offset,
        }
    }

    pub fn up(&self) -> Vector3<f32> {
        (self.rot * -Vector3::y()).normalize()
    }

    pub fn right(&self) -> Vector3<f32> {
        (self.rot * Vector3::x()).normalize()
    }

    pub fn forward(&self) -> Vector3<f32> {
        (self.rot * Vector3::z()).normalize()
    }

    pub fn translate(&mut self, vec: Vector3<f32>) {
        self.pos += vec;
    }

    pub fn relative_translate(&mut self, pos: Vector3<f32>) {
        self.translate(self.up() * pos.y + self.right() * pos.x + self.forward() * pos.z);
    }

    pub fn rotate_euler(&mut self, rot: Rotation3<f32>) {
        self.rot = UnitQuaternion::from(rot) * self.rot;
    }

    pub fn relative_rotate_euler(&mut self, rot: Rotation3<f32>) {
        self.rot *= UnitQuaternion::from(rot);
    }

    pub fn rotation_euler(&self) -> Rotation3<f32> {
        self.rot.into()
    }

    pub fn set_rotation_euler(&mut self, rot: Rotation3<f32>) {
        self.rot = rot.into()
    }

    pub fn look_at(&mut self, target: Vector3<f32>) {
        let dir = (target - self.pos.coords).normalize();
        //let mut up = -self.up().abs();
        let mut up = Vector3::<f32>::y();

        if (dir.abs() - up).abs().sum() < 0.001 {
            //up = self.forward();
            up = Vector3::z();
        }

        self.rot = UnitQuaternion::face_towards(&dir, &up);
    }

    pub fn look_at2(&mut self, p: Vector3<f32>) {
        if let Some(rot) = UnitQuaternion::rotation_between(&self.forward(), &self.pos.coords) {
            self.rot = rot * self.rot;
        }
        if let Some(rot) = UnitQuaternion::rotation_between(&-self.forward(), &self.pos.coords) {
            self.rot = rot * self.rot;
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.pos, &(self.pos + self.forward()), &self.up())
    }

    pub fn model(&self) -> Matrix4<f32> {
        let mut model = Matrix4::identity();

        Matrix4::new_translation(&self.pos.coords)
            * Matrix4::from(self.rot)
            * Matrix4::from(self.rot_offset)
            * model.append_nonuniform_scaling(&self.scale)
    }
}
