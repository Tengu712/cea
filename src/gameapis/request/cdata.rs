use super::*;

pub struct CDataDiff {
    pub scl_xy: [f32; 2],
    pub rot_xyz: [f32; 3],
    pub trs_xy: [f32; 2],
    pub view_xy: [f32; 2],
    pub col_rgba: [f32; 4],
}
impl CDataDiff {
    pub fn new() -> Self {
        Self {
            scl_xy: [1.0; 2],
            rot_xyz: [0.0; 3],
            trs_xy: [0.0; 2],
            view_xy: [0.0; 2],
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
        self_mut.trs_xy = trs_xy;
        self_mut
    }
    pub fn set_view(self, view_xy: [f32; 2]) -> Self {
        let mut self_mut = self;
        self_mut.view_xy = view_xy;
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
