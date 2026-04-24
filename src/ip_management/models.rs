use super::models::{IpAsset, IpSecrecyLevel};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IpType {
    Patent,
    Copyright,
    Trademark,
    TradeSecret,
    DataRight,
    MoralRight,
    IndigenousKnowledge,
    MetaIp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IpDomain {
    Code,
    Content,
    Design,
    Algorithm,
    BiologicalSystem,
    MaterialScience,
    CognitiveArchitecture,
    Protocol,
    SimulatedReality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IpStatus {
    Conception,
    Disclosed,
    PendingRegistration,
    Registered,
    Maintained,
    Licensed,
    PublicDomain,
    Obfuscated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OwnershipShare {
    pub owner_id: String, // e.g., Samuel Mukandara, AGI_X, Human_Y
    pub percentage: f64,  // Share of ownership (0.0 to 100.0)
    pub rights: Vec<String>, // Specific rights (e.g., "monetize", "modify", "distribute")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAsset {
    pub id: Uuid,
    pub title: String,
    pub ip_type: IpType,
    pub domain: IpDomain,
    pub owner_id: String, // Always Samuel Mukandara
    pub status: IpStatus,
    pub valuation_usd: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: std::collections::HashMap<String, String>,
    pub provenance_hash: String, // Quantum-secured hash
    pub ownership_structure: Vec<OwnershipShare>, // New field for Feature 10
    pub secrecy_level: IpSecrecyLevel, // New field for Feature 34
}

impl IpAsset {
    /// Feature 6: IP Lifecycle Management - Part of managing lifecycle
    pub fn update_status(&mut self, new_status: IpStatus) {
        self.status = new_status;
        self.updated_at = Utc::now();
    }

    /// Feature 11: Version Control Integration (IP) - Update provenance hash
    pub fn update_provenance_hash(&mut self, new_hash: String) {
        self.provenance_hash = new_hash;
        self.updated_at = Utc::now();
    }

    /// Feature 34: IP Secrecy Level Management - Update secrecy level
    pub fn update_secrecy_level(&mut self, new_level: IpSecrecyLevel) {
        self.secrecy_level = new_level;
        self.updated_at = Utc::now();
        println!("IP Asset '{}' secrecy level updated to {:?}", self.title, new_level);
    }
}
