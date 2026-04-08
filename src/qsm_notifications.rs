// src/qsm_notifications.rs

//! Tariro's Quantum Security Module Notification Service.
//! This service autonomously monitors for critical quantum threats and
//! generates actionable alerts, including direct notifications to the SUPER ADMIN.

use std::sync::{Arc, Mutex};
use tokio::time::{self, Duration};
use crate::telemetry::TelemetryLogger;
use crate::database::DatabaseService;
use crate::quantum_security::mod::{QuantumSecurityManager, QuantumAutonomyLevel};
use crate::quantum_security::threat_intelligence::{QuantumThreat, ThreatSeverity};
use crate::quantum_security::mod::prelude as qsm_prelude;
use crate::event_bus::{EventBus, AGIEvent};
use crate::config::AGIConfig;
use tariro_sdk::entities::NotificationRecord;
use tariro_sdk::entities::notification_record::{Category, CausalPriority, Channel};

/// The QSM Notification Service is responsible for autonomously monitoring quantum threats
/// and creating NotificationRecord entities for critical events.
pub struct QsmNotificationService {
    config: Arc<AGIConfig>,
    telemetry_logger: TelemetryLogger,
    db_service: Arc<DatabaseService>,
    qsm: Arc<Mutex<QuantumSecurityManager>>,
    event_bus: Arc<EventBus>,
    last_alert_level: Arc<Mutex<ThreatSeverity>>, 
}

impl QsmNotificationService {
    /// Creates a new `QsmNotificationService` instance.
    pub fn new(
        config: Arc<AGIConfig>,
        telemetry_logger: TelemetryLogger,
        db_service: Arc<DatabaseService>,
        qsm: Arc<Mutex<QuantumSecurityManager>>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        QsmNotificationService {
            config,
            telemetry_logger,
            db_service,
            qsm,
            event_bus,
            last_alert_level: Arc::new(Mutex::new(ThreatSeverity::Low)), 
        }
    }

    /// Initializes the service and starts the autonomous monitoring loop.
    pub async fn initialize(&self) -> qsm_prelude::Result<()> {
        info!("[QSM Notification Service] Initializing autonomous threat monitoring.");
        self.telemetry_logger.log_event(
            crate::telemetry::TelemetryEvent::ServiceInitialized { service_name: "QSMNotificationService".to_string() }
        ).await;

        let qsm_clone = self.qsm.clone();
        let telemetry_logger_clone = self.telemetry_logger.clone();
        let db_service_clone = self.db_service.clone();
        let event_bus_clone = self.event_bus.clone();
        let config_clone = self.config.clone();
        let last_alert_level_clone = self.last_alert_level.clone();

        tokio::spawn(async move {
            loop {
                // Autonomous monitoring interval
                time::sleep(Duration::from_secs(config_clone.qsm_monitoring_interval_seconds)).await;

                let qsm_locked = qsm_clone.lock().await;
                let threat_intel = qsm_locked.threat_intel.get_current_threats().await; 
                drop(qsm_locked); 

                let mut current_highest_severity = ThreatSeverity::Low;
                for threat in threat_intel {
                    if threat.severity > current_highest_severity { 
                        current_highest_severity = threat.severity;
                    }

                    if threat.severity >= ThreatSeverity::Critical {
                        Self::generate_critical_notification(&config_clone, &telemetry_logger_clone, &db_service_clone, &event_bus_clone, &threat).await;
                    } else if threat.severity >= ThreatSeverity::High {
                        Self::generate_high_notification(&config_clone, &telemetry_logger_clone, &db_service_clone, &event_bus_clone, &threat).await;
                    }
                }

                // Update last_alert_level and potentially trigger higher-level alerts if severity escalates
                let mut last_level = last_alert_level_clone.lock().await;
                if current_highest_severity > *last_level {
                    info!("[QSM Notification Service] Quantum threat level escalated from {:?} to {:?}. Triggering adaptive response.", *last_level, current_highest_severity);
                }
                *last_level = current_highest_severity;
            }
        });

        Ok(())
    }

