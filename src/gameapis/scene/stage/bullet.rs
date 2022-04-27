use super::*;

const BULLET_RECT: [f32; 4] = [
    GAME_LEFT - 80.0,
    GAME_RIGHT + 80.0,
    GAME_TOP + 80.0,
    GAME_BOTTOM - 80.0,
];

#[derive(Clone)]
pub(super) struct BulletKind {
    imgid: ImgID,
    size: f32,
    pub(super) r: f32,
    pub(super) is_fragile: bool,
}
pub(super) const BUL_FLAN: BulletKind = BulletKind {
    imgid: IMGID_BUL_FLAN,
    size: 90.0,
    r: 100.0,
    is_fragile: false,
};
pub(super) const BUL_CIRCLE: BulletKind = BulletKind {
    imgid: IMGID_BUL_CIRCLE,
    size: 30.0,
    r: 10.0,
    is_fragile: false,
};
pub(super) const BUL_CIRCLE_FRAGILE: BulletKind = BulletKind {
    imgid: IMGID_BUL_CIRCLE_FRAGILE,
    size: 30.0,
    r: 10.0,
    is_fragile: true,
};
pub(super) const BUL_MID_CIRCLE: BulletKind = BulletKind {
    imgid: IMGID_BUL_CIRCLE,
    size: 50.0,
    r: 20.0,
    is_fragile: false,
};
pub(super) const BUL_MID_CIRCLE_FRAGILE: BulletKind = BulletKind {
    imgid: IMGID_BUL_CIRCLE_FRAGILE,
    size: 50.0,
    r: 20.0,
    is_fragile: true,
};
pub(super) const BUL_BIG_CIRCLE: BulletKind = BulletKind {
    imgid: IMGID_BUL_BIG_CIRCLE,
    size: 110.0,
    r: 20.0,
    is_fragile: false,
};
pub(super) const BUL_BIG_CIRCLE_FRAGILE: BulletKind = BulletKind {
    imgid: IMGID_BUL_BIG_CIRCLE_FRAGILE,
    size: 110.0,
    r: 20.0,
    is_fragile: true,
};

#[derive(Clone)]
pub(super) struct Bullet {
    pub(super) knd: BulletKind,
    pub(super) vel: f32,
    pub(super) deg: f32,
    pub(super) pos: [f32; 2],
    pub(super) col: [f32; 4],
    pub(super) dmg: i32,
    pub(super) is_grazed: bool,
}
impl Bullet {
    pub(super) fn new(knd: BulletKind) -> Self {
        Self {
            knd,
            vel: 0.0,
            deg: 0.0,
            pos: [0.0; 2],
            col: [1.0; 4],
            dmg: 0,
            is_grazed: false,
        }
    }
    pub(super) fn set_pos(self, pos: [f32; 2]) -> Self {
        let mut self_mut = self;
        self_mut.pos = pos;
        self_mut
    }
    pub(super) fn set_vel(self, vel: f32) -> Self {
        let mut self_mut = self;
        self_mut.vel = vel;
        self_mut
    }
    pub(super) fn set_deg(self, deg: f32) -> Self {
        let mut self_mut = self;
        self_mut.deg = deg;
        self_mut
    }
    pub(super) fn set_dmg(self, dmg: i32) -> Self {
        let mut self_mut = self;
        self_mut.dmg = dmg;
        self_mut
    }
    pub(super) fn set_col(self, col: [f32; 4]) -> Self {
        let mut self_mut = self;
        self_mut.col = col;
        self_mut
    }
    pub(super) fn update(self) -> Option<Self> {
        let mut self_mut = self;
        self_mut.pos = [
            self_mut.pos[0] + self_mut.vel * self_mut.deg.to_radians().cos(),
            self_mut.pos[1] + self_mut.vel * self_mut.deg.to_radians().sin(),
        ];
        if self_mut.pos[0] < BULLET_RECT[0]
            || self_mut.pos[0] > BULLET_RECT[1]
            || self_mut.pos[1] > BULLET_RECT[2]
            || self_mut.pos[1] < BULLET_RECT[3]
        {
            None
        } else {
            Some(self_mut)
        }
    }
    pub(super) fn create_reqs(&self) -> LinkedList<Request> {
        let mut reqs = LinkedList::new();
        reqs.push_back(self.knd.imgid.clone().pack());
        reqs.push_back(
            CDataDiff::new()
                .set_trs(self.pos)
                .set_scl([self.knd.size, self.knd.size])
                .set_col(self.col)
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs
    }
}
