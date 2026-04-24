use super::models::{IpAsset, IpStatus, IpType, IpDomain};
use chrono::Utc;

pub struct IpLifecycleManager;

impl IpLifecycleManager {
    /// Feature 6: IP Lifecycle Management
    /// Manages the entire lifecycle of IP, from conception and registration to maintenance, licensing, and eventual expiration/public domain.
    pub fn advance_lifecycle(&self, asset: &mut IpAsset, new_status: IpStatus) -> Result<(), String> {
        // Here, we can add complex logic to validate state transitions
        // and trigger various autonomous actions based on the new status.
        println!("Advancing lifecycle for IP '{}' from {:?} to {:?}", asset.title, asset.status, new_status);
        asset.update_status(new_status);
        Ok(())
    }

    /// Example of a specific lifecycle action, related to Feature 12 (Automated IP Maintenance & Renewal)
    pub fn renew_ip_asset(&self, asset: &mut IpAsset) -> Result<(), String> {
        if matches!(asset.status, IpStatus::Registered | IpStatus::Maintained) {
            asset.updated_at = Utc::now();
            asset.status = IpStatus::Maintained; // Explicitly set to Maintained or similar
            println!("IP Asset '{}' renewed successfully.", asset.title);
            Ok(())
        } else {
            Err(format!("IP Asset '{}' cannot be renewed in its current status: {:?}", asset.title, asset.status))
        }
    }
}
