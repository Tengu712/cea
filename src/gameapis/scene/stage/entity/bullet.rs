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
}
pub(super) const BUL_FLAN: BulletKind = BulletKind {
    imgid: IMGID_BUL_FLAN,
    size: 90.0,
    r: 100.0,
};

#[derive(Clone)]
pub(in super::super) struct Bullet {
    pub(super) knd: BulletKind,
    pub(super) vel: f32,
    pub(super) deg: f32,
    pub(super) pos: [f32; 2],
    pub(super) dmg: i32,
}
impl Bullet {
    pub(super) fn new(knd: BulletKind) -> Self {
        Self {
            knd,
            vel: 0.0,
            deg: 0.0,
            pos: [0.0; 2],
            dmg: 0,
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
    pub(super) fn update(self) -> Option<Self> {
        let pos = [
            self.pos[0] + self.vel * self.deg.to_radians().cos(),
            self.pos[1] + self.vel * self.deg.to_radians().sin(),
        ];
        if self.pos[0] < BULLET_RECT[0]
            || self.pos[0] > BULLET_RECT[1]
            || self.pos[1] > BULLET_RECT[2]
            || self.pos[1] < BULLET_RECT[3]
        {
            None
        } else {
            Some(Self {
                knd: self.knd,
                vel: self.vel,
                deg: self.deg,
                pos,
                dmg: self.dmg,
            })
        }
    }
    pub(super) fn create_reqs(&self) -> LinkedList<Request> {
        let mut reqs = LinkedList::new();
        reqs.push_back(self.knd.imgid.clone().pack());
        reqs.push_back(
            CDataDiff::new()
                .set_trs(self.pos)
                .set_scl([self.knd.size, self.knd.size])
                .pack(),
        );
        reqs.push_back(Request::DrawImage);
        reqs
    }
}
