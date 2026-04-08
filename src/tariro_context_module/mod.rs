pub mod voice_processor;
pub mod sensor_data_ingestor;
pub mod visual_analyzer;

pub trait ContextualSensor {
    fn get_context_data(&self) -> String;
    fn process_data(&self, data: String);
}

pub struct TariroContextManager {
    // Manages various sensor and input processors
    voice_proc: voice_processor::VoiceProcessor,
    sensor_ingestor: sensor_data_ingestor::SensorDataIngestor,
    visual_analyzer: visual_analyzer::VisualAnalyzer,
}

impl TariroContextManager {
    pub fn new() -> Self {
        TariroContextManager {
            voice_proc: voice_processor::VoiceProcessor::new(),
            sensor_ingestor: sensor_data_ingestor::SensorDataIngestor::new(),
            visual_analyzer: visual_analyzer::VisualAnalyzer::new(),
        }
    }

    pub fn process_multimodal_input(&self, input_type: &str, data: String) {
        match input_type {
            "voice" => self.voice_proc.process_voice_input(data),
            "sensor" => self.sensor_ingestor.ingest_sensor_data(data),
            "visual" => self.visual_analyzer.analyze_visual_input(data),
            _ => println!("Unknown input type: {}", input_type),
        }
    }

    // Example of integrating with existing AutonomyPolicy
    pub fn check_autonomy_for_sensor(&self, sensor_name: &str) -> bool {
        // Placeholder for actual AutonomyPolicy check
        println!("Checking AutonomyPolicy for sensor: {}", sensor_name);
        true // Assume allowed for now
    }
}
