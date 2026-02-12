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
                "Commit to a deeper, stable catch position.",
            Fault::ExcessiveTorsoLean =>
                "Maintain upright chest through the pull.",
            Fault::JumpForward =>
                "Drive vertically and land over midfoot.",
        }
    }
}