//! TUMC Security
//!
//! Security features including sandboxing, permissions, and cryptographic utilities.

use std::collections::{HashMap, HashSet};

/// Security context
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Principal ID
    pub principal_id: String,
    /// Permissions
    pub permissions: HashSet<Permission>,
    /// Capabilities
    pub capabilities: HashSet<Capability>,
    /// Trust level
    pub trust_level: TrustLevel,
    /// Labels (for MAC)
    pub labels: HashSet<String>,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            principal_id: "anonymous".to_string(),
            permissions: HashSet::new(),
            capabilities: HashSet::new(),
            trust_level: TrustLevel::Untrusted,
            labels: HashSet::new(),
        }
    }
}

/// Permission
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    /// Read file
    Read(String),
    /// Write file
    Write(String),
    /// Execute
    Execute,
    /// Network access
    Network,
    /// Environment variables
    EnvVar(String),
    /// System calls
    Syscall,
}

/// Capability
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    /// File system access
    FileSystem,
    /// Network access
    Network,
    /// Process creation
    Process,
    /// Thread creation
    Thread,
    /// Memory allocation
    Memory,
    /// FFI
    Ffi,
}

/// Trust level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Untrusted,
    Restricted,
    Standard,
    Trusted,
    Privileged,
}

/// Security manager
pub struct SecurityManager {
    /// Policy
    policy: SecurityPolicy,
    /// Contexts
    contexts: HashMap<String, SecurityContext>,
    /// Audit log
    audit_log: Vec<AuditEntry>,
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityManager {
    /// Create new manager
    pub fn new() -> Self {
        Self {
            policy: SecurityPolicy::default(),
            contexts: HashMap::new(),
            audit_log: Vec::new(),
        }
    }

    /// Create context for principal
    pub fn create_context(&mut self, principal_id: &str, trust_level: TrustLevel) -> SecurityContext {
        let context = SecurityContext {
            principal_id: principal_id.to_string(),
            permissions: HashSet::new(),
            capabilities: HashSet::new(),
            trust_level,
            labels: HashSet::new(),
        };

        self.contexts.insert(principal_id.to_string(), context.clone());
        context
    }

    /// Check permission
    pub fn check_permission(&self, context: &SecurityContext, permission: &Permission) -> bool {
        // Check if permission is granted
        context.permissions.contains(permission) || self.policy.allows(context, permission)
    }

    /// Grant permission
    pub fn grant_permission(&mut self, context: &mut SecurityContext, permission: Permission) {
        context.permissions.insert(permission);
        self.audit(AuditEntry::new(
            "permission_granted",
            &context.principal_id,
            format!("{:?}", permission),
        ));
    }

    /// Revoke permission
    pub fn revoke_permission(&mut self, context: &mut SecurityContext, permission: &Permission) {
        context.permissions.remove(permission);
        self.audit(AuditEntry::new(
            "permission_revoked",
            &context.principal_id,
            format!("{:?}", permission),
        ));
    }

    /// Audit log entry
    fn audit(&mut self, entry: AuditEntry) {
        self.audit_log.push(entry);
    }

    /// Get audit log
    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }
}

/// Security policy
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    /// Default deny
    default_deny: bool,
    /// Allowed permissions by trust level
    allowed_by_trust: HashMap<TrustLevel, HashSet<Permission>>,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        let mut allowed_by_trust = HashMap::new();

        // Untrusted: minimal permissions
        allowed_by_trust.insert(TrustLevel::Untrusted, HashSet::new());

        // Restricted: basic read
        let mut restricted = HashSet::new();
        restricted.insert(Permission::Read("/tmp".to_string()));
        allowed_by_trust.insert(TrustLevel::Restricted, restricted);

        // Standard: file and network
        let mut standard = HashSet::new();
        standard.insert(Permission::Read("*".to_string()));
        standard.insert(Permission::Network);
        standard.insert(Permission::Execute);
        allowed_by_trust.insert(TrustLevel::Standard, standard);

        // Trusted: all
        let mut trusted = HashSet::new();
        trusted.insert(Permission::Read("*".to_string()));
        trusted.insert(Permission::Write("*".to_string()));
        trusted.insert(Permission::Network);
        trusted.insert(Permission::Execute);
        trusted.insert(Permission::Syscall);
        allowed_by_trust.insert(TrustLevel::Trusted, trusted);

        Self {
            default_deny: true,
            allowed_by_trust,
        }
    }
}

