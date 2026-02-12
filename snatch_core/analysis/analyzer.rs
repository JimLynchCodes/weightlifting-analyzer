use crate::models::*;
use super::{angles, fault_detection};

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(frames: &[PoseFrame]) -> LiftAnalysis {
        let mut knee_angles = Vec::new();
        let mut hip_angles = Vec::new();
        let mut torso_angles = Vec::new();

        for frame in frames {
            knee_angles.push(angles::knee_angle(frame));
            hip_angles.push(angles::hip_angle(frame));
            torso_angles.push(angles::torso_angle(frame));
        }

        let faults = fault_detection::detect_faults(
            &knee_angles,
            &hip_angles,
            &torso_angles,
        );

        LiftAnalysis {
            knee_angles,
            hip_angles,
            torso_angles,
            faults,
        }
    }
}
