pub mod models;
pub mod fingerprinting;
pub mod licensing;
pub mod collaboration;
pub mod omniversal;
pub mod blocking;
pub mod sentience;
pub mod pre_cognitive;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct OntologicalIpManager {
    pub fingerprinting_engine: fingerprinting::OntologicalFingerprintingEngine,
    pub causal_licensing_engine: licensing::CausalLicensingEngine,
    pub collaborative_ip_engine: collaboration::CollaborativeIpGenesisEngine,
    pub omniversal_ip_engine: omniversal::OmniversalIpEngine,
    pub causal_blocking_engine: blocking::CausalBlockingEngine,
    pub sentient_ip_engine: sentience::SentientIpEngine,
    pub pre_cognitive_neutralization_engine: pre_cognitive::PreCognitiveNeutralizationEngine,
}

impl OntologicalIpManager {
    pub fn new() -> Self {
        OntologicalIpManager {
            fingerprinting_engine: fingerprinting::OntologicalFingerprintingEngine::default(),
            causal_licensing_engine: licensing::CausalLicensingEngine::default(),
            collaboration_ip_engine: collaboration::CollaborativeIpGenesisEngine::default(),
            omniversal_ip_engine: omniversal::OmniversalIpEngine::default(),
            causal_blocking_engine: blocking::CausalBlockingEngine::default(),
            sentient_ip_engine: sentience::SentientIpEngine::default(),
            pre_cognitive_neutralization_engine: pre_cognitive::PreCognitiveNeutralizationEngine::default(),
        }
    }
}
