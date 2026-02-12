
/// Aggregated biomechanical analysis for a full lift.
pub struct LiftAnalysis {
    /// Knee angle (in degrees) for each processed frame.
    /// 
    /// Computed from hip → knee → ankle keypoints.
    /// Used to detect extension quality, rebend, and catch depth.
    pub knee_angles: Vec<f32>,

    /// Hip angle (in degrees) for each processed frame.
    /// 
    /// Computed from shoulder → hip → knee keypoints.
    /// Used to analyze hip extension timing and explosiveness.
    pub hip_angles: Vec<f32>,

    /// Torso angle (in degrees) for each processed frame.
    /// 
    /// Typically measured relative to vertical using shoulder → hip vector.
    /// Used to detect excessive forward lean or unstable catch position.
    pub torso_angles: Vec<f32>,

    /// List of detected technical faults identified during analysis.
    /// 
    /// Each entry represents a rule-based coaching observation
    /// (e.g., "Early arm bend", "Forward bar drift").
    pub faults: Vec<Fault>
}