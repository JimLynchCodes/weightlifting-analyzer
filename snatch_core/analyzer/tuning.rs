use std::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Clone, Copy, Debug)]
pub struct AnalyzerTuning {
    pub barbell_plate_radius: f32,
    pub bar_width: f32,
    pub torso_reference_length: f32,
}

impl Default for AnalyzerTuning {
    fn default() -> Self {
        Self {
            barbell_plate_radius: 0.225,
            bar_width: 1.3,
            torso_reference_length: 0.5,
        }
    }
}

static ANALYZER_TUNING: Lazy<RwLock<AnalyzerTuning>> =
    Lazy::new(|| RwLock::new(AnalyzerTuning::default()));

impl Analyzer {
    pub fn set_tuning(tuning: AnalyzerTuning) {
        *ANALYZER_TUNING.write().unwrap() = tuning;
    }

    pub fn get_tuning() -> AnalyzerTuning {
        *ANALYZER_TUNING.read().unwrap()
    }
}