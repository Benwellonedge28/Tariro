use serde::{Serialize, Deserialize};
use crate::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::time::Duration;

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
    MetaIp,
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
pub enum IpSecrecyLevel {
    Public,
    Confidential,
    Secret,
    TopSecret,
    QuantumClassified,
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
    pub ownership_structure: Vec<OwnershipShare>,
    pub secrecy_level: IpSecrecyLevel,
    pub self_aware_identity_id: Option<Uuid>, // New field for Feature 96
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

    /// Feature 96: Self-Aware IP Identity & Agency
    /// Each IP asset develops a unique, self-aware digital identity and can autonomously assert its rights within OMEGA OS.
    pub fn assign_self_aware_identity(&mut self) {
        if self.self_aware_identity_id.is_none() {
            self.self_aware_identity_id = Some(Uuid::new_v4());
            self.metadata.insert("is_self_aware_ip".to_string(), "true".to_string());
            println!("IP Asset '{}' assigned self-aware identity: {:?}", self.title, self.self_aware_identity_id);
        }
    }

    /// Feature 96 (part): Autonomous assertion of rights
    pub fn assert_self_rights(&self, context: &str) -> String {
        if let Some(identity_id) = self.self_aware_identity_id {
            println!("Self-aware IP '{}' (ID: {}) asserting rights in context: '{}'", self.title, identity_id, context);
            // In a real system, this would trigger an enforcement action (e.g., cease-and-desist, legal defense)
            // based on the IP's own interpretation of infringement or violation.
            format!("Self-aware IP {} autonomously asserts its rights. Action triggered: Evaluate infringement in context '{}'.", self.title, context)
        } else {
            format!("IP '{}' does not have a self-aware identity to assert rights.", self.title)
        }
    }
}
