use anyhow::{Context, Result};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;
use std::path::Path;
use video_rs::{self, Decoder, Encoder, EncoderSettings, Options, Time};

use crate::models::{Barbell, Keypoint, PoseFrame};

pub fn process_video(video_path: &str, render_output: bool) -> Result<Vec<PoseFrame>> {
    video_rs::init().map_err(|e| anyhow::anyhow!(e))?;
    let path = Path::new(video_path);

    // 1. Setup Decoder
    let mut decoder = Decoder::new(path).context("Failed to open source video")?;
    let (width, height) = decoder.size();
    let frame_rate = decoder.frame_rate();

    // 2. Setup Encoder (Optional)
    let mut encoder = if render_output {
        let output_path = path.with_file_name(format!(
            "{}-ai.{}",
            path.file_stem().unwrap().to_str().unwrap(),
            path.extension().unwrap().to_str().unwrap()
        ));
        
        let settings = EncoderSettings::for_h264(width as usize, height as usize, frame_rate);
        Some(Encoder::new(&output_path, settings).context("Failed to create video encoder")?)
    } else {
        None
    };

    let mut pose_frames = Vec::new();

    for (timestamp, frame) in decoder.decode_iter() {
        let seconds = timestamp.as_secs_f32();

        // --- MOCK DETECTION (Replace with your actual AI logic) ---
        let current_pose = PoseFrame {
            timestamp: seconds,
            keypoints: vec![Keypoint { x: 0.5, y: 0.5, confidence: 1.0 }; 17],
            barbell: Some(Barbell { center_x: 0.5, center_y: 0.5, width: 0.1, height: 0.1, confidence: 1.0 }),
        };
        pose_frames.push(current_pose.clone());

        // 3. Render Overlay
        if let Some(ref mut enc) = encoder {
            // Convert ndarray frame to an image buffer
            let mut img: RgbImage = image::ImageBuffer::from_raw(
                width, height, frame.clone().into_raw_vec()
            ).context("Failed to create image buffer")?;

            // Points to draw: ankle (15,16), knee (13,14), hip (11,12), shoulder (5,6)
            let joints_to_draw = [5, 6, 11, 12, 13, 14, 15, 16];
            
            for &idx in &joints_to_draw {
                if let Some(kp) = current_pose.keypoints.get(idx) {
                    let cx = (kp.x * width as f32) as i32;
                    let cy = (kp.y * height as f32) as i32;
                    draw_filled_circle_mut(&mut img, (cx, cy), 5, Rgb([0, 255, 0]));
                }
            }

            // Encode the modified frame back to the new video
            let frame_ndarray = ndarray::Array3::from_shape_vec(
                (height as usize, width as usize, 3), 
                img.into_raw()
            )?;
            enc.encode(&frame_ndarray, timestamp).context("Encoding error")?;
        }
    }

    if let Some(mut enc) = encoder {
        enc.finish().context("Failed to finalize video")?;
    }

    Ok(pose_frames)
}