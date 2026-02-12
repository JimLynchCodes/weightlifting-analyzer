
/// Pose data for a single video frame.
pub struct PoseFrame {
    /// Timestamp of the frame in seconds from the start of the lift.
    /// 
    /// Used for velocity calculations, timing analysis,
    /// and phase detection (e.g., hip extension timing).
    pub timestamp: f32,

    /// Ordered list of detected body keypoints for this frame.
    /// 
    /// For MoveNet, this will contain 17 keypoints in a fixed order
    /// (nose, eyes, shoulders, elbows, wrists, hips, knees, ankles).
    /// Index mapping must remain consistent with the model specification.
    pub keypoints: Vec<Keypoint>,
}