use super::*;

const LOG: [(&'static str, &'static str, &'static str, bool); 13] = [
    (
        "何？　突然弾幕ごっこがしたいなんて\nあいつにでも頼めばいいのに",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        false,
    ),
    (
        "あなた最近身体を動かしていないでしょう？",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        true,
    ),
    (
        "妹をペット扱いするのかしら",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        false,
    ),
    (
        "それなら世話の焼けるペットだわ",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        true,
    ),
    (
        "咲夜の仕事でしょ\n稀にしか顔を見せないのに",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        false,
    ),
    (
        "普段寝ているじゃないの",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        true,
    ),
    (
        "お姉さまも幽閉されてみる？",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        false,
    ),
    (
        "時代は変わるものよ\nだからこうして来たの",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        true,
    ),
    (
        "つまり遊びたいってわけね",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        false,
    ),
    (
        "そういうことにするわ",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        true,
    ),
    (
        "退屈しのぎに遊びましょう？",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        true,
    ),
    (
        "久々に暴れられそうね",
        IMGID_FLAN_ST0,
        IMGID_REMILIA_ST0,
        false,
    ),
    ("", IMGID_FLAN_ST0, IMGID_REMILIA_ST0, false),
];
const COLOR_DISACTIVE: Vector4D = Vector4D {
    x: 0.6,
    y: 0.6,
    z: 0.6,
    w: 1.0,
};

/// Move count with input.
pub fn unique_logue1(emngr: &mut EntityManager) {
    let id = if let Some(n) = emngr.unique_ids.get(UNIQUE_LOGUE1) {
        *n
    } else {
        return;
    };
    let left_id = if let Some(n) = emngr.unique_ids.get(UNIQUE_LOGUE_LEFT) {
        *n
    } else {
        return;
    };
    let right_id = if let Some(n) = emngr.unique_ids.get(UNIQUE_LOGUE_RIGHT) {
        *n
    } else {
        return;
    };
    if let Some(counter) = emngr.coms.counters.get_mut(&id) {
        if let Some(text) = emngr.coms.texts.get_mut(&id) {
            text.text = String::from(LOG[counter.count as usize].0);
        }
        if let Some(left) = emngr.coms.sprites.get_mut(&left_id) {
            left.translation.z = Z_LOGUE_IMAGE;
            if LOG[counter.count as usize].3 {
                left.imgid = Some(LOG[counter.count as usize].1);
                left.translation.y = -80.0;
                left.translation.x = -380.0;
                left.scaling.x = 800.0;
                left.scaling.y = 800.0;
                left.color = COLOR_DISACTIVE;
            } else {
                left.imgid = Some(LOG[counter.count as usize].1);
                left.translation.x = -360.0;
                left.translation.y = -60.0;
                left.scaling.x = 840.0;
                left.scaling.y = 840.0;
                left.color = COLOR_WHITE;
            }
        }
        if let Some(right) = emngr.coms.sprites.get_mut(&right_id) {
            right.translation.z = Z_LOGUE_IMAGE;
            if LOG[counter.count as usize].3 {
                right.imgid = Some(LOG[counter.count as usize].2);
                right.translation.x = 360.0;
                right.translation.y = -60.0;
                right.scaling.x = 840.0;
                right.scaling.y = 840.0;
                right.color = COLOR_WHITE;
            } else {
                right.imgid = Some(LOG[counter.count as usize].2);
                right.translation.x = 380.0;
                right.translation.y = -80.0;
                right.scaling.x = 800.0;
                right.scaling.y = 800.0;
                right.color = COLOR_DISACTIVE;
            }
        }
        if emngr.input.z == 1 {
            counter.count += 1;
        }
    }
}
