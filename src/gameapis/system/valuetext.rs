use super::*;

/// Reflect counter value to text.
pub fn system_value_text(emngr: &mut EntityManager) {
    for (k, s, v) in emngr.coms.valuetexts.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(counter) = emngr.coms.counters.get(k) {
            if let Some(n) = emngr.coms.texts.get_mut(k) {
                match v.0 {
                    Some(f) => *n = f(counter, n),
                    None => n.text = counter.count.to_string(),
                }
            }
        }
    }
}
