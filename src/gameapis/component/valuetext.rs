#[derive(Default)]
pub struct ValueText {
    pub format: Option<fn(u64) -> String>,
}
