pub mod models;
pub mod fingerprinting;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct OntologicalIpManager {
    pub fingerprinting_engine: fingerprinting::OntologicalFingerprintingEngine,
}

impl OntologicalIpManager {
    pub fn new() -> Self {
        OntologicalIpManager {
            fingerprinting_engine: fingerprinting::OntologicalFingerprintingEngine::default(),
        }
    }
}
