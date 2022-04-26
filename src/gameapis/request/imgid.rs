use super::*;

#[derive(Clone)]
pub struct ImgID(pub &'static str);
impl PackingRequest for ImgID {
    fn pack(self) -> Request {
        Request::SetImage(self)
    }
}
// Bullet
pub const IMGID_BUL_FLAN: ImgID = ImgID("bu_flan.png");
pub const IMGID_BUL_CIRCLE: ImgID = ImgID("bu_circle.png");
// Flan
pub const IMGID_FLAN_B0: ImgID = ImgID("ch_flan_b0.png");
pub const IMGID_FLAN_L0: ImgID = ImgID("ch_flan_l0.png");
pub const IMGID_FLAN_R0: ImgID = ImgID("ch_flan_r0.png");
pub const IMGID_FLAN_ST0: ImgID = ImgID("st_flan_0.png");
// Remilia
pub const IMGID_REMILIA_F0: ImgID = ImgID("ch_remilia_f0.png");
// UI
pub const IMGID_SLOWCIRCLE: ImgID = ImgID("ch_slowcircle.png");
pub const IMGID_HITCIRCLE: ImgID = ImgID("ch_hitcircle.png");
pub const IMGID_FRAME: ImgID = ImgID("ui_frame.png");
// Back ground
pub const IMGID_FLOOR: ImgID = ImgID("bg_floor.png");
pub const IMG_RESOURCE_SIZE: usize = 11;
pub const IMGID_ARRAY: [ImgID; IMG_RESOURCE_SIZE] = [
    IMGID_BUL_FLAN,
    IMGID_BUL_CIRCLE,
    IMGID_FLAN_B0,
    IMGID_FLAN_L0,
    IMGID_FLAN_R0,
    IMGID_FLAN_ST0,
    IMGID_REMILIA_F0,
    IMGID_SLOWCIRCLE,
    IMGID_HITCIRCLE,
    IMGID_FRAME,
    IMGID_FLOOR,
];
