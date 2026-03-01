pub fn smooth(prev: Option<f32>, current: f32, alpha: f32) -> f32 {
    match prev {
        Some(p) => alpha * current + (1.0 - alpha) * p,
        None => current,
    }
}