pub mod models;
pub mod fingerprinting;
pub mod licensing;
pub mod collaboration;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct OntologicalIpManager {
    pub fingerprinting_engine: fingerprinting::OntologicalFingerprintingEngine,
    pub causal_licensing_engine: licensing::CausalLicensingEngine,
    pub collaborative_ip_engine: collaboration::CollaborativeIpGenesisEngine,
}

impl OntologicalIpManager {
    pub fn new() -> Self {
        OntologicalIpManager {
            fingerprinting_engine: fingerprinting::OntologicalFingerprintingEngine::default(),
            causal_licensing_engine: licensing::CausalLicensingEngine::default(),
            collaborative_ip_engine: collaboration::CollaborativeIpGenesisEngine::default(),
        }
    }
}
