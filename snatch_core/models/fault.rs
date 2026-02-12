
/// Represents a technical fault detected during snatch analysis.
///
/// Faults are identified using rule-based thresholds derived from
/// pose keypoints and joint angle calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum Fault {
    /// The elbows begin flexing before full hip extension is achieved.
    ///
    /// Detection:
    /// - Elbow angle decreases before hip angle reaches near-full extension.
    ///
    /// Why it matters:
    /// - Reduces efficient force transfer from hips to bar.
    /// - Often leads to weaker second pull and bar looping.
    EarlyArmBend,

    /// The hips do not reach full extension during the second pull.
    ///
    /// Detection:
    /// - Maximum hip angle remains below ~170–180 degrees.
    ///
    /// Why it matters:
    /// - Limits upward bar velocity.
    /// - Reduces power output from posterior chain.
    IncompleteHipExtension,

    /// The bar drifts forward away from the athlete's midfoot during the pull.
    ///
    /// Detection:
    /// - Horizontal displacement of wrist midpoint (or bar)
    ///   exceeds a defined threshold relative to starting position.
    ///
    /// Why it matters:
    /// - Increases likelihood of missed lifts.
    /// - Indicates poor bar proximity or improper timing.
    ForwardBarDrift,

    /// The athlete catches the bar in a squat position that is not sufficiently deep.
    ///
    /// Detection:
    /// - Hip angle at catch remains above ~100 degrees.
    ///
    /// Why it matters:
    /// - Reduces ability to stabilize heavy loads.
    /// - May indicate mobility or confidence limitations.
    ShallowCatch,

    /// The torso leans excessively forward during the pull or catch.
    ///
    /// Detection:
    /// - Torso angle relative to vertical exceeds ~20–25 degrees.
    ///
    /// Why it matters:
    /// - Shifts center of mass forward.
    /// - Increases stress on lower back and reduces lift efficiency.
    ExcessiveTorsoLean,

    /// The athlete jumps significantly forward from the starting position.
    ///
    /// Detection:
    /// - Final foot or bar horizontal position deviates beyond threshold
    ///   from initial midfoot alignment.
    ///
    /// Why it matters:
    /// - Indicates imbalance or improper force direction.
    /// - Often associated with forward bar drift.
    JumpForward,
}

impl Fault {
    pub fn coaching_tip(&self) -> &'static str {
        match self {
            Fault::EarlyArmBend =>
                "Keep arms straight until hips fully extend.",
            Fault::IncompleteHipExtension =>
                "Finish tall — fully extend hips and knees.",
            Fault::ForwardBarDrift =>
                "Keep the bar close and pull vertically.",
            Fault::ShallowCatch =>
                "Commit to a deeper catch position.",
            Fault::ExcessiveTorsoLean =>
                "Keep chest tall during pull and catch.",
            Fault::JumpForward =>
                "Drive vertically and land over midfoot.",
        }
    }
}