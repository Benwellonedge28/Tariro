// Tariro AGI - Privacy Preservation Module
// Privacy protection and data governance

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

// ============================================================================
// Privacy Types
// ============================================================================

/// Privacy policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyPolicy {
    pub policy_id: String,
    pub name: String,
    pub rules: Vec<PrivacyRule>,
    pub consent_requirements: Vec<ConsentRequirement>,
    pub data_retention: DataRetentionPolicy,
    pub compliance_frameworks: Vec<ComplianceFramework>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRule {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub rule_type: RuleType,
    pub scope: PrivacyScope,
    pub enforcement: EnforcementLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleType {
    Collection,      // Data collection rules
    Processing,      // Processing rules
    Storage,         // Storage rules
    Sharing,         // Sharing/disclosure rules
    Retention,       // Retention rules
    Deletion,       // Deletion rules
    Access,         // Access control rules
    Consent,        // Consent rules
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyScope {
    pub data_types: Vec<DataType>,
    pub user_types: Vec<String>,
    pub purposes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementLevel {
    Strict,    // Block non-compliance
    Warn,     // Warn but allow
    Monitor,   // Just log
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRequirement {
    pub requirement_id: String,
    pub data_type: DataType,
    pub consent_type: ConsentType,
    pub withdrawal_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentType {
    Explicit,
    Implicit,
    OptIn,
    OptOut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    pub default_retention: Duration,
    pub retention_rules: Vec<RetentionRule>,
    pub auto_delete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionRule {
    pub data_type: DataType,
    pub retention_period: Duration,
    pub archive_before_delete: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceFramework {
    GDPR,
    CCPA,
    HIPAA,
    SOC2,
    ISO27001,
    Custom,
}

// ============================================================================
// Data Classification
// ============================================================================

/// Data classification level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum SensitivityLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    HighlyRestricted,
}

/// Personal data identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalData {
    pub data_id: String,
    pub data_type: DataType,
    pub contains_pii: bool,
    pub pii_types: Vec<PIAType>,
    pub sensitivity: SensitivityLevel,
    pub source: String,
    pub collected_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    UserProfile,
    Financial,
    Health,
    Location,
    Behavioral,
    Communication,
    Biometric,
    Genetic,
    Social,
    Technical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PIAType {
    Name,
    Email,
    Phone,
    Address,
    SSN,
    CreditCard,
    HealthRecord,
    Biometric,
    IPAddress,
    DeviceID,
}

// ============================================================================
// Privacy Engine
// ============================================================================

/// Privacy preservation engine
pub struct PrivacyPreservationEngine {
    pub policies: Vec<PrivacyPolicy>,
    pub current_policy: Option<String>,
    pub consent_registry: ConsentRegistry,
    pub data_inventory: DataInventory,
    pub access_logs: VecDeque<AccessLog>,
    pub config: PrivacyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRegistry {
    pub consents: HashMap<String, ConsentRecord>,
    pub consent_history: VecDeque<ConsentRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub record_id: String,
    pub user_id: String,
    pub data_type: DataType,
    pub consent_type: ConsentType,
    pub granted: bool,
    pub timestamp: Instant,
    pub expires_at: Option<Instant>,
    pub withdrawal_timestamp: Option<Instant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataInventory {
    pub data_items: HashMap<String, PersonalData>,
    pub processed_records: VecDeque<ProcessedDataRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedDataRecord {
    pub record_id: String,
    pub original_data_id: String,
    pub processing_type: ProcessingType,
    pub anonymized: bool,
    pub pseudonymized: bool,
    pub processed_at: Instant,
    pub retention_until: Option<Instant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingType {
    Collection,
    Storage,
    Analysis,
    Sharing,
    Deletion,
    Anonymization,
    Aggregation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLog {
    pub log_id: String,
    pub user_id: Option<String>,
    pub data_id: String,
    pub access_type: AccessType,
    pub granted: bool,
    pub reason: Option<String>,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
    Delete,
    Share,
    Export,
}

#[derive(Debug, Clone)]
pub struct PrivacyConfig {
    pub enable_consent_check: bool,
    pub enable_data_minimization: bool,
    pub enable_anonymization: bool,
    pub enable_pseudonymization: bool,
    pub require_dpia: bool,
    pub auto_delete_expired: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            enable_consent_check: true,
            enable_data_minimization: true,
            enable_anonymization: true,
            enable_pseudonymization: true,
            require_dpia: false,
            auto_delete_expired: true,
        }
    }
}

impl PrivacyPreservationEngine {
    pub fn new() -> Self {
        Self {
            policies: vec![Self::default_policy()],
            current_policy: Some("default".to_string()),
            consent_registry: ConsentRegistry {
                consents: HashMap::new(),
                consent_history: VecDeque::new(),
            },
            data_inventory: DataInventory {
                data_items: HashMap::new(),
                processed_records: VecDeque::new(),
            },
            access_logs: VecDeque::new(),
            config: PrivacyConfig::default(),
        }
    }

    fn default_policy() -> PrivacyPolicy {
        PrivacyPolicy {
            policy_id: "default".to_string(),
            name: "Default Privacy Policy".to_string(),
            rules: vec![
                PrivacyRule {
                    rule_id: "consent_required".to_string(),
                    name: "Consent Required".to_string(),
                    description: "Require consent for personal data collection".to_string(),
                    rule_type: RuleType::Collection,
                    scope: PrivacyScope {
                        data_types: vec![DataType::UserProfile, DataType::Location, DataType::Behavioral],
                        user_types: vec!["all".to_string()],
                        purposes: vec!["all".to_string()],
                    },
                    enforcement: EnforcementLevel::Strict,
                },
                PrivacyRule {
                    rule_id: "data_minimization".to_string(),
                    name: "Data Minimization".to_string(),
                    description: "Collect only necessary data".to_string(),
                    rule_type: RuleType::Collection,
                    scope: PrivacyScope {
                        data_types: vec![DataType::UserProfile, DataType::Behavioral],
                        user_types: vec!["all".to_string()],
                        purposes: vec!["necessary".to_string()],
                    },
                    enforcement: EnforcementLevel::Warn,
                },
                PrivacyRule {
                    rule_id: "no_sharing".to_string(),
                    name: "No Unconsented Sharing".to_string(),
                    description: "Do not share data without consent".to_string(),
                    rule_type: RuleType::Sharing,
                    scope: PrivacyScope {
                        data_types: vec![
                            DataType::Health, DataType::Financial, DataType::Biometric
                        ],
                        user_types: vec!["all".to_string()],
                        purposes: vec!["marketing".to_string(), "third_party".to_string()],
                    },
                    enforcement: EnforcementLevel::Strict,
                },
            ],
            consent_requirements: vec![
                ConsentRequirement {
                    requirement_id: "health_consent".to_string(),
                    data_type: DataType::Health,
                    consent_type: ConsentType::Explicit,
                    withdrawal_allowed: true,
                },
                ConsentRequirement {
                    requirement_id: "location_consent".to_string(),
                    data_type: DataType::Location,
                    consent_type: ConsentType::OptIn,
                    withdrawal_allowed: true,
                },
            ],
            data_retention: DataRetentionPolicy {
                default_retention: Duration::from_secs(365 * 24 * 3600),
                retention_rules: vec![
                    RetentionRule {
                        data_type: DataType::Health,
                        retention_period: Duration::from_secs(7 * 365 * 24 * 3600),
                        archive_before_delete: true,
                    },
                    RetentionRule {
                        data_type: DataType::Financial,
                        retention_period: Duration::from_secs(7 * 365 * 24 * 3600),
                        archive_before_delete: true,
                    },
                ],
                auto_delete: true,
            },
            compliance_frameworks: vec![ComplianceFramework::GDPR, ComplianceFramework::CCPA],
        }
    }

    /// Check if data processing is allowed
    pub fn check_processing(&self, request: &ProcessingRequest) -> ProcessingDecision {
        // Check consent
        if self.config.enable_consent_check {
            if let Some(consent) = self.consent_registry.consents.get(&request.data_id) {
                if !consent.granted {
                    return ProcessingDecision {
                        allowed: false,
                        reason: "Consent not granted".to_string(),
                        requires_action: Some(PrivacyAction::RequestConsent),
                    };
                }
            }
        }

        // Check policy rules
        if let Some(policy_id) = &self.current_policy {
            if let Some(policy) = self.policies.iter().find(|p| &p.policy_id == policy_id) {
                for rule in &policy.rules {
                    if rule.enforcement == EnforcementLevel::Strict {
                        return ProcessingDecision {
                            allowed: false,
                            reason: format!("Strict rule violated: {}", rule.name),
                            requires_action: Some(PrivacyAction::BlockAndReport),
                        };
                    }
                }
            }
        }

        ProcessingDecision {
            allowed: true,
            reason: "All checks passed".to_string(),
            requires_action: None,
        }
    }

    /// Record consent
    pub fn record_consent(&mut self, record: ConsentRecord) {
        let key = format!("{}_{}", record.user_id, format!("{:?}", record.data_type));
        self.consent_registry.consents.insert(key, record.clone());
        self.consent_registry.consent_history.push_back(record);
    }

    /// Check consent status
    pub fn has_consent(&self, user_id: &str, data_type: DataType) -> bool {
        let key = format!("{}_{}", user_id, format!("{:?}", data_type));
        self.consent_registry.consents.get(&key)
            .map(|c| c.granted)
            .unwrap_or(false)
    }

    /// Anonymize data
    pub fn anonymize(&self, data: &str, method: AnonymizationMethod) -> String {
        match method {
            AnonymizationMethod::Redaction => {
                let words: Vec<&str> = data.split_whitespace().collect();
                words.iter().map(|w| {
                    if w.len() > 2 {
                        format!("{}{}", &w[..1], "*".repeat(w.len() - 1))
                    } else {
                        "*".repeat(w.len())
                    }
                }).collect::<Vec<_>>().join(" ")
            }
            AnonymizationMethod::Generalization => {
                "[REDACTED]".to_string()
            }
            AnonymizationMethod::Pseudonymization => {
                format!("USER_{}", generate_id())
            }
            AnonymizationMethod::Aggregation => {
                "[AGGREGATED]".to_string()
            }
        }
    }

    /// Process and protect data
    pub fn process_data(&mut self, request: &ProcessingRequest) -> ProcessingResult {
        let decision = self.check_processing(request);

        if !decision.allowed {
            return ProcessingResult {
                success: false,
                processed_data: None,
                anonymized: false,
                error: Some(decision.reason),
            };
        }

        // Process based on request
        let processed = self.apply_processing(request);

        // Record processing
        let record = ProcessedDataRecord {
            record_id: generate_id(),
            original_data_id: request.data_id.clone(),
            processing_type: request.processing_type,
            anonymized: request.anonymize,
            pseudonymized: request.pseudonymize,
            processed_at: Instant::now(),
            retention_until: Some(Instant::now() + Duration::from_secs(365 * 24 * 3600)),
        };

        self.data_inventory.processed_records.push_back(record);

        ProcessingResult {
            success: true,
            processed_data: Some(processed),
            anonymized: request.anonymize,
            error: None,
        }
    }

    fn apply_processing(&self, request: &ProcessingRequest) -> String {
        let mut result = request.data.clone();

        if request.anonymize {
            result = self.anonymize(&result, AnonymizationMethod::Redaction);
        }

        if request.pseudonymize {
            result = self.anonymize(&result, AnonymizationMethod::Pseudonymization);
        }

        result
    }

    /// Log data access
    pub fn log_access(&mut self, log: AccessLog) {
        self.access_logs.push_back(log);

        while self.access_logs.len() > 10000 {
            self.access_logs.pop_front();
        }
    }

    /// Get privacy audit report
    pub fn generate_audit_report(&self) -> PrivacyAuditReport {
        let now = Instant::now();
        let recent_logs: Vec<_> = self.access_logs.iter().rev().take(100).collect();

        PrivacyAuditReport {
            report_id: generate_id(),
            generated_at: now,
            total_data_items: self.data_inventory.data_items.len(),
            total_access_logs: self.access_logs.len(),
            consent_records: self.consent_registry.consents.len(),
            violations: 0, // Would track actual violations
            last_audit: Some(now),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingRequest {
    pub request_id: String,
    pub data_id: String,
    pub user_id: String,
    pub data: String,
    pub data_type: DataType,
    pub processing_type: ProcessingType,
    pub purpose: String,
    pub anonymize: bool,
    pub pseudonymize: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingDecision {
    pub allowed: bool,
    pub reason: String,
    pub requires_action: Option<PrivacyAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyAction {
    RequestConsent,
    BlockAndReport,
    Anonymize,
    Pseudonymize,
    NotifyUser,
    Escalate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub success: bool,
    pub processed_data: Option<String>,
    pub anonymized: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnonymizationMethod {
    Redaction,
    Generalization,
    Pseudonymization,
    Aggregation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditReport {
    pub report_id: String,
    pub generated_at: Instant,
    pub total_data_items: usize,
    pub total_access_logs: usize,
    pub consent_records: usize,
    pub violations: usize,
    pub last_audit: Option<Instant>,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("priv_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_check() {
        let engine = PrivacyPreservationEngine::new();
        assert!(!engine.has_consent("user1", DataType::Health));
    }

    #[test]
    fn test_processing_check() {
        let engine = PrivacyPreservationEngine::new();

        let request = ProcessingRequest {
            request_id: "req1".to_string(),
            data_id: "data1".to_string(),
            user_id: "user1".to_string(),
            data: "test data".to_string(),
            data_type: DataType::UserProfile,
            processing_type: ProcessingType::Analysis,
            purpose: "analysis".to_string(),
            anonymize: false,
            pseudonymize: false,
        };

        let decision = engine.check_processing(&request);
        assert!(decision.allowed);
    }

    #[test]
    fn test_anonymization() {
        let engine = PrivacyPreservationEngine::new();

        let result = engine.anonymize("John Smith", AnonymizationMethod::Redaction);
        assert!(result.contains("*"));
    }
}
