use clap::Parser;
use anyhow::Result;
use snatch_core_2::process_video; // Import your function

#[derive(Parser, Debug)]
#[command(author, version, about = "AI Lift Analyzer CLI")]
struct Args {
    /// Path to the input video file
    #[arg(short, long)]
    input: String,

    /// If present, renders a new video with AI overlays suffixing "-ai"
    #[arg(short, long, default_value_t = false)]
    render: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚀 Starting analysis on: {}", args.input);
    
    if args.render {
        println!("🎨 Render mode enabled. Output will include '-ai' suffix.");
    }

    // Call your library function
    let results = process_video(&args.input, args.render)?;

    println!("✅ Analysis complete!");
    println!("Processed {} frames.", results.len());

    // Optional: Print some stats from the first frame
    if let Some(first) = results.first() {
        println!("First frame timestamp: {:.2}s", first.timestamp);
    }

    Ok(())
}