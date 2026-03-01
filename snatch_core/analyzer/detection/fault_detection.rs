use crate::models::Fault;

pub fn detect_faults(
    knee_angles: &[f32],
    hip_angles: &[f32],
    torso_angles: &[f32],
) -> Vec<Fault> {
    let mut faults = Vec::new();

    if hip_angles.iter().cloned().fold(0./0., f32::max) < 170.0 {
        faults.push(Fault::IncompleteHipExtension);
    }

    faults
}
