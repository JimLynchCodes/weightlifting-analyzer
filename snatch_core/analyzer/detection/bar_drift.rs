pub fn bar_drift(
    frame: &PoseFrame,
    start_x: f32,
    real_bar_width_m: f32,
) -> f32 {
    let bar = match &frame.barbell {
        Some(b) if b.confidence > 0.2 && b.width > 0.0 => b,
        _ => return 0.0,
    };

    let meters_per_pixel = real_bar_width_m / bar.width;

    let pixel_drift = bar.center_x - start_x;

    pixel_drift * meters_per_pixel
}