    /// Generates a critical notification for the SUPER ADMIN.
    async fn generate_critical_notification(
        config: &AGIConfig,
        telemetry_logger: &TelemetryLogger,
        db_service: &DatabaseService,
        event_bus: &EventBus,
        threat: &QuantumThreat,
    ) {
        error!("[QSM Notification Service] CRITICAL ALERT: Quantum threat detected: {}", threat.description);
        
        let notification_title = format!("CRITICAL QUANTUM THREAT: {}", threat.affected_algorithms.join(", "));
        let notification_content = format!(
            "An immediate and critical quantum threat has been detected.\nDescription: {}\nSeverity: {:?}\nAffected Algorithms: {:?}\nRecommended Action: {}\nPredicted Impact: {:?}",
            threat.description, threat.severity, threat.affected_algorithms, threat.recommended_action, threat.predicted_impact_date
        );

        let mut new_notification_record = NotificationRecord {
            id: None, 
            created_at: Utc::now(),
            updated_at: Utc::now(),
            title: notification_title.clone(),
            content: notification_content.clone(),
            category: Category::SecurityAlert,
            causal_priority: CausalPriority::Critical,
            recipients: Some(vec![config.super_admin_email.clone()]),
            channel: Some(Channel::NeuralAlert), 
            ini_verified: true, 
            mca_required: true, 
            privacy_masked: false, 
            project_id: None, 
            sent: false, 
        };

        match db_service.create(&mut new_notification_record).await {
            Ok(_) => info!("[QSM Notification Service] Critical quantum threat notification recorded and sent to SUPER ADMIN."),
            Err(e) => error!("[QSM Notification Service] Failed to record critical quantum threat notification: {}", e),
        }

        telemetry_logger.log_event(crate::telemetry::TelemetryEvent::SystemAlert {
            alert_type: "CriticalQuantumThreat".to_string(),
            description: notification_title,
            severity: "Critical".to_string(),
        }).await;

        event_bus.publish(AGIEvent::CriticalSystemAlert {
            message: "Critical Quantum Threat Detected".to_string()
        }).await;
    }

    /// Generates a high-severity notification.
    async fn generate_high_notification(
        config: &AGIConfig,
        telemetry_logger: &TelemetryLogger,
        db_service: &DatabaseService,
        event_bus: &EventBus,
        threat: &QuantumThreat,
    ) {
        warn!("[QSM Notification Service] HIGH ALERT: Quantum threat detected: {}", threat.description);
        
        let notification_title = format!("High Quantum Threat: {}", threat.affected_algorithms.join(", "));
        let notification_content = format!(
            "A high-severity quantum threat has been detected.\nDescription: {}\nSeverity: {:?}\nAffected Algorithms: {:?}\nRecommended Action: {}\nPredicted Impact: {:?}",
            threat.description, threat.severity, threat.affected_algorithms, threat.recommended_action, threat.predicted_impact_date
        );

        let mut new_notification_record = NotificationRecord {
            id: None, 
            created_at: Utc::now(),
            updated_at: Utc::now(),
            title: notification_title.clone(),
            content: notification_content.clone(),
            category: Category::SecurityAlert,
            causal_priority: CausalPriority::High,
            recipients: Some(vec![config.super_admin_email.clone()]),
            channel: Some(Channel::InApp),
            ini_verified: true,
            mca_required: false, 
            privacy_masked: true, 
            project_id: None,
            sent: false,
        };

        match db_service.create(&mut new_notification_record).await {
            Ok(_) => info!("[QSM Notification Service] High quantum threat notification recorded and sent."),
            Err(e) => error!("[QSM Notification Service] Failed to record high quantum threat notification: {}", e),
        }

        telemetry_logger.log_event(crate::telemetry::TelemetryEvent::SystemAlert {
            alert_type: "HighQuantumThreat".to_string(),
            description: notification_title,
            severity: "High".to_string(),
        }).await;
    }
}

// Custom PartialEq and PartialOrd for ThreatSeverity for easy comparison
impl PartialOrd for ThreatSeverity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ThreatSeverity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

impl PartialEq for ThreatSeverity {
    fn eq(&self, other: &Self) -> bool {
        self.to_usize() == other.to_usize()
    }
}

impl Eq for ThreatSeverity {}

impl ThreatSeverity {
    fn to_usize(&self) -> usize {
        match self {
            ThreatSeverity::Low => 0,
            ThreatSeverity::Medium => 1,
            ThreatSeverity::High => 2,
            ThreatSeverity::Critical => 3,
            ThreatSeverity::Existential => 4,
        }
    }
}
