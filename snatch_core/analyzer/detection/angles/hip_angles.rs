use crate::models::PoseFrame;
use super::vector::angle_between_vectors;

const RIGHT_SHOULDER: usize = 6;
const RIGHT_HIP: usize = 12;
const RIGHT_KNEE: usize = 14;

/// Returns hip angle in degrees.
/// 180° = fully extended.
/// Smaller angle = more hip flexion.
pub fn hip_angle(frame: &PoseFrame) -> f32 {
    if frame.keypoints.len() <= RIGHT_KNEE {
        return 0.0;
    }

    let shoulder = &frame.keypoints[RIGHT_SHOULDER];
    let hip = &frame.keypoints[RIGHT_HIP];
    let knee = &frame.keypoints[RIGHT_KNEE];

    if shoulder.confidence < 0.2 || hip.confidence < 0.2 || knee.confidence < 0.2 {
        return 0.0;
    }

    // Vectors from hip
    let hip_to_shoulder_x = shoulder.x - hip.x;
    let hip_to_shoulder_y = shoulder.y - hip.y;

    let hip_to_knee_x = knee.x - hip.x;
    let hip_to_knee_y = knee.y - hip.y;

    angle_between_vectors(
        hip_to_shoulder_x,
        hip_to_shoulder_y,
        hip_to_knee_x,
        hip_to_knee_y,
    )
}