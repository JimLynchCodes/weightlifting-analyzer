pub fn angle_between_vectors(
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
) -> f32 {
    let dot = ax * bx + ay * by;

    let mag_a = (ax.powi(2) + ay.powi(2)).sqrt();
    let mag_b = (bx.powi(2) + by.powi(2)).sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }

    let cos_angle = (dot / (mag_a * mag_b)).clamp(-1.0, 1.0);
    cos_angle.acos().to_degrees()
}