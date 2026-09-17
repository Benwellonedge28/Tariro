// Tariro AGI - Compliance Framework
// Multi-framework regulatory compliance engine

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;

// Missing stub types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalSafeguards {
    pub facility_access: bool,
    pub workstation_security: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalSafeguards {
    pub access_control: bool,
    pub audit_controls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HIPAABreachNotification {
    pub notification_required: bool,
    pub notification_sent: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAnalysis {
    pub analysis_date: DateTime<Utc>,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkforceSecurity {
    pub background_checks: bool,
    pub training_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessManagement {
    pub role_based_access: bool,
    pub periodic_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAwarenessProgram {
    pub training_frequency: String,
    pub last_training: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    pub plan_exists: bool,
    pub tested: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContingencyPlan {
    pub backup_frequency: String,
    pub recovery_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvaluation {
    pub evaluation_date: DateTime<Utc>,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BABaintenance {
    pub maintenance_date: DateTime<Utc>,
    pub description: String,
}

// ============================================================================
// Compliance Framework Types
// ============================================================================

/// Comprehensive compliance management engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEngine {
    pub engine_id: String,
    pub frameworks: HashMap<String, ComplianceFramework>,
    pub assessments: Vec<ComplianceAssessment>,
    pub controls: Vec<Control>,
    pub findings: Vec<ComplianceFinding>,
    pub evidence_store: EvidenceStore,
    pub audit_trail: VecDeque<AuditEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFramework {
    pub framework_id: String,
    pub name: String,
    pub version: String,
    pub controls: Vec<FrameworkControl>,
    pub requirements: Vec<Requirement>,
    pub mappings: Vec<ControlMapping>,
    pub audit_schedule: AuditSchedule,
    pub certifications: Vec<Certification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkControl {
    pub control_id: String,
    pub name: String,
    pub description: String,
    pub category: ControlCategory,
    pub subcategory: String,
    pub control_type: ControlType,
    pub implementation_status: ImplementationStatus,
    pub last_reviewed: Option<DateTime<Utc>>,
    pub next_review: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlCategory {
    AccessControl,
    AuditLogging,
    Encryption,
    NetworkSecurity,
    DataProtection,
    IncidentResponse,
    RiskManagement,
    VendorManagement,
    PhysicalSecurity,
    BusinessContinuity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlType {
    Preventive,
    Detective,
    Corrective,
    Administrative,
    Technical,
    Physical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplementationStatus {
    NotImplemented,
    PartiallyImplemented,
    Implemented,
    NotApplicable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub requirement_id: String,
    pub title: String,
    pub description: String,
    pub severity: RequirementSeverity,
    pub applicable: bool,
    pub compensating_controls: Vec<String>,
    pub evidence_required: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequirementSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlMapping {
    pub mapping_id: String,
    pub source_framework: String,
    pub source_control: String,
    pub target_frameworks: Vec<TargetMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetMapping {
    pub framework_id: String,
    pub control_id: String,
    pub relationship: MappingRelationship,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MappingRelationship {
    ExactMatch,
    PartialMatch,
    BroadlyRelated,
    Compensating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSchedule {
    pub frequency: AuditFrequency,
    pub next_audit: Option<DateTime<Utc>>,
    pub auditor: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditFrequency {
    Continuous,
    Monthly,
    Quarterly,
    Annually,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certification {
    pub cert_id: String,
    pub cert_type: CertificationType,
    pub issued_by: String,
    pub issued_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: CertificationStatus,
    pub scope: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationStatus {
    Active,
    Expired,
    Suspended,
    Revoked,
    InProgress,
}

// ============================================================================
// Control Management
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub control_id: String,
    pub framework_id: String,
    pub name: String,
    pub description: String,
    pub control_owner: String,
    pub control_type: ControlType,
    pub risk_level: RiskLevel,
    pub implementation: ControlImplementation,
    pub testing: ControlTesting,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlImplementation {
    pub status: ImplementationStatus,
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub completion_date: Option<DateTime<Utc>>,
    pub automation_level: AutomationLevel,
    pub integration_points: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutomationLevel {
    Manual,
    PartiallyAutomated,
    FullyAutomated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlTesting {
    pub last_test_date: Option<DateTime<Utc>>,
    pub next_test_date: Option<DateTime<Utc>>,
    pub test_results: Vec<TestResult>,
    pub test_methodology: TestMethodology,
    pub exceptions: Vec<ControlException>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub result_id: String,
    pub test_date: DateTime<Utc>,
    pub tester: String,
    pub result: TestOutcome,
    pub findings: Vec<String>,
    pub remediation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestOutcome {
    Pass,
    Fail,
    PassWithExceptions,
    NotTested,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestMethodology {
    Walkthrough,
    Inspection,
    Observation,
    Reperformance,
    Automated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlException {
    pub exception_id: String,
    pub description: String,
    pub risk_accepted_by: String,
    pub approval_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub remediation_plan: String,
}

// ============================================================================
// Evidence Management
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceStore {
    pub evidence_items: Vec<Evidence>,
    pub storage_type: StorageType,
    pub retention_policy: RetentionPolicy,
    pub chain_of_custody: Vec<ChainOfCustodyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_id: String,
    pub control_id: String,
    pub requirement_id: String,
    pub evidence_type: EvidenceType,
    pub title: String,
    pub description: String,
    pub location: String,
    pub collected_at: DateTime<Utc>,
    pub collected_by: String,
    pub integrity_hash: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    Document,
    Screenshot,
    Log,
    Configuration,
    Certificate,
    Report,
    Email,
    Interview,
    SystemOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOfCustodyEntry {
    pub entry_id: String,
    pub evidence_id: String,
    pub action: CustodyAction,
    pub performed_by: String,
    pub performed_at: DateTime<Utc>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustodyAction {
    Created,
    Accessed,
    Modified,
    Transferred,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub min_retention_days: u32,
    pub max_retention_days: u32,
    pub auto_archive: bool,
    pub archive_after_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    Local,
    SecureCloud,
    ColdStorage,
    Distributed,
}

// ============================================================================
// Assessment
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAssessment {
    pub assessment_id: String,
    pub framework_id: String,
    pub assessment_type: AssessmentType,
    pub scope: Vec<String>,
    pub status: AssessmentStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub assessor: String,
    pub findings: Vec<ComplianceFinding>,
    pub overall_score: f32,
    pub risk_rating: RiskRating,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssessmentType {
    Internal,
    External,
    SelfAssessment,
    Continuous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssessmentStatus {
    Planned,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFinding {
    pub finding_id: String,
    pub assessment_id: String,
    pub finding_type: FindingType,
    pub severity: FindingSeverity,
    pub title: String,
    pub description: String,
    pub affected_controls: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub root_cause: String,
    pub impact: String,
    pub likelihood: Likelihood,
    pub risk_score: f32,
    pub recommendations: Vec<String>,
    pub remediation_plan: RemediationPlan,
    pub status: FindingStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Status of a compliance finding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingStatus {
    Open,
    InProgress,
    UnderReview,
    Resolved,
    Closed,
    WontFix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingType {
    NonCompliance,
    ControlGap,
    Vulnerability,
    Misconfiguration,
    ProcessDeviation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Likelihood {
    Rare,
    Unlikely,
    Possible,
    Likely,
    AlmostCertain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskRating {
    Critical,
    High,
    Medium,
    Low,
    Minimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationPlan {
    pub plan_id: String,
    pub actions: Vec<RemediationAction>,
    pub owner: String,
    pub deadline: Option<DateTime<Utc>>,
    pub resources_required: Vec<String>,
    pub estimated_effort: String,
    pub status: RemediationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    pub action_id: String,
    pub description: String,
    pub assignee: String,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: RemediationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationStatus {
    NotStarted,
    InProgress,
    Completed,
    Verified,
    RiskAccepted,
}

// ============================================================================
// GDPR Specific
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDPRCompliance {
    pub data_subject_rights: DataSubjectRights,
    pub lawful_basis: LawfulBasisRegistry,
    pub data_protection_officer: Option<DPOInfo>,
    pub privacy_impact_assessments: Vec<PIA>,
    pub breach_notification: BreachNotificationProcess,
    pub cross_border_transfers: Vec<TransferMechanism>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubjectRights {
    pub right_to_access: bool,
    pub right_to_rectification: bool,
    pub right_to_erasure: bool,
    pub right_to_restrict: bool,
    pub right_to_portability: bool,
    pub right_to_object: bool,
    pub automated_decision_making: AutomatedDecisionMaking,
    pub requests: Vec<SubjectAccessRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectAccessRequest {
    pub request_id: String,
    pub request_type: SARType,
    pub submitted_by: String,
    pub submitted_at: DateTime<Utc>,
    pub status: SARStatus,
    pub response_deadline: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub data_provided: Vec<String>,
    pub exemptions_claimed: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SARType {
    Access,
    Rectification,
    Erasure,
    Restriction,
    Portability,
    Objection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SARStatus {
    Received,
    Processing,
    AwaitingVerification,
    Completed,
    Denied,
    PartiallyCompleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedDecisionMaking {
    pub enabled: bool,
    pub profiling: bool,
    pub human_intervention_available: bool,
    pub safeguards_in_place: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawfulBasisRegistry {
    pub bases: Vec<LawfulBasis>,
    pub processing_activities: Vec<ProcessingActivity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawfulBasis {
    pub basis_id: String,
    pub basis_type: BasisType,
    pub description: String,
    pub documented_at: DateTime<Utc>,
    pub documented_by: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BasisType {
    Consent,
    Contract,
    LegalObligation,
    VitalInterests,
    PublicTask,
    LegitimateInterests,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingActivity {
    pub activity_id: String,
    pub name: String,
    pub purpose: String,
    pub lawful_basis: BasisType,
    pub data_categories: Vec<String>,
    pub data_subjects: Vec<String>,
    pub recipients: Vec<String>,
    pub retention_period: String,
    pub safeguards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DPOInfo {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub qualifications: Vec<String>,
    pub appointed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PIA {
    pub pia_id: String,
    pub project_name: String,
    pub description: String,
    pub necessity: String,
    pub risks: Vec<PIARisk>,
    pub safeguards: Vec<String>,
    pub status: PIAStatus,
    pub consulted_dpo: bool,
    pub approved_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PIARisk {
    pub risk_id: String,
    pub description: String,
    pub likelihood: Likelihood,
    pub severity: FindingSeverity,
    pub mitigation: String,
    pub residual_risk: RiskRating,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PIAStatus {
    Draft,
    InReview,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachNotificationProcess {
    pub process_id: String,
    pub detection_method: String,
    pub assessment_timeline_hours: u32,
    pub notification_timeline_days: u32,
    pub authorities: Vec<String>,
    pub documented_breaches: Vec<BreachRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachRecord {
    pub breach_id: String,
    pub detected_at: DateTime<Utc>,
    pub described_at: Option<DateTime<Utc>>,
    pub notified_at: Option<DateTime<Utc>>,
    pub type_: BreachType,
    pub categories: Vec<String>,
    pub data_affected: Vec<String>,
    pub subjects_affected: u32,
    pub risk_to_rights: RiskLevel,
    pub authority_notified: bool,
    pub supervisory_authority: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreachType {
    Confidentiality,
    Integrity,
    Availability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferMechanism {
    pub mechanism_id: String,
    pub mechanism_type: TransferType,
    pub destination_country: String,
    pub adequacy_status: AdequacyStatus,
    pub safeguards: Vec<String>,
    pub exceptions_used: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferType {
    SCC,
    BCR,
    PrivacyShield,
    Contract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdequacyStatus {
    Adequate,
    Inadequate,
    Pending,
    NotAssessed,
}

// ============================================================================
// HIPAA Specific
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HIPAACompliance {
    pub administrative_safeguards: AdministrativeSafeguards,
    pub physical_safeguards: PhysicalSafeguards,
    pub technical_safeguards: TechnicalSafeguards,
    pub breach_notification: HIPAABreachNotification,
    pub risk_analysis: RiskAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministrativeSafeguards {
    pub security_management_process: SecurityManagementProcess,
    pub workforce_security: WorkforceSecurity,
    pub information_access_management: AccessManagement,
    pub security_awareness: SecurityAwarenessProgram,
    pub security_incident: IncidentResponse,
    pub contingency: ContingencyPlan,
    pub evaluation: SecurityEvaluation,
    pub business_associates: BABaintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityManagementProcess {
    pub risk_analysis: bool,
    pub risk_management: bool,
    pub sanction_policy: bool,
    pub information_system_activity: ISAAudit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ISAAudit {
    pub audit_controls: bool,
    pub audit_logs: Vec<AuditLogEntry>,
    pub log_retention_days: u32,
    pub review_frequency: String,
}

// ============================================================================
// Audit Entry
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub entry_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: ComplianceEventType,
    pub user: String,
    pub description: String,
    pub affected_items: Vec<String>,
}

// ============================================================================
// Risk Assessment Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("cmp_{}", now % 1_000_000)
}

// ============================================================================
// Implementation
// ============================================================================

impl ComplianceEngine {
    pub fn new() -> Self {
        Self {
            engine_id: generate_id(),
            frameworks: HashMap::new(),
            assessments: Vec::new(),
            controls: Vec::new(),
            findings: Vec::new(),
            evidence_store: EvidenceStore {
                evidence_items: Vec::new(),
                storage_type: StorageType::SecureCloud,
                retention_policy: RetentionPolicy {
                    min_retention_days: 2555, // 7 years
                    max_retention_days: 3650, // 10 years
                    auto_archive: true,
                    archive_after_days: 365,
                },
                chain_of_custody: Vec::new(),
            },
            audit_trail: VecDeque::new(),
        }
    }

    /// Initialize SOC 2 framework
    pub fn init_soc2(&mut self) {
        let framework = ComplianceFramework {
            framework_id: "soc2".to_string(),
            name: "SOC 2 Type II".to_string(),
            version: "2017".to_string(),
            controls: vec![
                FrameworkControl {
                    control_id: "CC1.1".to_string(),
                    name: "Control Environment".to_string(),
                    description: "Entity demonstrates commitment to integrity and ethical values".to_string(),
                    category: ControlCategory::AccessControl,
                    subcategory: "Control Environment".to_string(),
                    control_type: ControlType::Administrative,
                    implementation_status: ImplementationStatus::Implemented,
                    last_reviewed: Some(DateTime::<Utc>::now()),
                    next_review: Some(DateTime::<Utc>::now()),
                },
                FrameworkControl {
                    control_id: "CC6.1".to_string(),
                    name: "Logical Access Controls".to_string(),
                    description: "Entity implements logical access security software".to_string(),
                    category: ControlCategory::AccessControl,
                    subcategory: "Logical Access".to_string(),
                    control_type: ControlType::Technical,
                    implementation_status: ImplementationStatus::Implemented,
                    last_reviewed: Some(DateTime::<Utc>::now()),
                    next_review: Some(DateTime::<Utc>::now()),
                },
                FrameworkControl {
                    control_id: "CC7.1".to_string(),
                    name: "System Operations".to_string(),
                    description: "Entity manages day-to-day system operations".to_string(),
                    category: ControlCategory::NetworkSecurity,
                    subcategory: "Operations".to_string(),
                    control_type: ControlType::Technical,
                    implementation_status: ImplementationStatus::Implemented,
                    last_reviewed: Some(DateTime::<Utc>::now()),
                    next_review: Some(DateTime::<Utc>::now()),
                },
            ],
            requirements: vec![
                Requirement {
                    requirement_id: "CC6.1.1".to_string(),
                    title: "Authentication".to_string(),
                    description: "Prior to issuing system credentials, confirms user identity".to_string(),
                    severity: RequirementSeverity::High,
                    applicable: true,
                    compensating_controls: vec![],
                    evidence_required: vec!["Identity verification process".to_string()],
                },
                Requirement {
                    requirement_id: "CC6.6.1".to_string(),
                    title: "Security for Endpoints".to_string(),
                    description: "Implements controls to prevent or detect unauthorized software".to_string(),
                    severity: RequirementSeverity::High,
                    applicable: true,
                    compensating_controls: vec![],
                    evidence_required: vec!["Endpoint protection logs".to_string()],
                },
            ],
            mappings: vec![],
            audit_schedule: AuditSchedule {
                frequency: AuditFrequency::Annually,
                next_audit: Some(DateTime::<Utc>::now()),
                auditor: "External Auditor".to_string(),
                scope: vec!["Security".to_string(), "Availability".to_string()],
            },
            certifications: vec![
                Certification {
                    cert_id: "soc2_cert_001".to_string(),
                    cert_type: CertificationType::SOC2TypeII,
                    issued_by: "AICPA".to_string(),
                    issued_date: DateTime<Utc>::now(),
                    expiry_date: Some(DateTime::<Utc>::now()),
                    status: CertificationStatus::Active,
                    scope: "Security and Availability".to_string(),
                },
            ],
        };

        self.frameworks.insert("soc2".to_string(), framework);
    }

    /// Initialize ISO 27001 framework
    pub fn init_iso27001(&mut self) {
        let framework = ComplianceFramework {
            framework_id: "iso27001".to_string(),
            name: "ISO/IEC 27001:2022".to_string(),
            version: "2022".to_string(),
            controls: vec![
                FrameworkControl {
                    control_id: "A.5.1".to_string(),
                    name: "Policies for Information Security".to_string(),
                    description: "Management direction and support for information security".to_string(),
                    category: ControlCategory::AccessControl,
                    subcategory: "Information Security Policies".to_string(),
                    control_type: ControlType::Administrative,
                    implementation_status: ImplementationStatus::Implemented,
                    last_reviewed: Some(DateTime::<Utc>::now()),
                    next_review: Some(DateTime::<Utc>::now()),
                },
                FrameworkControl {
                    control_id: "A.8.1".to_string(),
                    name: "Asset Responsibility".to_string(),
                    description: "Identification and classification of information assets".to_string(),
                    category: ControlCategory::DataProtection,
                    subcategory: "Asset Management".to_string(),
                    control_type: ControlType::Administrative,
                    implementation_status: ImplementationStatus::Implemented,
                    last_reviewed: Some(DateTime::<Utc>::now()),
                    next_review: Some(DateTime::<Utc>::now()),
                },
                FrameworkControl {
                    control_id: "A.9.1".to_string(),
                    name: "Access Control Policy".to_string(),
                    description: "Limit access to information and information processing facilities".to_string(),
                    category: ControlCategory::AccessControl,
                    subcategory: "Access Control".to_string(),
                    control_type: ControlType::Technical,
                    implementation_status: ImplementationStatus::Implemented,
                    last_reviewed: Some(DateTime::<Utc>::now()),
                    next_review: Some(DateTime::<Utc>::now()),
                },
            ],
            requirements: vec![
                Requirement {
                    requirement_id: "A.9.2.1".to_string(),
                    title: "User Registration".to_string(),
                    description: "Formal user registration and de-registration process".to_string(),
                    severity: RequirementSeverity::High,
                    applicable: true,
                    compensating_controls: vec![],
                    evidence_required: vec!["User registration forms".to_string()],
                },
            ],
            mappings: vec![],
            audit_schedule: AuditSchedule {
                frequency: AuditFrequency::Annually,
                next_audit: Some(DateTime::<Utc>::now()),
                auditor: "Certification Body".to_string(),
                scope: vec!["All Annex A controls".to_string()],
            },
            certifications: vec![],
        };

        self.frameworks.insert("iso27001".to_string(), framework);
    }

    /// Create compliance assessment
    pub fn create_assessment(&mut self, framework_id: &str, assessment_type: AssessmentType) -> Result<ComplianceAssessment, ComplianceError> {
        let framework = self.frameworks.get(framework_id)
            .ok_or(ComplianceError::FrameworkNotFound)?;

        let assessment = ComplianceAssessment {
            assessment_id: generate_id(),
            framework_id: framework_id.to_string(),
            assessment_type,
            scope: framework.audit_schedule.scope.clone(),
            status: AssessmentStatus::Planned,
            start_date: DateTime<Utc>::now(),
            end_date: None,
            assessor: String::new(),
            findings: Vec::new(),
            overall_score: 0.0,
            risk_rating: RiskRating::Medium,
        };

        self.assessments.push(assessment.clone());
        self.log_event(ComplianceEventType::AssessmentStarted, &assessment.assessment_id);

        Ok(assessment)
    }

    /// Add finding
    pub fn add_finding(&mut self, finding: ComplianceFinding) {
        self.findings.push(finding.clone());

        // Update audit trail
        self.log_event(ComplianceEventType::FindingCreated, &finding.finding_id);
    }

    /// Add evidence
    pub fn add_evidence(&mut self, evidence: Evidence) {
        self.evidence_store.evidence_items.push(evidence.clone());

        // Update chain of custody
        self.evidence_store.chain_of_custody.push(ChainOfCustodyEntry {
            entry_id: generate_id(),
            evidence_id: evidence.evidence_id,
            action: CustodyAction::Created,
            performed_by: "system".to_string(),
            performed_at: DateTime<Utc>::now(),
            notes: String::new(),
        });
    }

    /// Calculate compliance score
    pub fn calculate_compliance_score(&self, framework_id: &str) -> f32 {
        let framework = match self.frameworks.get(framework_id) {
            Some(f) => f,
            None => return 0.0,
        };

        let implemented = framework.controls.iter()
            .filter(|c| c.implementation_status == ImplementationStatus::Implemented)
            .count() as f32;

        let total = framework.controls.len() as f32;

        if total == 0.0 {
            return 100.0;
        }

        (implemented / total) * 100.0
    }

    /// Get framework status
    pub fn get_framework_status(&self, framework_id: &str) -> Option<FrameworkStatus> {
        let framework = self.frameworks.get(framework_id)?;

        let compliance_score = self.calculate_compliance_score(framework_id);
        let controls_implemented = framework.controls.iter()
            .filter(|c| c.implementation_status == ImplementationStatus::Implemented)
            .count();

        let findings_count = self.findings.iter()
            .filter(|f| {
                let framework = self.frameworks.get(&f.assessment_id).is_some();
                framework
            })
            .filter(|f| f.status != FindingStatus::Resolved)
            .count();

        Some(FrameworkStatus {
            framework_id: framework_id.to_string(),
            name: framework.name.clone(),
            compliance_score,
            controls_implemented,
            total_controls: framework.controls.len(),
            open_findings: findings_count,
            certification_status: framework.certifications.iter()
                .find(|c| c.status == CertificationStatus::Active)
                .map(|c| c.cert_type.clone()),
        })
    }

    /// Log audit event
    fn log_event(&mut self, event_type: ComplianceEventType, item_id: &str) {
        self.audit_trail.push_back(AuditEntry {
            entry_id: generate_id(),
            timestamp: DateTime<Utc>::now(),
            event_type,
            user: "system".to_string(),
            description: format!("{:?} on {}", event_type, item_id),
            affected_items: vec![item_id.to_string()],
        });

        // Limit trail size
        if self.audit_trail.len() > 10000 {
            self.audit_trail.pop_front();
        }
    }
}

impl Default for ComplianceEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Framework status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkStatus {
    pub framework_id: String,
    pub name: String,
    pub compliance_score: f32,
    pub controls_implemented: usize,
    pub total_controls: usize,
    pub open_findings: usize,
    pub certification_status: Option<CertificationType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceEventType {
    FrameworkAdded,
    AssessmentStarted,
    AssessmentCompleted,
    FindingCreated,
    FindingResolved,
    EvidenceAdded,
    ControlUpdated,
    CertificationObtained,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceError {
    FrameworkNotFound,
    AssessmentNotFound,
    InvalidFramework,
}

impl std::fmt::Display for ComplianceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplianceError::FrameworkNotFound => write!(f, "Framework not found"),
            ComplianceError::AssessmentNotFound => write!(f, "Assessment not found"),
            ComplianceError::InvalidFramework => write!(f, "Invalid framework"),
        }
    }
}

// ============================================================================
// Audit Log Entry
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub entry_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub user: String,
    pub resource: String,
    pub action: String,
    pub result: String,
    pub ip_address: String,
}

// ============================================================================
// Certification Type
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationType {
    SOC2TypeI,
    SOC2TypeII,
    ISO27001,
    ISO27017,
    ISO27018,
    SOC1,
    PCI,
    HIPAA,
    GDPR,
}


// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_engine_creation() {
        let engine = ComplianceEngine::new();
        assert!(!engine.engine_id.is_empty());
    }

    #[test]
    fn test_init_soc2() {
        let mut engine = ComplianceEngine::new();
        engine.init_soc2();

        assert!(engine.frameworks.contains_key("soc2"));
        let score = engine.calculate_compliance_score("soc2");
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_add_evidence() {
        let mut engine = ComplianceEngine::new();

        let evidence = Evidence {
            evidence_id: "ev1".to_string(),
            control_id: "ctrl1".to_string(),
            requirement_id: "req1".to_string(),
            evidence_type: EvidenceType::Document,
            title: "Test Evidence".to_string(),
            description: "Test description".to_string(),
            location: "/evidence/test.pdf".to_string(),
            collected_at: DateTime<Utc>::now(),
            collected_by: "auditor".to_string(),
            integrity_hash: "abc123".to_string(),
            metadata: HashMap::new(),
        };

        engine.add_evidence(evidence);
        assert_eq!(engine.evidence_store.evidence_items.len(), 1);
    }
}
