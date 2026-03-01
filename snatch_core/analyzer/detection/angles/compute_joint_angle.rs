use super::vector::angle_between_vectors;

fn compute_joint_angle(a: &Keypoint, b: &Keypoint, c: &Keypoint) -> f32 {
    let ba_x = a.x - b.x;
    let ba_y = a.y - b.y;

    let bc_x = c.x - b.x;
    let bc_y = c.y - b.y;

    angle_between_vectors(ba_x, ba_y, bc_x, bc_y)
}