use crate::prelude::*;
pub struct DigitalNudgeSystem;

impl DigitalNudgeSystem {
    pub fn new() -> Self {
        DigitalNudgeSystem {}
    }

    pub fn provide_contextual_nudge(&self, user_id: &str, activity: &str) {
        println!("Providing contextual nudge for user {} during activity: {}", user_id, activity);
        // Based on LearningInsights (productivity, well-being goals), provide gentle nudges
        if activity.contains("social media") && !activity.contains("work-related") {
            println!("Nudge: 'Remember your deep work policy. Shall I block social media for 30 mins?'");
        }
    }
}
