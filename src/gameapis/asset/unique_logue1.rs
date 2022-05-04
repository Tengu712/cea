use super::*;

/// Move count with input.
pub fn unique_logue1(emngr: &mut EntityManager) {
    let id = if let Some(n) = emngr.unique_ids.get(UNIQUE_LOGUE1) {
        *n
    } else {
        return;
    };
    if let Some(counter) = emngr.coms.counters.get_mut(&id) {
        if let Some(text) = emngr.coms.texts.get_mut(&id) {
            if emngr.input.z == 1 {
                counter.count += 1;
            }
            text.text = String::from("はろーわーるど");
        }
    }
}
