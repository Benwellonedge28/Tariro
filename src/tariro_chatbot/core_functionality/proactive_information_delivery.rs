// src/tariro_chatbot/core_functionality/proactive_information_delivery.rs

pub struct ProactiveInformationDelivery;

impl ProactiveInformationDelivery {
    pub fn deliver_proactive_info(&self, info: &str) -> String {
        format!("Proactive Information Delivery: Interjected with relevant information or alerts during conversation: '{}'.", info)
    }
}
