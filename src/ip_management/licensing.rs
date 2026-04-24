use super::models::{IpAsset, IpType};
use chrono::{Utc, Duration};

#[derive(Default)]
pub struct IpLicensingEngine {}

#[derive(Debug, Clone)]
pub struct LicenseAgreement {
    pub license_id: uuid::Uuid,
    pub asset_id: uuid::Uuid,
    pub licensee_id: String,
    pub agreement_terms: String,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: Option<chrono::DateTime<Utc>>,
    pub royalty_rate: Option<f64>, // Percentage
    pub is_exclusive: bool,
}

impl IpLicensingEngine {
    /// Feature 15: IP Licensing Template Generator
    /// Generates legally sound licensing agreements for various IP types, customizable via natural language or pre-defined templates.
    pub fn generate_license_template(&self, asset: &IpAsset, licensee: &str, terms_description: &str) -> Result<LicenseAgreement, String> {
        // In a production system, this would involve complex templating, legal clause generation
        // and natural language processing to customize agreements.
        let mut agreement_terms = format!(
            "This License Agreement for IP Asset \"{}\" (ID: {}) is granted by Samuel Mukandara to {}.\n\n",
            asset.title, asset.id, licensee
        );

        agreement_terms.push_str(&format!("Licensee is granted rights for: {}. Specific terms: {}.\n\n", self.ip_type_to_string(&asset.ip_type), terms_description));
        agreement_terms.push_str("Standard clauses for indemnification, warranties, and governing law apply.");

        let license = LicenseAgreement {
            license_id: uuid::Uuid::new_v4(),
            asset_id: asset.id,
            licensee_id: licensee.to_string(),
            agreement_terms,
            start_date: Utc::now(),
            end_date: Some(Utc::now() + Duration::days(365 * 5)), // Default 5 years
            royalty_rate: Some(5.0), // Default 5% royalty
            is_exclusive: false,
        };

        println!("Generated license agreement template for IP asset {}: {:?}", asset.title, license.license_id);
        Ok(license)
    }

    // A simple helper for display, would be more sophisticated
    fn ip_type_to_string(&self, ip_type: &IpType) -> &'static str {
        match ip_type {
            IpType::Patent => "Patent",
            IpType::Copyright => "Copyright",
            IpType::Trademark => "Trademark",
            IpType::TradeSecret => "Trade Secret",
            IpType::DataRight => "Data Right",
            IpType::MoralRight => "Moral Right",
            IpType::IndigenousKnowledge => "Indigenous Knowledge",
            IpType::MetaIp => "Meta IP",
        }
    }
}
