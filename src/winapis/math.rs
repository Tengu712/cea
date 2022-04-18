pub struct Matrix4x4([f32; 16]);
impl Matrix4x4 {
    pub fn new_identity() -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }
    pub fn new_scaling(x: f32, y: f32, z: f32) -> Self {
        Self([
              x, 0.0, 0.0, 0.0, 
            0.0,   y, 0.0, 0.0, 
            0.0, 0.0,   z, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }
    pub fn new_rotation_x(rad: f32) -> Self {
        Self([
            1.0,       0.0,        0.0, 0.0,
            0.0, rad.cos(), -rad.sin(), 0.0,
            0.0, rad.sin(),  rad.cos(), 0.0,
            0.0,       0.0,        0.0, 1.0,
        ])
    }
    pub fn new_rotation_y(rad: f32) -> Self {
        Self([
             rad.cos(), 0.0, rad.sin(), 0.0, 
                   0.0, 1.0,       0.0, 0.0, 
            -rad.sin(), 0.0, rad.cos(), 0.0, 
                   0.0, 0.0,       0.0, 1.0,
        ])
    }
    pub fn new_rotation_z(rad: f32) -> Self {
        Self([
            rad.cos(), -rad.sin(), 0.0, 0.0, 
            rad.sin(),  rad.cos(), 0.0, 0.0, 
                  0.0,        0.0, 1.0, 0.0, 
                  0.0,        0.0, 0.0, 1.0,
        ])
    }
    pub fn new_translation(x: f32, y:f32, z:f32) -> Self {
        Self([
            1.0, 0.0, 0.0,   x, 
            0.0, 1.0, 0.0,   y, 
            0.0, 0.0, 1.0,   z, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }
    pub fn new_ortho(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Self {
        Self([
            2.0 / (r - l),           0.0,            0.0, 0.0, 
                      0.0, 2.0 / (t - b),            0.0, 0.0, 
                      0.0,           0.0, -2.0 / (f - n), 0.0, 
                      0.0,           0.0,            0.0, 1.0,
        ])
    }
}
