// Tariro AGI - Zero Trust Security Architecture
// Sovereign, defense-in-depth security model

use std::collections::{HashMap, HashSet, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// ============================================================================
// Zero Trust Types
// ============================================================================

/// Zero Trust Security Engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroTrustEngine {
    pub engine_id: String,
    pub identity_provider: IdentityProvider,
    pub policy_engine: PolicyEngine,
    pub network_security: NetworkSecurity,
    pub device_security: DeviceSecurity,
    pub data_security: DataSecurity,
    pub session_manager: SessionManager,
    pub audit_logger: AuditLogger,
    pub threat_detection: ThreatDetection,
    pub config: ZeroTrustConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProvider {
    pub identity_store: HashMap<String, Identity>,
    pub authentication_methods: Vec<AuthMethod>,
    pub mfa_config: MFAConfig,
    pub identity_providers: Vec<ExternalIdP>,
    pub password_policy: PasswordPolicy,
    pub session_store: HashMap<String, Session>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub identity_id: String,
    pub username: String,
    pub email: Option<String>,
    pub user_type: UserType,
    pub credentials: Vec<Credential>,
    pub attributes: IdentityAttributes,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_authenticated: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub risk_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAttributes {
    pub department: Option<String>,
    pub location: Option<String>,
    pub clearance_level: Option<ClearanceLevel>,
    pub custom_attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClearanceLevel {
    Public,
    Internal,
    Confidential,
    Secret,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub credential_id: String,
    pub credential_type: CredentialType,
    pub hashed_value: String,
    pub salt: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CredentialType {
    Password,
    SSHKey,
    APIKey,
    Certificate,
    Token,
    Biometric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub identity_id: String,
    pub device_id: Option<String>,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
    pub mfa_verified: bool,
    pub context: SessionContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub location: Option<String>,
    pub device_type: String,
    pub network_type: String,
    pub risk_score: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserType {
    Human,
    ServiceAccount,
    Robot,
    AI,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthMethod {
    pub method_type: AuthMethodType,
    pub is_enabled: bool,
    pub config: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethodType {
    Password,
    TOTP,
    HOTP,
    WebAuthn,
    HardwareKey,
    SMS,
    Email,
    Certificate,
    OAuth,
    SAML,
    LDAP,
    Kerberos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFAConfig {
    pub require_mfa: bool,
    pub mfa_methods: Vec<MFARequirement>,
    pub mfa_grace_period: Duration,
    pub trusted_devices: Vec<TrustedDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFARequirement {
    pub user_type: UserType,
    pub required_methods: Vec<AuthMethodType>,
    pub risk_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedDevice {
    pub device_id: String,
    pub device_fingerprint: String,
    pub registered_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub trust_level: TrustLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    Untrusted,
    Managed,
    Verified,
    Privileged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIdP {
    pub provider_id: String,
    pub provider_type: IdPType,
    pub client_id: String,
    pub endpoints: Vec<String>,
    pub signing_certs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdPType {
    AzureAD,
    Okta,
    Auth0,
    Google,
    GitHub,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special: bool,
    pub max_age_days: Option<u32>,
    pub history_length: usize,
    pub lockout_threshold: usize,
    pub lockout_duration: Duration,
}

// ============================================================================
// Policy Engine
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEngine {
    pub policies: Vec<SecurityPolicy>,
    pub policy_store: PolicyStore,
    pub decision_cache: HashMap<u64, PolicyDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub policy_id: String,
    pub name: String,
    pub policy_type: PolicyType,
    pub conditions: Vec<PolicyCondition>,
    pub actions: Vec<PolicyAction>,
    pub effect: PolicyEffect,
    pub priority: usize,
    pub is_enabled: bool,
    pub applies_to: Vec<TargetScope>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyType {
    AccessControl,
    NetworkSecurity,
    DataProtection,
    Authentication,
    Authorization,
    Audit,
    Encryption,
    Session,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    pub condition_type: ConditionType,
    pub attribute: String,
    pub operator: ConditionOperator,
    pub value: ConditionValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<String>),
    Pattern(String),
}

impl std::fmt::Display for ConditionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConditionValue::String(s) => write!(f, "\"{}\"", s),
            ConditionValue::Number(n) => write!(f, "{}", n),
            ConditionValue::Boolean(b) => write!(f, "{}", b),
            ConditionValue::List(items) => write!(f, "[{}]", items.join(", ")),
            ConditionValue::Pattern(p) => write!(f, "/{}/", p),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    Matches,
    GreaterThan,
    LessThan,
    In,
    NotIn,
    Regex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    Identity,
    Device,
    Network,
    Time,
    Location,
    Risk,
    Resource,
    Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    Allow,
    Deny,
    Challenge,
    Log,
    Alert,
    Quarantine,
    Encrypt,
    Mask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEffect {
    Permit,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetScope {
    pub target_type: TargetType,
    pub identifiers: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetType {
    User,
    Group,
    Role,
    Device,
    Network,
    Resource,
    Application,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub decision_id: String,
    pub decision: PolicyEffect,
    pub matched_policy: Option<String>,
    pub reasons: Vec<String>,
    pub obligations: Vec<Obligation>,
    pub timestamp: DateTime<Utc>,
    pub ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    pub obligation_type: ObligationType,
    pub parameter: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationType {
    RequireMFA,
    RequireEncryptedChannel,
    RequireManagedDevice,
    RequireSpecificLocation,
    RequireMinimumClearance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStore {
    pub policies_by_id: HashMap<String, SecurityPolicy>,
    pub policies_by_priority: Vec<String>,
}

// ============================================================================
// Network Security
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurity {
    pub zones: Vec<NetworkZone>,
    pub firewalls: Vec<FirewallRule>,
    pub intrusion_detection: IntrusionDetection,
    pub tls_config: TLSConfig,
    pub mTLS_enabled: bool,
    pub allowed_ports: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkZone {
    pub zone_id: String,
    pub name: String,
    pub zone_type: ZoneType,
    pub cidr_blocks: Vec<String>,
    pub security_level: SecurityLevel,
    pub allowed_transit: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneType {
    Public,
    DMZ,
    Internal,
    Restricted,
    Confidential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Untrusted,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub rule_id: String,
    pub name: String,
    pub priority: usize,
    pub action: FirewallAction,
    pub source_zones: Vec<String>,
    pub destination_zones: Vec<String>,
    pub protocols: Vec<Protocol>,
    pub ports: Vec<PortRange>,
    pub conditions: Vec<FirewallCondition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FirewallAction {
    Allow,
    Deny,
    Log,
    Alert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallCondition {
    pub attribute: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionDetection {
    pub enabled: bool,
    pub mode: IDSMode,
    pub rules: Vec<IDSRule>,
    pub alerts: Vec<SecurityAlert>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IDSMode {
    Passive,
    Active,
    Learning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDSRule {
    pub rule_id: String,
    pub name: String,
    pub severity: AlertSeverity,
    pub pattern: String,
    pub action: IDSAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IDSAction {
    Log,
    Alert,
    Drop,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSConfig {
    pub min_version: TLSVersion,
    pub max_version: TLSVersion,
    pub cipher_suites: Vec<String>,
    pub cert_path: String,
    pub private_key_path: String,
    pub ca_bundle_path: String,
    pub require_client_cert: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TLSVersion {
    TLS1_2,
    TLS1_3,
}

// ============================================================================
// Device Security
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSecurity {
    pub device_registry: HashMap<String, ManagedDevice>,
    pub compliance_policies: Vec<CompliancePolicy>,
    pub device_assessment: DeviceAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedDevice {
    pub device_id: String,
    pub device_type: DeviceType,
    pub owner: String,
    pub enrolled_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub platform: String,
    pub os_version: String,
    pub security_posture: SecurityPosture,
    pub certificates: Vec<DeviceCertificate>,
    pub encryption_status: EncryptionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Laptop,
    Desktop,
    Mobile,
    Tablet,
    Server,
    IoT,
    API,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPosture {
    pub os_updated: bool,
    pub antivirus_active: bool,
    pub firewall_active: bool,
    pub disk_encrypted: bool,
    pub screen_locked: bool,
    pub jailbroken: bool,
    pub compliance_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCertificate {
    pub cert_id: String,
    pub thumbprint: String,
    pub issued_to: String,
    pub issued_by: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionStatus {
    pub full_disk_encryption: bool,
    pub encryption_algorithm: Option<String>,
    pub key_management: KeyManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagement {
    pub method: KeyManagementMethod,
    pub hsm_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyManagementMethod {
    Software,
    TPM,
    HSM,
    CloudKMS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompliancePolicy {
    pub policy_id: String,
    pub name: String,
    pub framework: ComplianceFramework,
    pub requirements: Vec<ComplianceRequirement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceFramework {
    SOC2,
    ISO27001,
    NIST,
    HIPAA,
    PCI_DSS,
    GDPR,
    FedRAMP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub requirement_id: String,
    pub description: String,
    pub check_type: CheckType,
    pub remediation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckType {
    FileExists,
    RegistryValue,
    ProcessRunning,
    ServiceRunning,
    ConfigurationValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAssessment {
    pub device_id: String,
    pub assessed_at: DateTime<Utc>,
    pub findings: Vec<AssessmentFinding>,
    pub risk_score: f32,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentFinding {
    pub finding_id: String,
    pub severity: AlertSeverity,
    pub category: String,
    pub description: String,
    pub evidence: String,
    pub remediation: String,
}

// ============================================================================
// Data Security
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSecurity {
    pub classification_engine: DataClassifier,
    pub encryption_at_rest: EncryptionAtRest,
    pub tokenization: TokenizationService,
    pub data_loss_prevention: DLPEngine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataClassifier {
    pub rules: Vec<ClassificationRule>,
    pub labels: Vec<DataLabel>,
    pub auto_classify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLabel {
    pub label_id: String,
    pub name: String,
    pub color: String,
    pub description: String,
    pub handling_requirements: Vec<HandlingRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlingRequirement {
    pub requirement_type: HandlingType,
    pub details: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandlingType {
    Encryption,
    AccessControl,
    Retention,
    Transmission,
    Processing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationRule {
    pub rule_id: String,
    pub name: String,
    pub patterns: Vec<String>,
    pub assigned_label: String,
    pub confidence_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionAtRest {
    pub default_algorithm: EncryptionAlgorithm,
    pub key_rotation_period: Duration,
    pub key_store: KeyStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    AES256GCM,
    AES256CBC,
    ChaCha20,
    SM4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStore {
    pub store_type: KeyStoreType,
    pub uri: String,
    pub credentials: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyStoreType {
    AWSKMS,
    GCPKMS,
    AzureKeyVault,
    HashiCorpVault,
    AWSSealman,
    Local,
    HSM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationService {
    pub vault_url: String,
    pub token_format: String,
    pub reversible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DLPEngine {
    pub enabled: bool,
    pub policies: Vec<DLPPolicy>,
    pub scan_on_upload: bool,
    pub scan_on_download: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DLPPolicy {
    pub policy_id: String,
    pub name: String,
    pub data_patterns: Vec<DataPattern>,
    pub action: DLPAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPattern {
    pub pattern_id: String,
    pub pattern_type: PatternType,
    pub regex: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    CreditCard,
    SSN,
    Email,
    Phone,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DLPAction {
    Allow,
    Block,
    Quarantine,
    Alert,
}

// ============================================================================
// Session Manager
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionManager {
    pub sessions: HashMap<String, Session>,
    pub config: SessionConfig,
    pub active_sessions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub max_session_duration: Duration,
    pub idle_timeout: Duration,
    pub absolute_timeout: Duration,
    pub max_concurrent_sessions: usize,
    pub require_reauth_for_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogger {
    pub events: VecDeque<AuditEvent>,
    pub config: AuditConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub actor: ActorInfo,
    pub resource: ResourceInfo,
    pub action: String,
    pub outcome: Outcome,
    pub ip_address: String,
    pub user_agent: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorInfo {
    pub identity_id: String,
    pub username: String,
    pub user_type: UserType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub resource_type: String,
    pub resource_id: String,
    pub resource_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventType {
    Authentication,
    Authorization,
    DataAccess,
    DataModification,
    ConfigurationChange,
    AdministrativeAction,
    SecurityEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Success,
    Failure,
    Denied,
    PartialSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub enabled: bool,
    pub retention_days: u32,
    pub log_level: LogLevel,
    pub destinations: Vec<LogDestination>,
    pub sensitive_fields: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogDestination {
    pub destination_type: DestinationType,
    pub endpoint: String,
    pub format: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DestinationType {
    File,
    Syslog,
    SIEM,
    CloudLogging,
}

// ============================================================================
// Threat Detection
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetection {
    pub enabled: bool,
    pub detection_rules: Vec<ThreatRule>,
    pub anomalies: Vec<Anomaly>,
    pub ioCs: Vec<IOC>,
    pub threat_intel: ThreatIntelligence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatRule {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub severity: AlertSeverity,
    pub conditions: Vec<ThreatCondition>,
    pub threshold: f32,
    pub action: ThreatAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatAction {
    Log,
    Alert,
    Block,
    Quarantine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub anomaly_id: String,
    pub detected_at: DateTime<Utc>,
    pub anomaly_type: AnomalyType,
    pub score: f32,
    pub description: String,
    pub involved_entities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    Behavioral,
    Statistical,
    Sequence,
    EntityCentric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOC {
    pub ioc_id: String,
    pub ioc_type: IOCType,
    pub value: String,
    pub source: String,
    pub confidence: f32,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IOCType {
    IP,
    Domain,
    URL,
    FileHash,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    pub feeds: Vec<ThreatFeed>,
    pub last_updated: DateTime<Utc>,
    pub indicators: Vec<IOC>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatFeed {
    pub feed_id: String,
    pub name: String,
    pub url: String,
    pub feed_type: FeedType,
    pub update_frequency: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedType {
    BlockList,
    ThreatIntel,
    Malware,
    Phishing,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    pub alert_id: String,
    pub severity: AlertSeverity,
    pub title: String,
    pub description: String,
    pub source: String,
    pub detected_at: DateTime<Utc>,
    pub status: AlertStatus,
    pub assigned_to: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertStatus {
    Open,
    Investigating,
    Resolved,
    FalsePositive,
    Blocked,
}

// ============================================================================
// Configuration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroTrustConfig {
    pub default_deny: bool,
    pub assume_breach: bool,
    pub microsegmentation: bool,
    pub continuous_verification: bool,
    pub least_privilege: bool,
    pub inspection_required: bool,
}

// ============================================================================
// Implementation
// ============================================================================

impl ZeroTrustEngine {
    pub fn new(config: ZeroTrustConfig) -> Self {
        Self {
            engine_id: generate_id(),
            identity_provider: IdentityProvider {
                identity_store: HashMap::new(),
                authentication_methods: vec![],
                mfa_config: MFAConfig {
                    require_mfa: true,
                    mfa_methods: vec![],
                    mfa_grace_period: Duration::from_secs(3600),
                    trusted_devices: vec![],
                },
                identity_providers: vec![],
                password_policy: PasswordPolicy {
                    min_length: 12,
                    require_uppercase: true,
                    require_lowercase: true,
                    require_numbers: true,
                    require_special: true,
                    max_age_days: Some(90),
                    history_length: 5,
                    lockout_threshold: 5,
                    lockout_duration: Duration::from_secs(900),
                },
                session_store: HashMap::new(),
            },
            policy_engine: PolicyEngine {
                policies: vec![],
                policy_store: PolicyStore {
                    policies_by_id: HashMap::new(),
                    policies_by_priority: vec![],
                },
                decision_cache: HashMap::new(),
            },
            network_security: NetworkSecurity {
                zones: vec![],
                firewalls: vec![],
                intrusion_detection: IntrusionDetection {
                    enabled: true,
                    mode: IDSMode::Passive,
                    rules: vec![],
                    alerts: vec![],
                },
                tls_config: TLSConfig {
                    min_version: TLSVersion::TLS1_3,
                    max_version: TLSVersion::TLS1_3,
                    cipher_suites: vec![],
                    cert_path: String::new(),
                    private_key_path: String::new(),
                    ca_bundle_path: String::new(),
                    require_client_cert: true,
                },
                mTLS_enabled: true,
                allowed_ports: vec![443, 80, 22],
            },
            device_security: DeviceSecurity {
                device_registry: HashMap::new(),
                compliance_policies: vec![],
                device_assessment: DeviceAssessment {
                    device_id: String::new(),
                    assessed_at: DateTime<Utc>::now(),
                    findings: vec![],
                    risk_score: 0.0,
                    recommendation: String::new(),
                },
            },
            data_security: DataSecurity {
                classification_engine: DataClassifier {
                    rules: vec![],
                    labels: vec![],
                    auto_classify: true,
                },
                encryption_at_rest: EncryptionAtRest {
                    default_algorithm: EncryptionAlgorithm::AES256GCM,
                    key_rotation_period: Duration::from_secs(86400 * 30),
                    key_store: KeyStore {
                        store_type: KeyStoreType::HSM,
                        uri: String::new(),
                        credentials: None,
                    },
                },
                tokenization: TokenizationService {
                    vault_url: String::new(),
                    token_format: String::new(),
                    reversible: true,
                },
                data_loss_prevention: DLPEngine {
                    enabled: true,
                    policies: vec![],
                    scan_on_upload: true,
                    scan_on_download: true,
                },
            },
            session_manager: SessionManager {
                sessions: HashMap::new(),
                config: SessionConfig {
                    max_session_duration: Duration::from_secs(28800), // 8 hours
                    idle_timeout: Duration::from_secs(1800),         // 30 minutes
                    absolute_timeout: Duration::from_secs(86400),    // 24 hours
                    max_concurrent_sessions: 3,
                    require_reauth_for_sensitive: true,
                },
                active_sessions: 0,
            },
            audit_logger: AuditLogger {
                events: VecDeque::new(),
                config: AuditConfig {
                    enabled: true,
                    retention_days: 365,
                    log_level: LogLevel::Info,
                    destinations: vec![],
                    sensitive_fields: vec!["password".to_string(), "token".to_string()],
                },
            },
            threat_detection: ThreatDetection {
                enabled: true,
                detection_rules: vec![],
                anomalies: vec![],
                ioCs: vec![],
                threat_intel: ThreatIntelligence {
                    feeds: vec![],
                    last_updated: DateTime<Utc>::now(),
                    indicators: vec![],
                },
            },
            config,
        }
    }

    /// Authenticate user
    pub fn authenticate(&mut self, username: &str, password: &str, device_id: Option<&str>) -> Result<AuthResult, AuthError> {
        // Find identity
        let identity = self.identity_provider.identity_store.get(username)
            .ok_or(AuthError::UserNotFound)?;

        // Verify credentials
        let credential = identity.credentials.iter()
            .find(|c| c.credential_type == CredentialType::Password && c.is_primary)
            .ok_or(AuthError::InvalidCredentials)?;

        // Hash and compare password
        let hashed_input = hash_password(password, &credential.salt);
        if hashed_input != credential.hashed_value {
            self.log_audit_event(AuditEventType::Authentication, &identity.identity_id, Outcome::Failure);
            return Err(AuthError::InvalidCredentials);
        }

        // Check MFA if required
        let mfa_verified = if self.identity_provider.mfa_config.require_mfa {
            self.verify_mfa(identity)?;
            true
        } else {
            true
        };

        // Create session
        let session = Session {
            session_id: generate_id(),
            identity_id: identity.identity_id.clone(),
            device_id: device_id.map(String::from),
            started_at: DateTime<Utc>::now(),
            last_activity: DateTime<Utc>::now(),
            expires_at: DateTime<Utc>::now() + self.session_manager.config.max_session_duration,
            ip_address: String::new(),
            user_agent: String::new(),
            mfa_verified,
            context: SessionContext {
                location: None,
                device_type: String::new(),
                network_type: String::new(),
                risk_score: self.calculate_risk_score(identity, device_id),
            },
        };

        let session_id = session.session_id.clone();
        self.identity_provider.session_store.insert(session_id.clone(), session);
        self.session_manager.active_sessions += 1;

        self.log_audit_event(AuditEventType::Authentication, &identity.identity_id, Outcome::Success);

        Ok(AuthResult {
            session_id,
            mfa_required: !mfa_verified,
            risk_score: identity.risk_score,
            redirect_url: None,
        })
    }

    /// Verify MFA
    fn verify_mfa(&self, _identity: &Identity) -> Result<(), AuthError> {
        // In real implementation, would verify TOTP/HOTP/etc.
        Ok(())
    }

    /// Authorize access request
    pub fn authorize(&mut self, session_id: &str, resource: &str, action: &str) -> Result<PolicyDecision, AuthError> {
        let session = self.identity_provider.session_store.get(session_id)
            .ok_or(AuthError::InvalidSession)?;

        // Check if session is valid
        if Utc::now() > session.expires_at {
            return Err(AuthError::SessionExpired);
        }

        // Update last activity
        let mut session = session.clone();
        session.last_activity = Utc::now();
        self.identity_provider.session_store.insert(session_id.to_string(), session);

        // Build authorization request
        let authz_request = AuthorizationRequest {
            session_id: session_id.to_string(),
            identity_id: session.identity_id.clone(),
            resource: resource.to_string(),
            action: action.to_string(),
            context: AuthorizationContext {
                device_trust: session.context.device_type.clone(),
                network_type: session.context.network_type.clone(),
                location: session.context.location.clone(),
                time: DateTime<Utc>::now(),
            },
        };

        // Evaluate policies
        self.evaluate_policies(&authz_request)
    }

    /// Evaluate policies
    fn evaluate_policies(&self, request: &AuthorizationRequest) -> Result<PolicyDecision, AuthError> {
        let mut matched_policies = Vec::new();

        // Find applicable policies
        for policy in &self.policy_engine.policies {
            if !policy.is_enabled {
                continue;
            }

            // Check if policy applies to this request
            let applies = policy.applies_to.iter().any(|t| {
                match t.target_type {
                    TargetType::Resource => t.identifiers.contains(&request.resource),
                    TargetType::User | TargetType::Group | TargetType::Role => true,
                    _ => false,
                }
            });

            if !applies {
                continue;
            }

            // Evaluate conditions
            let conditions_met = policy.conditions.iter().all(|c| {
                self.evaluate_condition(c, request)
            });

            if conditions_met {
                matched_policies.push(policy);
            }
        }

        // Sort by priority and apply
        matched_policies.sort_by(|a, b| a.priority.cmp(&b.priority));

        if let Some(policy) = matched_policies.first() {
            Ok(PolicyDecision {
                decision_id: generate_id(),
                decision: policy.effect,
                matched_policy: Some(policy.policy_id.clone()),
                reasons: vec![format!("Matched policy: {}", policy.name)],
                obligations: vec![],
                timestamp: DateTime<Utc>::now(),
                ttl: Duration::from_secs(300),
            })
        } else {
            // Default deny if no policy matches
            Ok(PolicyDecision {
                decision_id: generate_id(),
                decision: if self.config.default_deny { PolicyEffect::Deny } else { PolicyEffect::Permit },
                matched_policy: None,
                reasons: vec!["No matching policy found".to_string()],
                obligations: vec![],
                timestamp: DateTime<Utc>::now(),
                ttl: Duration::from_secs(300),
            })
        }
    }

    /// Evaluate single condition
    fn evaluate_condition(&self, condition: &PolicyCondition, request: &AuthorizationRequest) -> bool {
        match condition.condition_type {
            ConditionType::Identity => {
                let value = match condition.operator {
                    ConditionOperator::Equals => request.identity_id == condition.value.to_string(),
                    ConditionOperator::In => {
                        if let ConditionValue::List(list) = &condition.value {
                            list.contains(&request.identity_id)
                        } else {
                            false
                        }
                    }
                    _ => false,
                };
                value
            }
            ConditionType::Risk => {
                // Evaluate risk conditions
                true
            }
            _ => true,
        }
    }

    /// Calculate risk score
    fn calculate_risk_score(&self, identity: &Identity, device_id: Option<&str>) -> f32 {
        let mut risk = identity.risk_score;

        // Check device trust
        if let Some(device_id) = device_id {
            if let Some(device) = self.device_security.device_registry.get(device_id) {
                risk += (1.0 - device.security_posture.compliance_score) * 0.3;
            }
        }

        risk.min(1.0).max(0.0)
    }

    /// Log audit event
    fn log_audit_event(&mut self, event_type: AuditEventType, identity_id: &str, outcome: Outcome) {
        let event = AuditEvent {
            event_id: generate_id(),
            timestamp: DateTime<Utc>::now(),
            event_type,
            actor: ActorInfo {
                identity_id: identity_id.to_string(),
                username: identity_id.to_string(),
                user_type: UserType::Human,
            },
            resource: ResourceInfo {
                resource_type: "authentication".to_string(),
                resource_id: String::new(),
                resource_name: String::new(),
            },
            action: "authenticate".to_string(),
            outcome,
            ip_address: String::new(),
            user_agent: String::new(),
            metadata: HashMap::new(),
        };

        self.audit_logger.events.push_back(event);

        // Limit event history
        if self.audit_logger.events.len() > 10000 {
            self.audit_logger.events.pop_front();
        }
    }

    /// Add identity
    pub fn add_identity(&mut self, identity: Identity) {
        self.identity_provider.identity_store.insert(identity.username.clone(), identity);
    }

    /// Add policy
    pub fn add_policy(&mut self, policy: SecurityPolicy) {
        self.policy_engine.policies.push(policy.clone());
        self.policy_engine.policy_store.policies_by_id.insert(policy.policy_id.clone(), policy);
    }
}

/// Authorization request
#[derive(Debug, Clone)]
pub struct AuthorizationRequest {
    pub session_id: String,
    pub identity_id: String,
    pub resource: String,
    pub action: String,
    pub context: AuthorizationContext,
}

#[derive(Debug, Clone)]
pub struct AuthorizationContext {
    pub device_trust: String,
    pub network_type: String,
    pub location: Option<String>,
    pub time: DateTime<Utc>,
}

/// Authentication result
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub session_id: String,
    pub mfa_required: bool,
    pub risk_score: f32,
    pub redirect_url: Option<String>,
}

/// Authentication error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthError {
    UserNotFound,
    InvalidCredentials,
    AccountLocked,
    MFARequired,
    DeviceNotRegistered,
    SessionExpired,
    InvalidSession,
    InsufficientPermissions,
    PolicyDenied,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::AccountLocked => write!(f, "Account locked"),
            AuthError::MFARequired => write!(f, "MFA required"),
            AuthError::DeviceNotRegistered => write!(f, "Device not registered"),
            AuthError::SessionExpired => write!(f, "Session expired"),
            AuthError::InvalidSession => write!(f, "Invalid session"),
            AuthError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            AuthError::PolicyDenied => write!(f, "Policy denied"),
        }
    }
}

/// Hash password with salt
fn hash_password(password: &str, salt: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    salt.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("zt_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_trust_creation() {
        let config = ZeroTrustConfig {
            default_deny: true,
            assume_breach: true,
            microsegmentation: true,
            continuous_verification: true,
            least_privilege: true,
            inspection_required: true,
        };

        let engine = ZeroTrustEngine::new(config);
        assert!(!engine.engine_id.is_empty());
        assert!(engine.config.default_deny);
    }

    #[test]
    fn test_add_identity() {
        let mut engine = ZeroTrustEngine::new(ZeroTrustConfig::default());

        let identity = Identity {
            identity_id: "user1".to_string(),
            username: "testuser".to_string(),
            email: Some("test@example.com".to_string()),
            user_type: UserType::Human,
            credentials: vec![],
            attributes: IdentityAttributes {
                department: Some("IT".to_string()),
                location: Some("US".to_string()),
                clearance_level: Some(ClearanceLevel::Secret),
                custom_attributes: HashMap::new(),
            },
            roles: vec!["admin".to_string()],
            groups: vec![],
            created_at: DateTime<Utc>::now(),
            last_authenticated: None,
            is_active: true,
            risk_score: 0.1,
        };

        engine.add_identity(identity);
        assert_eq!(engine.identity_provider.identity_store.len(), 1);
    }
}
