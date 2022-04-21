use super::*;

pub struct CDataDiff {
    pub scl_xy: Option<[f32; 2]>,
    pub rot_xyz: Option<[f32; 3]>,
    pub trs_xy: Option<[f32; 2]>,
    pub view_xy: Option<[f32; 2]>,
    pub col_rgba: Option<[f32; 4]>,
}
impl CDataDiff {
    pub fn new() -> Self {
        Self {
            scl_xy: None,
            rot_xyz: None,
            trs_xy: None,
            view_xy: None,
            col_rgba: None,
        }
    }
    pub fn set_scl(self, scl_xy: [f32; 2]) -> Self {
        let mut self_mut = self;
        self_mut.scl_xy = Some(scl_xy);
        self_mut
    }
    pub fn set_rot(self, rot_xyz: [f32; 3]) -> Self {
        let mut self_mut = self;
        self_mut.rot_xyz = Some(rot_xyz);
        self_mut
    }
    pub fn set_trs(self, trs_xy: [f32; 2]) -> Self {
        let mut self_mut = self;
        self_mut.trs_xy = Some(trs_xy);
        self_mut
    }
    pub fn set_view(self, view_xy: [f32; 2]) -> Self {
        let mut self_mut = self;
        self_mut.view_xy = Some(view_xy);
        self_mut
    }
    pub fn set_col(self, col_rgba: [f32; 4]) -> Self {
        let mut self_mut = self;
        self_mut.col_rgba = Some(col_rgba);
        self_mut
    }
}
impl PackingRequest for CDataDiff {
    fn pack(self) -> Request {
        Request::SetCData(self)
    }
}