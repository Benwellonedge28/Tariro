pub mod models;
pub mod axiomatic;
pub mod fabric;
pub mod advanced; // Add this line

use models::{OntologicalIpAsset, CausalSignature, AxiomaticIpGovernance, UniversalCreativeFabricStatus};
use crate::ip_management::ontological_ip::advanced::AdvancedIpFeatures; // Import AdvancedIpFeatures

// ... (existing struct and impl for OntologicalIpManager)
#[derive(Default)]
pub struct OntologicalIpManager {
    pub ip_assets: HashMap<Uuid, OntologicalIpAsset>,
    pub axiomatic_governance: axiomatic::AxiomaticGovernanceEngine,
    pub fabric_integration: fabric::FabricIntegrationEngine,
    pub advanced_features: advanced::AdvancedIpFeatures, // Add this line
}

impl OntologicalIpManager {
    pub fn new() -> Self {
        OntologicalIpManager {
            ip_assets: HashMap::new(),
            axiomatic_governance: axiomatic::AxiomaticGovernanceEngine::new(),
            fabric_integration: fabric::FabricIntegrationEngine::new(),
            advanced_features: advanced::AdvancedIpFeatures::new(), // Initialize
        }
    }

    // ... existing functions ...
}
