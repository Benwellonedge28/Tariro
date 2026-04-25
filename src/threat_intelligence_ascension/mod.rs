pub mod deep_threat_intelligence;

#[derive(Default)]
pub struct ThreatIntelligenceAscensionEngine {
    pub deep_threat_intelligence: deep_threat_intelligence::DeepThreatIntelligence,
}

impl ThreatIntelligenceAscensionEngine {
    pub fn new() -> Self {
        ThreatIntelligenceAscensionEngine {
            deep_threat_intelligence: deep_threat_intelligence::DeepThreatIntelligence::new(),
        }
    }
}
