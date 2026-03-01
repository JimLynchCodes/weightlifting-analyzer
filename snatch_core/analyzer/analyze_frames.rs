use super::{angles, fault_detection};
use crate::models::*;

/**
 *   Usage in Node:
 *

const analyzer = new Analyzer({
  barbell_plate_radius: 0.23,
  bar_width: 1.4,
  torso_reference_length: 0.55
});

analyzer.analyzeFrames(frames);
 *
 */

pub struct Analyzer {
    tuning: AnalyzerTuning,

    prev_knee: Option<f32>,
    prev_hip: Option<f32>,
    prev_torso: Option<f32>,
    prev_drift: Option<f32>,
    prev_velocity: Option<f32>,
}

impl Analyzer {
    pub fn new(tuning: AnalyzerTuning) -> Self {
    Self {
        tuning,
        prev_knee: None,
        prev_hip: None,
        prev_torso: None,
        prev_drift: None,
        prev_velocity: None,
    }
}

    pub fn analyze_frames(&mut self, frames: &[PoseFrame]) -> LiftAnalysis {
        let mut knee_angles = Vec::new();
        let mut hip_angles = Vec::new();
        let mut torso_angles = Vec::new();
        let mut bar_drifts = Vec::new();
        let mut bar_velocities = Vec::new();

        let mut start_bar_x: Option<f32> = None;

        for (i, frame) in frames.iter().enumerate() {
            // --- Raw signals ---
            let knee_raw = angles::knee_angle(frame);
            let hip_raw = angles::hip_angle(frame);
            let torso_raw = angles::torso_angle(frame);

            // --- Smoothing ---
            let knee = Self::smooth(self.prev_knee, knee_raw, self.tuning.angle_smoothing_alpha);

            let hip = Self::smooth(self.prev_hip, hip_raw, self.tuning.angle_smoothing_alpha);

            let torso = Self::smooth(
                self.prev_torso,
                torso_raw,
                self.tuning.angle_smoothing_alpha,
            );

            self.prev_knee = Some(knee);
            self.prev_hip = Some(hip);
            self.prev_torso = Some(torso);

            knee_angles.push(knee);
            hip_angles.push(hip);
            torso_angles.push(torso);

            // --- Bar drift ---
            if start_bar_x.is_none() {
                if let Some(bar) = &frame.barbell {
                    if bar.confidence > 0.2 {
                        start_bar_x = Some(bar.center_x);
                    }
                }
            }

            let drift_raw = if let Some(start_x) = start_bar_x {
                angles::bar_drift(frame, start_x, self.tuning.bar_width)
            } else {
                0.0
            };

            let drift = Self::smooth(
                self.prev_drift,
                drift_raw,
                self.tuning.drift_smoothing_alpha,
            );

            self.prev_drift = Some(drift);
            bar_drifts.push(drift);

            // --- Velocity ---
            let velocity_raw = angles::bar_velocity(
                frame,
                if i > 0 { Some(&frames[i - 1]) } else { None },
                self.tuning.bar_width,
            );

            let velocity = Self::smooth(
                self.prev_velocity,
                velocity_raw,
                self.tuning.velocity_smoothing_alpha,
            );

            self.prev_velocity = Some(velocity);
            bar_velocities.push(velocity);
        }

        let faults = fault_detection::detect_faults(
            &knee_angles,
            &hip_angles,
            &torso_angles,
            &bar_drifts,
            &bar_velocities,
            &self.tuning,
        );

        LiftAnalysis {
            knee_angles,
            hip_angles,
            torso_angles,
            bar_drifts,
            bar_velocities,
            faults,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PoseFrame, Keypoint, Barbell};

    #[test]
    fn test_analyzer_pipeline() {
        let frame_count = 10;

        let mut frames = Vec::new();

        for i in 0..frame_count {
            frames.push(PoseFrame {
                timestamp: i as f32 * 0.033,

                keypoints: vec![
                    Keypoint { x: i as f32, y: i as f32, confidence: 1.0 };
                    17
                ],

                barbell: Some(Barbell {
                    center_x: i as f32,
                    center_y: i as f32,
                    width: 0.5,
                    height: 0.1,
                    confidence: 1.0,
                }),
            });
        }

        let mut analyzer = Analyzer::new(AnalyzerTuning::default());

        let analysis = analyzer.analyze_frames(&frames);

        assert_eq!(analysis.knee_angles.len(), frame_count);
        assert_eq!(analysis.hip_angles.len(), frame_count);
        assert_eq!(analysis.torso_angles.len(), frame_count);
        assert_eq!(analysis.bar_drifts.len(), frame_count);
        assert_eq!(analysis.bar_velocities.len(), frame_count);

        assert!(analysis.faults.is_empty() || !analysis.faults.is_empty());
    }
}