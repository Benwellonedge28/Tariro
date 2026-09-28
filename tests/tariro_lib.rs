//! Tariro AGI - Library Integration Tests
//!
//! These tests verify the core functionality of the Tariro AGI system.

use tariro::*;

// ============================================================================
// Core System Tests
// ============================================================================

#[test]
fn test_tariro_version() {
    // Verify version is set correctly
    assert_eq!(env!("CARGO_PKG_VERSION"), "1.0.12");
}

#[test]
fn test_tariro_description() {
    // Verify package metadata
    assert_eq!(env!("CARGO_PKG_NAME"), "tariro");
    assert!(env!("CARGO_PKG_DESCRIPTION").contains("Tariro AGI"));
}

// ============================================================================
// Prelude Imports Tests
// ============================================================================

#[test]
fn test_prelude_serde_imports() {
    // Verify serde types are accessible through prelude
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestStruct {
        value: String,
    }

    let test = TestStruct { value: "test".to_string() };
    let serialized = serde_json::to_string(&test).unwrap();
    assert!(serialized.contains("test"));

    let deserialized: TestStruct = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.value, "test");
}

#[test]
fn test_prelude_collections_imports() {
    // Verify collection types are accessible
    use std::collections::{HashMap, HashSet, VecDeque};

    let mut hashmap = HashMap::new();
    hashmap.insert("key", "value");
    assert_eq!(hashmap.get("key"), Some(&"value"));

    let mut hashset = HashSet::new();
    hashset.insert(42);
    assert!(hashset.contains(&42));

    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    assert_eq!(deque.len(), 2);
}

#[test]
fn test_prelude_chrono_imports() {
    // Verify chrono types are accessible
    use chrono::{DateTime, Utc};

    let now: DateTime<Utc> = Utc::now();
    assert!(now.timestamp() > 0);
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn test_config_creation() {
    let config = TariroConfig::default();
    assert!(config.max_concurrent_tasks > 0);
}

#[test]
fn test_config_validation() {
    let mut config = TariroConfig::default();
    config.max_concurrent_tasks = 100;
    assert_eq!(config.max_concurrent_tasks, 100);
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_error_types_exist() {
    // Verify error types are properly defined
    let _error = TariroError::Configuration("test error".to_string());
    let _result: Result<(), TariroError> = Err(TariroError::Configuration("test".to_string()));
}

// ============================================================================
// Module Exports Tests
// ============================================================================

#[test]
fn test_module_exports() {
    // Verify key modules are exported
    use tariro::prelude::*;
    use tariro::executive_function::ExecutiveFunction;
    use tariro::meta_cognition::MetacognitiveMonitor;
    use tariro::goal_management::GoalManager;

    // Verify constructors work
    let _exec = ExecutiveFunction::new();
    let _meta = MetacognitiveMonitor::new();
    let _goals = GoalManager::new();
}

// ============================================================================
// Utility Function Tests
// ============================================================================

#[test]
fn test_id_generation() {
    // Test that ID generation produces unique IDs
    let id1 = generate_id();
    let id2 = generate_id();

    assert!(!id1.is_empty());
    assert!(!id2.is_empty());
    assert_ne!(id1, id2);
}

#[test]
fn test_uuid_generation() {
    // Test UUID generation
    let uuid1 = generate_uuid();
    let uuid2 = generate_uuid();

    assert!(uuid1.len() > 0);
    assert_ne!(uuid1, uuid2);
}

// ============================================================================
// Serialization Roundtrip Tests
// ============================================================================

#[test]
fn test_serialization_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SerializationTest {
        name: String,
        value: i32,
        enabled: bool,
    }

    let original = SerializationTest {
        name: "test".to_string(),
        value: 42,
        enabled: true,
    };

    let json = serde_json::to_string(&original).unwrap();
    let parsed: SerializationTest = serde_json::from_str(&json).unwrap();

    assert_eq!(original, parsed);
}
