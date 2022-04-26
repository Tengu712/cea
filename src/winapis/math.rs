pub struct Matrix4x4([f32; 16]);
impl Matrix4x4 {
    fn from(arr: [f32; 16]) -> Self {
        Self(arr)
    }
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
    pub fn new_view(pos: [f32; 3], rot: [f32; 3]) -> Self {
        let mat_move = Matrix4x4::new_translation(pos[0], pos[1], pos[2]);
        let mat_rot_x = Matrix4x4::new_rotation_x(rot[0]);
        let mat_rot_y = Matrix4x4::new_rotation_y(rot[1]);
        let mat_rot_z = Matrix4x4::new_rotation_z(rot[2]);
        let mat_rot_xy = mul(mat_rot_x, mat_rot_y);
        let mat_rot_xyz = mul(mat_rot_xy, mat_rot_z);
        let mat_rot_move = mul(mat_rot_xyz, mat_move);
        if let Some(n) = inverse(mat_rot_move) {
            n
        } else {
            Matrix4x4::new_identity()
        }
    }
    pub fn new_ortho(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Self {
        Self([
            2.0 / (r - l),           0.0,            0.0, 0.0, 
                      0.0, 2.0 / (t - b),            0.0, 0.0, 
                      0.0,           0.0, -2.0 / (f - n), 0.0, 
                      0.0,           0.0,            0.0, 1.0,
        ])
    }
    pub fn new_perse(w: f32, h: f32, theta: f32, n: f32, f: f32) -> Self {
        Self([
            h / w / theta.tan(),               0.0,               0.0,                    0.0, 
                            0.0, 1.0 / theta.tan(),               0.0,                    0.0, 
                            0.0,               0.0, (f + n) / (f - n), -2.0 * f * n / (f - n), 
                            0.0,               0.0,               1.0,                    1.0,
        ])
    }
}
fn f(i: usize, k: usize) -> usize {
    i + 4 * k
}
fn mul(a: Matrix4x4, b: Matrix4x4) -> Matrix4x4 {
    let mut c = [0.0; 16];
    for i in 0..4 {
        for k in 0..4 {
            c[f(i, k)] = 0.0;
            for j in 0..4 {
                c[f(i, k)] += a.0[f(i, j)] * b.0[f(j, k)];
            }
        }
    }
    Matrix4x4::from(c)
}
fn inverse(a: Matrix4x4) -> Option<Matrix4x4> {
    let m = a.0;
    let mut inv = [0.0; 16];
    inv[0] = m[5]  * m[10] * m[15] - 
             m[5]  * m[11] * m[14] - 
             m[9]  * m[6]  * m[15] + 
             m[9]  * m[7]  * m[14] +
             m[13] * m[6]  * m[11] - 
             m[13] * m[7]  * m[10];
    inv[4] = -m[4]  * m[10] * m[15] + 
              m[4]  * m[11] * m[14] + 
              m[8]  * m[6]  * m[15] - 
              m[8]  * m[7]  * m[14] - 
              m[12] * m[6]  * m[11] + 
              m[12] * m[7]  * m[10];
    inv[8] = m[4]  * m[9] * m[15] - 
             m[4]  * m[11] * m[13] - 
             m[8]  * m[5] * m[15] + 
             m[8]  * m[7] * m[13] + 
             m[12] * m[5] * m[11] - 
             m[12] * m[7] * m[9];
    inv[12] = -m[4]  * m[9] * m[14] + 
               m[4]  * m[10] * m[13] +
               m[8]  * m[5] * m[14] - 
               m[8]  * m[6] * m[13] - 
               m[12] * m[5] * m[10] + 
               m[12] * m[6] * m[9];
    inv[1] = -m[1]  * m[10] * m[15] + 
              m[1]  * m[11] * m[14] + 
              m[9]  * m[2] * m[15] - 
              m[9]  * m[3] * m[14] - 
              m[13] * m[2] * m[11] + 
              m[13] * m[3] * m[10];
    inv[5] = m[0]  * m[10] * m[15] - 
             m[0]  * m[11] * m[14] - 
             m[8]  * m[2] * m[15] + 
             m[8]  * m[3] * m[14] + 
             m[12] * m[2] * m[11] - 
             m[12] * m[3] * m[10];
    inv[9] = -m[0]  * m[9] * m[15] + 
              m[0]  * m[11] * m[13] + 
              m[8]  * m[1] * m[15] - 
              m[8]  * m[3] * m[13] - 
              m[12] * m[1] * m[11] + 
              m[12] * m[3] * m[9];
    inv[13] = m[0]  * m[9] * m[14] - 
              m[0]  * m[10] * m[13] - 
              m[8]  * m[1] * m[14] + 
              m[8]  * m[2] * m[13] + 
              m[12] * m[1] * m[10] - 
              m[12] * m[2] * m[9];
    inv[2] = m[1]  * m[6] * m[15] - 
             m[1]  * m[7] * m[14] - 
             m[5]  * m[2] * m[15] + 
             m[5]  * m[3] * m[14] + 
             m[13] * m[2] * m[7] - 
             m[13] * m[3] * m[6];
    inv[6] = -m[0]  * m[6] * m[15] + 
              m[0]  * m[7] * m[14] + 
              m[4]  * m[2] * m[15] - 
              m[4]  * m[3] * m[14] - 
              m[12] * m[2] * m[7] + 
              m[12] * m[3] * m[6];
    inv[10] = m[0]  * m[5] * m[15] - 
              m[0]  * m[7] * m[13] - 
              m[4]  * m[1] * m[15] + 
              m[4]  * m[3] * m[13] + 
              m[12] * m[1] * m[7] - 
              m[12] * m[3] * m[5];
    inv[14] = -m[0]  * m[5] * m[14] + 
               m[0]  * m[6] * m[13] + 
               m[4]  * m[1] * m[14] - 
               m[4]  * m[2] * m[13] - 
               m[12] * m[1] * m[6] + 
               m[12] * m[2] * m[5];
    inv[3] = -m[1] * m[6] * m[11] + 
              m[1] * m[7] * m[10] + 
              m[5] * m[2] * m[11] - 
              m[5] * m[3] * m[10] - 
              m[9] * m[2] * m[7] + 
              m[9] * m[3] * m[6];
    inv[7] = m[0] * m[6] * m[11] - 
             m[0] * m[7] * m[10] - 
             m[4] * m[2] * m[11] + 
             m[4] * m[3] * m[10] + 
             m[8] * m[2] * m[7] - 
             m[8] * m[3] * m[6];
    inv[11] = -m[0] * m[5] * m[11] + 
               m[0] * m[7] * m[9] + 
               m[4] * m[1] * m[11] - 
               m[4] * m[3] * m[9] - 
               m[8] * m[1] * m[7] + 
               m[8] * m[3] * m[5];
    inv[15] = m[0] * m[5] * m[10] - 
              m[0] * m[6] * m[9] - 
              m[4] * m[1] * m[10] + 
              m[4] * m[2] * m[9] + 
              m[8] * m[1] * m[6] - 
              m[8] * m[2] * m[5];
    let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
    if det == 0.0 {
        None
    } else {
        let det = 1.0 / det;
        for i in 0..16 {
            inv[i] = inv[i] * det;
        }
        Some(Matrix4x4::from(inv))
    }
}
