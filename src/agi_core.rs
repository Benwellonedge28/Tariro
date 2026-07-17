// src/agi_core.rs
// AGI Core module - simplified tests for Tariro AGI

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::config::AGIConfig;
    use crate::telemetry::TelemetryEvent;
    use tokio::sync::mpsc;
    use async_trait::async_trait;
    use crate::event_bus::EventBus;
    use crate::quantum_security::QuantumSecurityManager;
    use crate::autonomy::AutonomyLevel;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Simple mock config for tests
    #[derive(Debug, Clone)]
    struct TestConfig {
        autonomy_level: AutonomyLevel,
    }

    impl Default for TestConfig {
        fn default() -> Self {
            TestConfig {
                autonomy_level: AutonomyLevel::FullAuto,
            }
        }
    }

    #[tokio::test]
    async fn test_agi_core_basic_initialization() {
        let config = AGIConfig::default();
        let (tx, _rx) = mpsc::channel(10);
        let telemetry_logger = crate::telemetry::TelemetryLogger::new(tx);

        let event_bus = Arc::new(EventBus::new(16));

        // Just verify basic initialization works
        assert_eq!(config.get_autonomy_level(), AutonomyLevel::FullAuto);
        assert!(telemetry_logger.is_initialized());
    }

    #[tokio::test]
    async fn test_event_bus_functionality() {
        let event_bus = Arc::new(EventBus::new(4));
        let (tx, mut rx) = mpsc::channel(10);

        // Test that event bus can receive events
        let received = Arc::new(Mutex::new(false));
        let received_clone = received.clone();

        tokio::spawn(async move {
            if rx.recv().await.is_some() {
                let mut r = received_clone.lock().await;
                *r = true;
            }
        });

        // Send a test event
        let event = crate::event_bus::AGIEvent::ErrorOccurred {
            component: "test".to_string(),
            error: "test error".to_string(),
            context: None,
        };

        event_bus.publish(event).await;

        // Give time for the event to propagate
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let r = received.lock().await;
        assert!(*r, "Event should have been received");
    }

    #[tokio::test]
    async fn test_telemetry_logging() {
        let (tx, mut rx) = mpsc::channel(100);
        let logger = crate::telemetry::TelemetryLogger::new(tx);

        logger.log_info("test_source", "Test info message").await;
        logger.log_warning("test_source", "Test warning message").await;
        logger.log_error("test_source", "Test error message").await;

        // Verify events were sent
        let mut count = 0;
        while let Some(event) = rx.recv().await {
            match event {
                TelemetryEvent::InfoLogged { .. } |
                TelemetryEvent::WarningOccurred { .. } |
                TelemetryEvent::ErrorOccurred { .. } => count += 1,
                _ => {}
            }
            if count >= 3 {
                break;
            }
        }
        assert_eq!(count, 3, "Should have received 3 telemetry events");
    }

    #[tokio::test]
    async fn test_autonomy_levels() {
        // Test different autonomy levels
        let levels = vec![
            AutonomyLevel::Manual,
            AutonomyLevel::Advisory,
            AutonomyLevel::Supervised,
            AutonomyLevel::Autonomous,
            AutonomyLevel::FullAuto,
        ];

        for level in levels {
            let config = AGIConfig::new(level);
            assert_eq!(config.get_autonomy_level(), level);
        }
    }

    #[tokio::test]
    async fn test_config_serialization() {
        let config = AGIConfig::default();
        let serialized = serde_json::to_string(&config).expect("Failed to serialize config");
        let deserialized: AGIConfig = serde_json::from_str(&serialized).expect("Failed to deserialize config");
        assert_eq!(config.get_autonomy_level(), deserialized.get_autonomy_level());
    }
}
