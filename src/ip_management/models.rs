use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
