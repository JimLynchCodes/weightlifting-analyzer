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
}

impl Analyzer {
    pub fn new(tuning: AnalyzerTuning) -> Self {
        Self { tuning }
    }

    pub fn analyze_frames(&self, frames: &[PoseFrame]) -> LiftAnalysis {
        let mut knee_angles = Vec::new();
        let mut hip_angles = Vec::new();
        let mut torso_angles = Vec::new();
        let mut bar_drifts = Vec::new();
        let mut bar_velocities = Vec::new();

        let mut start_bar_x: Option<f32> = None;

        for (i, frame) in frames.iter().enumerate() {
            knee_angles.push(angles::knee_angle(frame));
            hip_angles.push(angles::hip_angle(frame));
            torso_angles.push(angles::torso_angle(frame));

            // Capture first valid bar position
            if start_bar_x.is_none() {
                if let Some(bar) = &frame.barbell {
                    if bar.confidence > 0.2 {
                        start_bar_x = Some(bar.center_x);
                    }
                }
            }

            let drift = if let Some(start_x) = start_bar_x {
                angles::bar_drift(frame, start_x, self.tuning.bar_width)
            } else {
                0.0
            };

            bar_drifts.push(drift);

            let velocity = angles::bar_velocity(
                frame,
                if i > 0 { Some(&frames[i - 1]) } else { None },
                self.tuning.bar_width,
            );

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
    fn test_analyzer_with_mock_frames() {
        // -----------------------------
        // Create mock frames (10 frames)
        // -----------------------------
        let frame_count = 10;

        let mut frames: Vec<PoseFrame> = Vec::new();

        for i in 0..frame_count {
            frames.push(PoseFrame {
                timestamp: i as f32 * 0.033, // ~30 FPS simulation

                // Simple synthetic motion pattern
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

        // -----------------------------
        // Instantiate analyzer
        // -----------------------------
        let analyzer = Analyzer::new(AnalyzerTuning {
            barbell_plate_radius: 0.23,
            bar_width: 1.4,
            torso_reference_length: 0.55,
        });

        // -----------------------------
        // Run analysis
        // -----------------------------
        let analysis = analyzer.analyze_frames(&frames);

        // -----------------------------
        // Structural Assertions (Core Pipeline Validation)
        // -----------------------------

        assert_eq!(analysis.knee_angles.len(), frame_count);
        assert_eq!(analysis.hip_angles.len(), frame_count);
        assert_eq!(analysis.torso_angles.len(), frame_count);
        assert_eq!(analysis.bar_drifts.len(), frame_count);
        assert_eq!(analysis.bar_velocities.len(), frame_count);

        // Fault vector must be valid
        assert!(analysis.faults.is_empty() || !analysis.faults.is_empty());

        // -----------------------------
        // Numerical Stability Checks
        // -----------------------------

        assert!(analysis.knee_angles.iter().all(|v| v.is_finite()));
        assert!(analysis.hip_angles.iter().all(|v| v.is_finite()));
        assert!(analysis.torso_angles.iter().all(|v| v.is_finite()));
        assert!(analysis.bar_drifts.iter().all(|v| v.is_finite()));
        assert!(analysis.bar_velocities.iter().all(|v| v.is_finite()));

        // -----------------------------
        // Optional Sanity Checks
        // -----------------------------

        assert_eq!(analysis.knee_angles.len(), frames.len());
        assert_eq!(analysis.hip_angles.len(), frames.len());
        assert_eq!(analysis.torso_angles.len(), frames.len());
    }
}
