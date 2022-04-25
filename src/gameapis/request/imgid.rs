use super::*;

#[derive(Clone)]
pub struct ImgID(pub &'static str);
impl ImgID {
    pub fn pack(self) -> Request {
        Request::SetImage(self)
    }
}
pub const IMGID_BUL_FLAN: ImgID = ImgID("bu_flan.png");
pub const IMGID_FLAN_B0: ImgID = ImgID("ch_flan_b0.png");
pub const IMGID_FLAN_L0: ImgID = ImgID("ch_flan_l0.png");
pub const IMGID_FLAN_R0: ImgID = ImgID("ch_flan_r0.png");
pub const IMGID_FLAN_ST0: ImgID = ImgID("st_flan_0.png");
pub const IMGID_REMILIA_F0: ImgID = ImgID("ch_remilia_f0.png");
pub const IMGID_FRAME: ImgID = ImgID("ui_frame.png");
pub const IMG_RESOURCE_SIZE: usize = 7;
pub const IMGID_ARRAY: [ImgID; IMG_RESOURCE_SIZE] = [
    IMGID_BUL_FLAN,
    IMGID_FLAN_B0,
    IMGID_FLAN_L0,
    IMGID_FLAN_R0,
    IMGID_FLAN_ST0,
    IMGID_REMILIA_F0,
    IMGID_FRAME,
];
