pub mod deep_threat_intelligence;
pub mod stealth_learning;

#[derive(Default)]
pub struct ThreatIntelligenceAscensionEngine {
    pub deep_threat_intelligence: deep_threat_intelligence::DeepThreatIntelligence,
    pub stealth_learning: stealth_learning::StealthLearning, // Initialize
}

impl ThreatIntelligenceAscensionEngine {
    pub fn new() -> Self {
        ThreatIntelligenceAscensionEngine {
            deep_threat_intelligence: deep_threat_intelligence::DeepThreatIntelligence::new(),
            stealth_learning: stealth_learning::StealthLearning::new(),
        }
    }
}
