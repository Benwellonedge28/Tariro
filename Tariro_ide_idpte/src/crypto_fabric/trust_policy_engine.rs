// Tariro_ide_idpte/src/crypto_fabric/trust_policy_engine.rs

//! Enforces cryptographic policies and manages the overall trust fabric of OMEGA OS.
//! This engine ensures that all cryptographic operations adhere to defined security standards,
//! and ethical considerations (e.g., from EVAS).

use crate::crypto_fabric::qr_algorithms::{QrAlgorithmType, QrKeyExchangeAlg, QrSignatureAlg, QrAlgorithmInfo};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, warn, error};

/// Defines the security and trust level required for a particular context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    /// All communications and data must use NIST-recommended Post-Quantum Cryptography (PQC).
    QuantumResistant,
    /// Standard strong cryptography (e.g., AES-256, RSA-4096) is sufficient.
    StandardStrong,
    /// Only basic integrity checks are required.
    BasicIntegrity,
    /// No specific cryptographic requirements (e.g., for public, non-sensitive data).
    Unsecured,
}

/// A trust policy rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustPolicy {
    pub id: String,
    pub applies_to: String, // e.g., "communication_channel:network_x", "data_storage:database_y", "agi_module:cognitive_core"
    pub required_trust_level: TrustLevel,
    pub enforced_algorithms: Vec<QrAlgorithmType>,
    pub ethical_compliance_mandates: Vec<String>, // Mandates from EVAS (e.g., "No data sharing with X")
    pub active: bool,
}

/// Manages and enforces trust policies across the OMEGA OS ecosystem.
pub struct TrustPolicyEngine {
    active_policies: HashMap<String, TrustPolicy>, // Use HashMap for easier updates/retrieval
}

impl TrustPolicyEngine {
    pub fn new() -> Self {
        println!("Trust Policy Engine initialized.");
        let mut policies = HashMap::new();
        let default_policy = TrustPolicy {
            id: "default-system-comm".to_string(),
            applies_to: "system:communication".to_string(),
            required_trust_level: TrustLevel::QuantumResistant,
            enforced_algorithms: QrAlgorithmInfo::recommended_kems().into_iter().map(|a| a.alg_type).collect(),
            ethical_compliance_mandates: vec![
                "Sovereign AI Caging & ASI Containment (SACC-ASI) Specification".to_string(),
                "Digital Age Sovereignty & Child Protection (DAS-CP) Framework".to_string(),
            ],
            active: true,
        };
        policies.insert(default_policy.id.clone(), default_policy);
        TrustPolicyEngine { active_policies: policies }
    }

    /// Retrieves the relevant trust policy for a given context.
    pub async fn get_policy_for_context(&self, context: &str) -> Option<TrustPolicy> {
        self.active_policies.values().find(|p| p.applies_to == context && p.active).cloned()
    }

    /// Evaluates if a given cryptographic configuration complies with policies for a context.
    /// Returns an error string if non-compliant, Ok(()) otherwise.
    pub async fn evaluate_compliance(
        &self,
        context: &str,
        used_algorithms: &[QrAlgorithmType],
    ) -> Result<(), String> {
        if let Some(policy) = self.get_policy_for_context(context).await {
            info!("Evaluating compliance for context '{}' with policy '{}' (Trust Level: {:?})", context, policy.id, policy.required_trust_level);
            match policy.required_trust_level {
                TrustLevel::QuantumResistant => {
                    let mut kex_compliant = false;
                    let required_kex_algs: Vec<QrKeyExchangeAlg> = policy.enforced_algorithms.iter()
                        .filter_map(|alg_type| if let QrAlgorithmType::KeyExchange(kex) = alg_type { Some(kex.clone()) } else { None })
                        .collect();

                    if required_kex_algs.is_empty() { // If no specific KEX specified, assume Kyber/FrodoKEM are fine
                        if used_algorithms.iter().any(|alg| matches!(alg, QrAlgorithmType::KeyExchange(QrKeyExchangeAlg::Kyber) | QrAlgorithmType::KeyExchange(QrKeyExchangeAlg::FrodoKEM))) {
                            kex_compliant = true;
                        }
                    } else { // Check if any of the required KEX algs are used
                        if used_algorithms.iter().any(|alg| if let QrAlgorithmType::KeyExchange(kex) = alg { required_kex_algs.contains(kex) } else { false }) {
                            kex_compliant = true;
                        }
                    }

                    if !kex_compliant {
                        return Err(format!("Context {} requires Quantum-Resistant Key Exchange (e.g., {:?}) but none of the enforced algorithms were used.", context, required_kex_algs));
                    }

                    // TODO: Add checks for digital signatures, hash functions, etc.

                    // Placeholder for ethical compliance mandates check
                    for mandate in &policy.ethical_compliance_mandates {
                        if mandate.contains("SACC-ASI") && context.contains("inter-agi") {
                            // Simulate call to ALEC-A Ethical Decision Engine for deeper check
                            warn!("TrustPolicyEngine: SACC-ASI mandate check for context '{}' (Mock)", context);
                        }
                    }
                    info!("Policy check for {}: QuantumResistant compliance OK (Mock)", context);
                }
                // Other trust levels would have their own specific checks
                _ => info!("Policy check for {}: TrustLevel {:?} compliance OK (Mock)", context, policy.required_trust_level),
            }
        }
        Ok(())
    }

    /// Updates an existing trust policy or adds a new one.
    /// Handles potential conflicts (simplistic for now).
    pub async fn update_policy(&mut self, policy: TrustPolicy) -> Result<(), String> {
        if self.active_policies.contains_key(&policy.id) {
            warn!("TrustPolicyEngine: Overwriting existing policy with ID {}.", policy.id);
        }
        info!("TrustPolicyEngine: Adding/Updating policy {}: {}", policy.id, policy.applies_to);
        self.active_policies.insert(policy.id.clone(), policy);
        Ok(())}

    /// Removes a policy by ID.
    pub async fn remove_policy(&mut self, policy_id: &str) -> Result<(), String> {
        if self.active_policies.remove(policy_id).is_some() {
            info!("TrustPolicyEngine: Policy {} removed.", policy_id);
            Ok(())
        } else {
            warn!("TrustPolicyEngine: Attempted to remove non-existent policy {}.", policy_id);
            Err(format!("Policy {} not found.", policy_id))
        }
    }

    /// Returns a list of recommended QrAlgorithmType based on the current context and active policies.
    pub async fn get_recommended_algorithms(&self, context: &str) -> Vec<QrAlgorithmType> {
        if let Some(policy) = self.get_policy_for_context(context).await {
            info!("Returning enforced algorithms from policy {} for context {}.", policy.id, context);
            policy.enforced_algorithms.clone()
        } else {
            warn!("No specific policy found for context {}. Returning general NIST recommended algorithms.", context);
            // Default to NIST recommended if no specific policy
            let mut recommended = QrAlgorithmInfo::recommended_kems().into_iter().map(|a| a.alg_type).collect::<Vec<QrAlgorithmType>>();
            recommended.extend(QrAlgorithmInfo::recommended_signatures().into_iter().map(|a| a.alg_type));
            recommended
        }
    }
}
