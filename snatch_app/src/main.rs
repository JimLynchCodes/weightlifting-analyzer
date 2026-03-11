
use anyhow::Result;
use clap::Parser;
use snatch_analyzer::core::{Analyzer, video_loader::load_video_as_pose_frames};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "../example_video_files/YTDown.com_YouTube_Full-Snatch-side-view_Media_pIWow9YEjfE_001_1080p.mp4")]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let frames = load_video_as_pose_frames(&args.file)?;
    println!("Loaded {} PoseFrames", frames.len());

    let analysis = Analyzer::analyze_frames(&frames);
    println!("Lift Analysis: {:#?}", analysis);

    Ok(())
}


// ⚡ Next Steps
// Replace the mocked keypoints and barbell with MoveNet + bar detection.

// Optional: add a max_duration argument or frame skipping for faster testing.

// Optional: return timestamps or associate frame indices with video frames for overlay/debugging.

// If you want, I can write a version that actually runs Hough circle bar detection per frame inside this load_video_as_pose_frames function, so it produces a real Vec<PoseFrame> with bar positions from a side-angle video.

// Do you want me to do that next?




