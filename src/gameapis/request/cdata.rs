use super::*;

pub struct CDataDiff {
    pub scl_xy: [f32; 2],
    pub rot_xyz: [f32; 3],
    pub trs_xyz: [f32; 3],
    pub col_rgba: [f32; 4],
}
impl CDataDiff {
    pub fn new() -> Self {
        Self {
            scl_xy: [1.0; 2],
            rot_xyz: [0.0; 3],
            trs_xyz: [0.0; 3],
            col_rgba: [1.0; 4],
        }
    }
    pub fn set_scl(self, scl_xy: [f32; 2]) -> Self {
        let mut self_mut = self;
        self_mut.scl_xy = scl_xy;
        self_mut
    }
    pub fn set_rot(self, rot_xyz: [f32; 3]) -> Self {
        let mut self_mut = self;
        self_mut.rot_xyz = rot_xyz;
        self_mut
    }
    pub fn set_trs(self, trs_xy: [f32; 2]) -> Self {
        let mut self_mut = self;
        self_mut.trs_xyz = [trs_xy[0], trs_xy[1], 0.0];
        self_mut
    }
    pub fn set_trs_xyz(self, trs_xyz: [f32; 3]) -> Self {
        let mut self_mut = self;
        self_mut.trs_xyz = trs_xyz;
        self_mut
    }
    pub fn set_col(self, col_rgba: [f32; 4]) -> Self {
        let mut self_mut = self;
        self_mut.col_rgba = col_rgba;
        self_mut
    }
}
impl PackingRequest for CDataDiff {
    fn pack(self) -> Request {
        Request::SetCData(self)
    }
}

pub struct ViewDesc {
    pub pos: [f32; 3],
    pub rot: [f32; 3],
}
impl PackingRequest for ViewDesc {
    fn pack(self) -> Request {
        Request::SetView(self)
    }
}

pub struct PerseDesc {
    pub w: f32,
    pub h: f32,
    pub f: f32,
    pub n: f32,
    pub theta: f32,
}
impl PackingRequest for PerseDesc {
    fn pack(self) -> Request {
        Request::SetPerse(self)
    }
}

pub struct OrthoDesc {
    pub l: f32,
    pub r: f32,
    pub t: f32,
    pub b: f32,
    pub f: f32,
    pub n: f32,
}
impl PackingRequest for OrthoDesc {
    fn pack(self) -> Request {
        Request::SetOrtho(self)
    }
}
