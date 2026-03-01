use super::vector::angle_between_vectors;

fn compute_torso_angle(shoulder: &Keypoint, hip: &Keypoint) -> f32 {
    let torso_x = shoulder.x - hip.x;
    let torso_y = shoulder.y - hip.y;

    // Vertical up vector
    let vertical_x = 0.0;
    let vertical_y = -1.0;

    angle_between_vectors(torso_x, torso_y, vertical_x, vertical_y)
}