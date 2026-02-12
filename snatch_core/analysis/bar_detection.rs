use opencv::{
    core::{self, Point, Vec3f},
    imgproc,
    prelude::*,
    types,
};

/// Detects large circular shapes (likely barbell plates) in a frame.
///
/// Returns a vector of (x, y, radius) for each detected circle.
pub fn detect_plates(frame: &Mat) -> opencv::Result<Vec<(f32, f32, f32)>> {
    // 1️⃣ Convert to grayscale
    let mut gray = Mat::default();
    imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // 2️⃣ Blur to reduce noise
    let mut blurred = Mat::default();
    imgproc::gaussian_blur(
        &gray,
        &mut blurred,
        core::Size::new(9, 9),
        2.0,
        2.0,
        core::BORDER_DEFAULT,
    )?;

    // 3️⃣ Run Hough Circle Transform
    let mut circles = types::VectorOfVec3f::new();

    imgproc::hough_circles(
        &blurred,
        &mut circles,
        imgproc::HOUGH_GRADIENT,
        1.2,             // dp
        100.0,           // min distance between circles
        100.0,           // param1 (Canny high threshold)
        30.0,            // param2 (accumulator threshold)
        40,              // min radius (tune this!)
        200,             // max radius (tune this!)
    )?;

    // 4️⃣ Convert results
    let mut results = Vec::new();

    for circle in circles {
        let x = circle[0];
        let y = circle[1];
        let radius = circle[2];

        results.push((x, y, radius));
    }

    Ok(results)
}
