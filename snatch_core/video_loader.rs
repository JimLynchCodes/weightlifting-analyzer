use anyhow::Result;
use opencv::{
    prelude::*,
    videoio,
    core,
};
use crate::models::{PoseFrame, Keypoint, Barbell};

/// Reads a video file and converts it into a Vec<PoseFrame>.
/// Currently mocks keypoints/barbell — replace with real detection later.
pub fn load_video_as_pose_frames(video_path: &str) -> Result<Vec<PoseFrame>> {
    let mut cap = videoio::VideoCapture::from_file(video_path, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        anyhow::bail!("Failed to open video file: {}", video_path);
    }

    let fps = cap.get(videoio::CAP_PROP_FPS)?;
    println!("Video FPS: {}", fps);

    let mut frames = Vec::new();
    let mut frame_mat = Mat::default();

    loop {
        cap.read(&mut frame_mat)?;
        if frame_mat.empty()? {
            break;
        }

        let timestamp = cap.get(videoio::CAP_PROP_POS_MSEC)? as f32 / 1000.0;

        // TODO: replace with real pose/bar detection
        let pose_frame = PoseFrame {
            timestamp,
            keypoints: vec![Keypoint { x: 0.0, y: 0.0, confidence: 1.0 }; 17],
            barbell: Some(Barbell {
                center_x: 0.0,
                center_y: 0.0,
                width: 1.0,
                height: 0.1,
                confidence: 1.0,
            }),
        };

        frames.push(pose_frame);
    }

    println!("Loaded {} frames from {}", frames.len(), video_path);
    Ok(frames)
}
