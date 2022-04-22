pub const STAGE1_START_LOG_SIZE: u32 = 2;

pub fn get_stage1_start_log(cnt: u32) -> String {
    if cnt == 0 {
        String::from("はろーわーるど")
    } else {
        String::from("ほげ")
    }
}
