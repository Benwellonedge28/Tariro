use super::models::{IpAsset, IpStatus, IpType, IpDomain};
use chrono::{Utc, Duration}; // Add Duration for date calculations

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

    /// Feature 12: Automated IP Maintenance & Renewal
    /// Tariro autonomously manages all IP renewal deadlines, fees, and necessary documentation globally.
    pub async fn check_and_process_renewals(&self, ip_assets: &mut [IpAsset]) -> Vec<String> {
        let mut processed_assets = Vec::new();
        let now = Utc::now();

        println!("Initiating automated IP maintenance and renewal check...");

        for asset in ip_assets.iter_mut() {
            // Simulate a renewal period (e.g., renew every year, or if updated_at is old)
            // In a real system, this would be based on specific IP laws and registration dates.
            let needs_renewal = match asset.ip_type {
                IpType::Patent => (now - asset.updated_at) > Duration::days(365 * 3), // Patents often 20 years, with renewal fees
                IpType::Trademark => (now - asset.updated_at) > Duration::days(365 * 5), // Trademarks often 10 years, renewable
                _ => (now - asset.updated_at) > Duration::days(365), // Other types, annual check
            };

            if needs_renewal && (asset.status == IpStatus::Registered || asset.status == IpStatus::Maintained) {
                match self.renew_ip_asset(asset) {
                    Ok(_) => processed_assets.push(format!("Successfully renewed IP: {}", asset.title)),
                    Err(e) => processed_assets.push(format!("Failed to renew IP {}: {}", asset.title, e)),
                }
            }
        }
        if processed_assets.is_empty() {
            println!("No IP assets required renewal at this time.");
        } else {
            println!("Automated IP renewal process complete. Processed {} assets.", processed_assets.len());
        }
        processed_assets
    }
}