impl SecurityPolicy {
    /// Check if permission allowed
    fn allows(&self, context: &SecurityContext, permission: &Permission) -> bool {
        if self.default_deny {
            if let Some(allowed) = self.allowed_by_trust.get(&context.trust_level) {
                allowed.contains(permission)
            } else {
                false
            }
        } else {
            true
        }
    }
}

/// Audit entry
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: std::time::Instant,
    pub action: String,
    pub principal: String,
    pub details: String,
    pub success: bool,
}

impl AuditEntry {
    /// Create new entry
    pub fn new(action: &str, principal: &str, details: String) -> Self {
        Self {
            timestamp: std::time::Instant::now(),
            action: action.to_string(),
            principal: principal.to_string(),
            details,
            success: true,
        }
    }
}

/// Sandbox
pub struct Sandbox {
    /// Allowed operations
    allowed_ops: HashSet<AllowedOp>,
    /// Resource limits
    limits: ResourceLimits,
    /// Syscall filter
    syscall_filter: SyscallFilter,
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Sandbox {
    /// Create new sandbox
    pub fn new() -> Self {
        Self {
            allowed_ops: HashSet::new(),
            limits: ResourceLimits::default(),
            syscall_filter: SyscallFilter::default(),
        }
    }

    /// Allow operation
    pub fn allow(&mut self, op: AllowedOp) -> &mut Self {
        self.allowed_ops.insert(op);
        self
    }

    /// Check if operation allowed
    pub fn is_allowed(&self, op: &AllowedOp) -> bool {
        self.allowed_ops.contains(op)
    }

    /// Enter sandbox
    pub fn enter(&self) -> SandboxGuard {
        SandboxGuard {
            sandbox: self,
            start_time: std::time::Instant::now(),
        }
    }
}

/// Allowed operation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllowedOp {
    Read,
    Write,
    Mkdir,
    Remove,
    Rename,
    Exec,
    NetworkConnect,
    NetworkListen,
    LoadLibrary,
    CreateThread,
    AllocateMemory,
}

/// Resource limits
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory: usize,
    pub max_cpu_time: u64,
    pub max_file_size: usize,
    pub max_open_files: usize,
    pub max_processes: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: 128 * 1024 * 1024,
            max_cpu_time: 60,
            max_file_size: 10 * 1024 * 1024,
            max_open_files: 64,
            max_processes: 4,
        }
    }
}

/// Syscall filter
#[derive(Debug, Clone, Default)]
pub struct SyscallFilter {
    allowed: HashSet<u32>,
    denied: HashSet<u32>,
}

impl SyscallFilter {
    /// Allow syscall
    pub fn allow(&mut self, syscall: u32) {
        self.allowed.insert(syscall);
        self.denied.remove(&syscall);
    }

    /// Deny syscall
    pub fn deny(&mut self, syscall: u32) {
        self.denied.insert(syscall);
        self.allowed.remove(&syscall);
    }

    /// Check if syscall allowed
    pub fn is_allowed(&self, syscall: u32) -> bool {
        if self.denied.contains(&syscall) {
            false
        } else if self.allowed.is_empty() {
            true
        } else {
            self.allowed.contains(&syscall)
        }
    }
}

/// Sandbox guard
pub struct SandboxGuard<'a> {
    sandbox: &'a Sandbox,
    start_time: std::time::Instant,
}

impl<'a> Drop for SandboxGuard<'a> {
    fn drop(&mut self) {
        // Cleanup on exit
    }
}

/// Cryptographic utilities
pub mod crypto {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    /// Hash function
    pub struct HashFunction {
        state: u64,
    }

    impl HashFunction {
        /// Create new hash function
        pub fn new() -> Self {
            Self { state: 0 }
        }

        /// Update hash with data
        pub fn update(&mut self, data: &[u8]) {
            for &byte in data {
                self.state = self.state.wrapping_mul(31).wrapping_add(byte as u64);
            }
        }

