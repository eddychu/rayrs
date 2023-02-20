use glam::DMat4;

pub struct Transform {
    pub matrix: glam::DMat4,
    pub inverse: glam::DMat4,
}

impl Transform {
    pub fn new(matrix: glam::DMat4) -> Transform {
        Transform { matrix, inverse: matrix.inverse() }
    }

    pub fn lookat(eye: glam::DVec3, center: glam::DVec3, up: glam::DVec3) -> Transform {
        let view = DMat4::look_at_rh(eye, center, up);
        let mat = view.inverse();
        Transform { matrix: mat, inverse: view }
    }

    pub fn vector_to_local(&self, v: glam::DVec3) -> glam::DVec3 {
        self.inverse.transform_vector3(v)
    }

    pub fn point_to_local(&self, p: glam::DVec3) -> glam::DVec3 {
        self.inverse.transform_point3(p)
    }

    pub fn vector_to_world(&self, v: glam::DVec3) -> glam::DVec3 {
        self.matrix.transform_vector3(v)
    }

    pub fn point_to_world(&self, p: glam::DVec3) -> glam::DVec3 {
        self.matrix.transform_point3(p)
    }
}