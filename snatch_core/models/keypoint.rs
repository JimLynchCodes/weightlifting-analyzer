/// A single body keypoint detected by the pose model.
pub struct Keypoint {
    /// Horizontal position of the keypoint.
    /// 
    /// Typically normalized between 0.0 and 1.0 relative to image width.
    /// (Multiply by frame width to convert to pixel coordinates.)
    pub x: f32,

    /// Vertical position of the keypoint.
    /// 
    /// Typically normalized between 0.0 and 1.0 relative to image height.
    /// (Multiply by frame height to convert to pixel coordinates.)
    pub y: f32,

    /// Confidence score from the pose model (0.0–1.0).
    /// 
    /// Indicates how certain the model is that this keypoint
    /// was correctly detected. Low-confidence points should
    /// usually be filtered or interpolated.
    pub confidence: f32,
}