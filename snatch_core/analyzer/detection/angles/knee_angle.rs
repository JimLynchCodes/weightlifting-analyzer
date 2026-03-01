use crate::models::PoseFrame;
use super::vector::angle_between_vectors;

const RIGHT_HIP: usize = 12;
const RIGHT_KNEE: usize = 14;
const RIGHT_ANKLE: usize = 16;

/// Returns knee angle in degrees.
/// 180° = fully extended leg.
/// Smaller angle = more knee flexion.
pub fn knee_angle(frame: &PoseFrame) -> f32 {
    if frame.keypoints.len() <= RIGHT_ANKLE {
        return 0.0;
    }

    let hip = &frame.keypoints[RIGHT_HIP];
    let knee = &frame.keypoints[RIGHT_KNEE];
    let ankle = &frame.keypoints[RIGHT_ANKLE];

    if hip.confidence < 0.2 || knee.confidence < 0.2 || ankle.confidence < 0.2 {
        return 0.0;
    }

    // Vectors from knee
    let knee_to_hip_x = hip.x - knee.x;
    let knee_to_hip_y = hip.y - knee.y;

    let knee_to_ankle_x = ankle.x - knee.x;
    let knee_to_ankle_y = ankle.y - knee.y;

    angle_between_vectors(
        knee_to_hip_x,
        knee_to_hip_y,
        knee_to_ankle_x,
        knee_to_ankle_y,
    )
}