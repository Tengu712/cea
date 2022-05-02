use super::*;

/// Reflect counter value to text.
pub fn system_value_text(manager: &mut EntityManager) {
    for (k, s, v) in manager.components.valuetexts.iter() {
        if !s.is_active() {
            continue;
        }
        if let Some(counter) = manager.components.counters.get(k) {
            if let Some(n) = manager.components.texts.get_mut(k) {
                match v.format {
                    Some(f) => n.text = f(counter.count),
                    None => n.text = counter.count.to_string(),
                }
            }
        }
    }
}