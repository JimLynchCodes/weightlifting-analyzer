/**
 * 
 * 
 * 
 * 
 * 
 Optional but powerful.
 
 Detect:
 
 FirstPull
 
 SecondPull
 
 Catch
 
 Later you’ll need this for timing-based faults.
 */

use crate::models::PoseFrame;
use super::super::analysis::angles::*;
use super::super::analysis::bar::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiftPhase {
    Setup,
    FirstPull,
    Transition,
    SecondPull,
    Turnover,
    Catch,
    Recovery
}


/// Returns one "phase" per frame
// pub fn detect_phases(
//     frames: &[PoseFrame],
//     hip_angles: &[f32],
//     knee_angles: &[f32],
// ) -> Vec<LiftPhase> {
//     []
// }

pub fn detect_phases(
    frames: &[PoseFrame],
    hip_angles: &[f32],
    knee_angles: &[f32],
) -> Vec<LiftPhase> {

    let mut phases = vec![LiftPhase::Setup; frames.len()];

    if frames.is_empty() {
        return phases;
    }

    let extension_frame = match find_full_extension(hip_angles) {
        Some(i) => i,
        None => return phases,
    };

    let transition_frame = match find_transition_frame(knee_angles) {
        Some(i) => i,
        None => return phases,
    };

    // Detect when bar starts descending
    let velocities = vertical_velocity(frames);

    let descent_frame = velocities
        .iter()
        .enumerate()
        .find(|(_, &v)| v < 0.0) // upward positive
        .map(|(i, _)| i)
        .unwrap_or(frames.len() - 1);

    for i in 0..frames.len() {

        if i < transition_frame {
            phases[i] = LiftPhase::FirstPull;
        }
        else if i < extension_frame {
            phases[i] = LiftPhase::SecondPull;
        }
        else if i < descent_frame {
            phases[i] = LiftPhase::Turnover;
        }
        else {
            phases[i] = LiftPhase::Catch;
        }
    }

    phases
}

fn find_full_extension(hip_angles: &[f32]) -> Option<usize> {
    hip_angles
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
}

fn find_transition_frame(knee_angles: &[f32]) -> Option<usize> {
    knee_angles
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
}