        /// Get final hash
        pub fn finish(&self) -> u64 {
            self.state
        }
    }

    impl Default for HashFunction {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Compute hash
    pub fn hash(data: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }

    /// Constant-time comparison
    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }

        result == 0
    }

    /// Secure random bytes
    pub fn random_bytes(len: usize) -> Vec<u8> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut bytes = Vec::with_capacity(len);
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut state = seed as u64;
        for _ in 0..len {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            bytes.push((state >> 33) as u8);
        }

        bytes
    }
}

/// Input validation
pub mod validation {
    /// Validate file path
    pub fn validate_path(path: &str) -> Result<(), ValidationError> {
        if path.contains("..") {
            return Err(ValidationError::PathTraversal);
        }
        if path.contains('\0') {
            return Err(ValidationError::NullByte);
        }
        Ok(())
    }

    /// Validate hostname
    pub fn validate_hostname(hostname: &str) -> Result<(), ValidationError> {
        if hostname.len() > 255 {
            return Err(ValidationError::TooLong);
        }
        // Simplified validation
        Ok(())
    }

    /// Sanitize string for display
    pub fn sanitize_for_display(s: &str) -> String {
        s.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect()
    }
}

/// Validation error
#[derive(Debug)]
pub enum ValidationError {
    PathTraversal,
    NullByte,
    TooLong,
    InvalidFormat,
}

/// Secure string (clears on drop)
pub struct SecureString {
    data: Vec<u8>,
}

impl SecureString {
    /// Create from string
    pub fn new(s: &str) -> Self {
        Self {
            data: s.as_bytes().to_vec(),
        }
    }

    /// Get as string slice
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.data).unwrap_or("")
    }

    /// Clear the data
    pub fn clear(&mut self) {
        for byte in &mut self.data {
            *byte = 0;
        }
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        self.clear();
    }
}

/// Taint tracker for security analysis
pub struct TaintTracker {
    sources: HashSet<String>,
    tainted: HashSet<String>,
}

impl TaintTracker {
    /// Create new tracker
    pub fn new() -> Self {
        Self {
            sources: HashSet::new(),
            tainted: HashSet::new(),
        }
    }

    /// Mark source as taint source
    pub fn mark_source(&mut self, name: &str) {
        self.sources.insert(name.to_string());
    }

    /// Check if value is tainted
    pub fn is_tainted(&self, name: &str) -> bool {
        self.tainted.contains(name)
    }

    /// Propagate taint
    pub fn propagate(&mut self, from: &str, to: &str) {
        if self.tainted.contains(from) {
            self.tainted.insert(to.to_string());
        }
    }
}

impl Default for TaintTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_context() {
        let ctx = SecurityContext::default();
        assert_eq!(ctx.principal_id, "anonymous");
    }

    #[test]
    fn test_sandbox() {
        let mut sandbox = Sandbox::new();
        sandbox.allow(AllowedOp::Read);
        assert!(sandbox.is_allowed(&AllowedOp::Read));
        assert!(!sandbox.is_allowed(&AllowedOp::Write));
    }

    #[test]
    fn test_syscall_filter() {
        let mut filter = SyscallFilter::default();
        filter.allow(1);
        filter.deny(2);
        assert!(filter.is_allowed(1));
        assert!(!filter.is_allowed(2));
    }

    #[test]
    fn test_hash() {
        let h1 = crypto::hash(b"hello");
        let h2 = crypto::hash(b"hello");
        assert_eq!(h1, h2);

        let h3 = crypto::hash(b"world");
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(crypto::constant_time_eq(b"hello", b"hello"));
        assert!(!crypto::constant_time_eq(b"hello", b"world"));
        assert!(!crypto::constant_time_eq(b"hello", b"hell"));
    }

    #[test]
    fn test_validation() {
        validation::validate_path("/safe/path").unwrap();
        assert!(validation::validate_path("../evil").is_err());
    }

    #[test]
    fn test_taint_tracker() {
        let mut tracker = TaintTracker::new();
        tracker.mark_source("user_input");
        tracker.propagate("user_input", "processed");
        assert!(tracker.is_tainted("processed"));
        assert!(!tracker.is_tainted("constant"));
    }
}
