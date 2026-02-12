pub mod models;
pub mod biomechanics;
pub mod faults;

use models::*;
use biomechanics::*;
use faults::*;

pub fn analyze(frames: &[PoseFrame]) -> LiftAnalysis {
    let mut knee_angles = Vec::new();
    let mut hip_angles = Vec::new();
    let mut torso_angles = Vec::new();

    for frame in frames {
        // Example placeholder logic
        knee_angles.push(0.0);
        hip_angles.push(0.0);
        torso_angles.push(0.0);
    }

    let faults = detect_faults(&knee_angles, &hip_angles);

    LiftAnalysis {
        knee_angles,
        hip_angles,
        torso_angles,
        faults,
    }
}