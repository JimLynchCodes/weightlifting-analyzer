pub fn bar_velocity(
    current: &PoseFrame,
    previous: Option<&PoseFrame>,
    real_bar_width_m: f32,
) -> f32 {
    let prev = match previous {
        Some(p) => p,
        None => return 0.0,
    };

    let curr_bar = match &current.barbell {
        Some(b) if b.confidence > 0.2 && b.width > 0.0 => b,
        _ => return 0.0,
    };

    let prev_bar = match &prev.barbell {
        Some(b) if b.confidence > 0.2 => b,
        _ => return 0.0,
    };

    let dt = current.timestamp - prev.timestamp;
    if dt <= 0.0 {
        return 0.0;
    }

    let meters_per_pixel = real_bar_width_m / curr_bar.width;

    // Y axis grows downward in images → invert
    let delta_y_pixels = prev_bar.center_y - curr_bar.center_y;

    let delta_y_meters = delta_y_pixels * meters_per_pixel;

    delta_y_meters / dt
}