use crate::prelude::*;
use super::models::{IpAsset, OwnershipShare};
use uuid::Uuid;
use chrono::Utc;

#[derive(Default)]
pub struct IpCollaborationManager {}

impl IpCollaborationManager {
    /// Feature 10: IP Collaboration & Joint Ownership Management
    /// Manages complex IP ownership structures for collaborative works (human-AI, AGI-AGI), including royalty splits and usage rights.
    pub fn create_collaborative_asset(&self, base_asset: IpAsset, collaborators: Vec<OwnershipShare>) -> Result<IpAsset, String> {
        let mut new_asset = base_asset;
        
        // Basic validation for ownership shares
        let total_percentage: f64 = collaborators.iter().map(|s| s.percentage).sum();
        if (total_percentage - 100.0).abs() > f64::EPSILON { // Using epsilon for float comparison
            return Err("Total ownership percentage must sum to 100%".to_string());
        }

        new_asset.id = Uuid::new_v4(); // Generate new ID for the collaborative work
        new_asset.created_at = Utc::now();
        new_asset.updated_at = Utc::now();
        new_asset.ownership_structure = collaborators; // Assign the ownership structure

        println!("Created collaborative IP asset: {} with ownership: {:?}", new_asset.title, new_asset.ownership_structure);
        Ok(new_asset)
    }

    pub fn update_ownership_share(&self, asset: &mut IpAsset, owner_id: &str, new_percentage: f64) -> Result<(), String> {
        if let Some(share) = asset.ownership_structure.iter_mut().find(|s| s.owner_id == owner_id) {
            share.percentage = new_percentage;
            asset.updated_at = Utc::now();
            println!("Updated ownership share for {} in asset {}.", owner_id, asset.title);
            Ok(())
        } else {
            Err(format!("Owner {} not found in asset {}.", owner_id, asset.title))
        }
    }
}
