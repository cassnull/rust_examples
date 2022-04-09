use super::chardistribution;
use super::codingstatemachine;
use super::enums;
use super::mbcssm;

const SHORTCUT_THRESHOLD: f64 = 0.95;

pub struct Big5Prober {
    coding_sm: codingstatemachine::CodingStateMachine,
    distribution_analyzer: chardistribution::Big5DistributionAnalysis,
    last_char: [usize; 2],
    state: enums::ProbingState,
}

impl Big5Prober {
    pub fn reset(&mut self) {
        self.coding_sm.reset();
        self.distribution_analyzer.reset();
        self.last_char = [0, 0];
        self.state = enums::ProbingState::Detecting;
    }

    pub fn get_confidence(&self) -> f64 {
        self.distribution_analyzer.get_confidence()
    }

    pub fn charset_name(&self) -> &str {
        "Big5"
    }

    pub fn language(&self) -> &str {
        "Chinese"
    }
}

impl Default for Big5Prober {
    fn default() -> Self {
        Self {
            coding_sm: codingstatemachine::CodingStateMachine::new(mbcssm::Big5SmModel::default()),
            distribution_analyzer: chardistribution::Big5DistributionAnalysis::default(),
            last_char: [0, 0],
            state: enums::ProbingState::Detecting,
        }
    }
}
