/// Module for Quantum Key Distribution (QKD) Network Simulation in Rust.
///
/// This module conceptually outlines a Rust-based implementation for a Quantum Key Distribution (QKD) Network Simulation.
/// This RAG-driven feature simulates the secure key exchange process over quantum channels, demonstrating unconditional
/// security based on the laws of quantum mechanics. It leverages RAG for QKD protocols (e.g., BB84, E91) and quantum
/// cryptography research.

use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::thread;
use std::time::Duration;
use rand::Rng;

// --- Conceptual Structures --- //

/// Represents a quantum bit (qubit) state (e.g., polarization).
#[derive(Debug, Clone, Copy)]
use serde::{Serialize, Deserialize};
pub enum QubitState {
    H, // Horizontal polarization
    V, // Vertical polarization
    D, // Diagonal polarization
    A, // Anti-diagonal polarization
}

impl QubitState {
    pub fn to_char(&self) -> char {
        match self {
            QubitState::H => '─',
            QubitState::V => '│',
            QubitState::D => '╱',
            QubitState::A => '╲',
        }
    }
}

/// Represents a measurement basis (e.g., rectilinear, diagonal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementBasis {
    Rectilinear, // Horizontal/Vertical
    Diagonal,    // Diagonal/Anti-diagonal
}

impl MeasurementBasis {
    pub fn to_char(&self) -> char {
        match self {
            MeasurementBasis::Rectilinear => '+',
            MeasurementBasis::Diagonal => 'x',
        }
    }
}

/// Represents a QKD participant (Alice or Bob).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QKDParticipant {
    pub participant_id: String,
    pub role: String, // "Alice" (sender) or "Bob" (receiver)
    pub raw_key: Vec<bool>,
    pub sifted_key: Vec<bool>,
}

/// Represents a QKD channel event.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QKDChannelEvent {
    pub event_id: String,
    pub sender: String,
    pub receiver: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub qubits_sent: u32,
    pub error_rate: f32,
    pub eavesdropping_detected: bool,
}

/// Overall QKDNS system report.
#[derive(Debug, Clone)]
pub struct QKDNS_Report {
    pub qkd_run_id: String,
    pub total_qubits_sent: u64,
    pub final_shared_key_length: u32,
    pub quantum_bit_error_rate: f32,
    pub eavesdropping_attempts_detected: u32,
    pub security_assurance_level: String,
    pub throughput_kbps: f32,
}

// --- QKDNS Engine --- //

pub struct QKDNS_Engine {
    pub qkd_protocols_quantum_cryptography_research_rag_db: HashMap<String, String>, // Conceptual: RAG for QKD protocols and quantum cryptography research
}

impl QKDNS_Engine {
    pub fn new() -> Self {
        let mut qpqc_db = HashMap::new();
        qpqc_db.insert("BB84_Protocol_Details".to_string(), "Detailed explanation of the BB84 Quantum Key Distribution protocol, including its four polarization states, random basis choices by Alice and Bob, sifting process, and privacy amplification steps. Discusses the fundamental principles of quantum mechanics that ensure its security against eavesdropping.".to_string());
        qpqc_db.insert("E91_Protocol_Entanglement_Based_QKD".to_string(), "Overview of the E91 Quantum Key Distribution protocol, which is based on quantum entanglement and Bell's theorem. Describes how entangled photon pairs are used to establish a shared secret key and detect eavesdropping by violating Bell's inequality.".to_string());

        QKDNS_Engine {
            qkd_protocols_quantum_cryptography_research_rag_db: qpqc_db,
        }
    }

    /// Simulates Alice sending a stream of randomly polarized qubits (conceptual RAG).
    pub fn alice_send_qubits_rag(&self, num_qubits: u32) -> Vec<QubitState> {
        println!("  [QKDNS]: Alice is sending {} qubits with random polarizations (RAG for BB84 protocol logic).", num_qubits);
        if let Some(bb84_info) = self.qkd_protocols_quantum_cryptography_research_rag_db.get("BB84_Protocol_Details") {
            format!("QKD RAG insight for Alice's sending: {}. Qubits: '{}'", bb84_info, num_qubits);
        }
        thread::sleep(Duration::from_millis(100));
        let mut rng = rand::thread_rng();

        (0..num_qubits).map(|_| match rng.gen_range(0..4) {
            0 => QubitState::H,
            1 => QubitState::V,
            2 => QubitState::D,
            _ => QubitState::A,
        }).collect()
    }

    /// Simulates Bob measuring qubits with random bases (conceptual RAG).
    pub fn bob_measure_qubits_rag(&self, qubits: &[QubitState]) -> (Vec<QubitState>, Vec<MeasurementBasis>) {
        println!("  [QKDNS]: Bob is measuring {} qubits with random bases (RAG for BB84 measurement strategy).", qubits.len());
        thread::sleep(Duration::from_millis(100));
        let mut rng = rand::thread_rng();

        let mut measured_states = Vec::new();
        let mut chosen_bases = Vec::new();

        for &qubit in qubits {
            let basis = if rng.gen_bool(0.5) { MeasurementBasis::Rectilinear } else { MeasurementBasis::Diagonal };
            chosen_bases.push(basis);
            // Simulate measurement outcome based on qubit state and chosen basis
            let measured_state = match (qubit, basis) {
                (QubitState::H, MeasurementBasis::Rectilinear) => QubitState::H,
                (QubitState::V, MeasurementBasis::Rectilinear) => QubitState::V,
                (QubitState::D, MeasurementBasis::Rectilinear) => if rng.gen_bool(0.5) { QubitState::H } else { QubitState::V },
                (QubitState::A, MeasurementBasis::Rectilinear) => if rng.gen_bool(0.5) { QubitState::H } else { QubitState::V },
                (QubitState::H, MeasurementBasis::Diagonal) => if rng.gen_bool(0.5) { QubitState::D } else { QubitState::A },
                (QubitState::V, MeasurementBasis::Diagonal) => if rng.gen_bool(0.5) { QubitState::D } else { QubitState::A },
                (QubitState::D, MeasurementBasis::Diagonal) => QubitState::D,
                (QubitState::A, MeasurementBasis::Diagonal) => QubitState::A,
            };
            measured_states.push(measured_state);
        }
        (measured_states, chosen_bases)
    }

    /// Simulates public discussion and sifting to establish a shared key (conceptual RAG).
    pub fn sifting_and_reconciliation_rag(&self, alice_bases: &[MeasurementBasis], bob_bases: &[MeasurementBasis], alice_qubits: &[QubitState], bob_measured_qubits: &[QubitState]) -> (QKDChannelEvent, Vec<bool>, Vec<bool>) {
        println!("  [QKDNS]: Performing sifting and reconciliation (RAG for QKD security aspects).", );
        if let Some(e91_info) = self.qkd_protocols_quantum_cryptography_research_rag_db.get("E91_Protocol_Entanglement_Based_QKD") {
            format!("QKD RAG insight for sifting: {}. Bases count: {}", e91_info, alice_bases.len());
        }
        thread::sleep(Duration::from_millis(150));
        let mut rng = rand::thread_rng();

        let mut alice_raw_key = Vec::new();
        let mut bob_raw_key = Vec::new();
        let mut errors = 0;
        let mut eavesdropping_detected = false;

        for i in 0..alice_bases.len().min(bob_bases.len()) {
            if alice_bases[i] == bob_bases[i] {
                let alice_bit = match alice_qubits[i] {
                    QubitState::H | QubitState::D => false,
                    QubitState::V | QubitState::A => true,
                };
                let bob_bit = match bob_measured_qubits[i] {
                    QubitState::H | QubitState::D => false,
                    QubitState::V | QubitState::A => true,
                };
                alice_raw_key.push(alice_bit);
                bob_raw_key.push(bob_bit);

                if alice_bit != bob_bit {
                    errors += 1;
                    if rng.gen_bool(0.1) { eavesdropping_detected = true; } // Conceptual: error rate might indicate eavesdropping
                }
            }
        }

        let error_rate = errors as f32 / alice_raw_key.len() as f32;
        if error_rate > 0.05 { eavesdropping_detected = true; } // Higher error rate -> more likely eavesdropping

        let event = QKDChannelEvent {
            event_id: format!("sift_event_{}", rng.gen::<u32>()),
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            timestamp: Utc::now(),
            qubits_sent: alice_qubits.len() as u32,
            error_rate,
            eavesdropping_detected,
        };
        (event, alice_raw_key, bob_raw_key)
    }

    /// Generates a comprehensive report for QKD network simulation.
    pub fn generate_qkd_report_rag(&self, channel_event: &QKDChannelEvent, shared_key_length: u32) -> String {
        println!("  [QKDNS]: Generating QKDNS report for event '{}' (RAG for report generation).", channel_event.event_id);
        thread::sleep(Duration::from_millis(80));

        format!("Quantum Key Distribution Network Simulation Report for Event '{}':\n  Total Qubits Sent: {}\n  Final Shared Key Length: {}\n  Quantum Bit Error Rate (QBER): {:.2}%\n  Eavesdropping Detected: {}\n  (Conceptual summary based on RAG for security analysis and protocol efficiency).",
            channel_event.event_id,
            channel_event.qubits_sent,
            shared_key_length,
            channel_event.error_rate * 100.0,
            channel_event.eavesdropping_detected
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qkd_engine_flow() {
        let engine = QKDNS_Engine::new();
        let num_qubits = 100;

        let alice_qubits = engine.alice_send_qubits_rag(num_qubits);
        assert_eq!(alice_qubits.len() as u32, num_qubits);

        let alice_bases: Vec<MeasurementBasis> = (0..num_qubits).map(|_| if rand::thread_rng().gen_bool(0.5) { MeasurementBasis::Rectilinear } else { MeasurementBasis::Diagonal }).collect();
        let (bob_measured_qubits, bob_bases) = engine.bob_measure_qubits_rag(&alice_qubits);
        assert_eq!(bob_measured_qubits.len() as u32, num_qubits);

        let (event, alice_raw_key, bob_raw_key) = engine.sifting_and_reconciliation_rag(&alice_bases, &bob_bases, &alice_qubits, &bob_measured_qubits);
        assert!(!alice_raw_key.is_empty());
        assert_eq!(alice_raw_key, bob_raw_key); // Should be equal after sifting for matching bases

        let report = engine.generate_qkd_report_rag(&event, alice_raw_key.len() as u32);
        assert!(!report.is_empty());
    }
}

fn main() {
    println!("\n--- Quantum Key Distribution (QKD) Network Simulation Demonstration ---");

    let qkd_engine = QKDNS_Engine::new();
    let num_qubits_in_flight = 1000;

    // Scenario 1: Alice generates random bits and encodes them into qubits
    println!("--- Scenario 1: Alice Sends Qubits ---");
    let alice_original_bits: Vec<bool> = (0..num_qubits_in_flight).map(|_| rand::thread_rng().gen_bool(0.5)).collect();
    let alice_encoding_bases: Vec<MeasurementBasis> = (0..num_qubits_in_flight).map(|_| if rand::thread_rng().gen_bool(0.5) { MeasurementBasis::Rectilinear } else { MeasurementBasis::Diagonal }).collect();
    
    let alice_sent_qubits = alice_original_bits.iter().zip(alice_encoding_bases.iter()).map(|(&bit, &basis)| {
        match (bit, basis) {
            (false, MeasurementBasis::Rectilinear) => QubitState::H,
            (true, MeasurementBasis::Rectilinear) => QubitState::V,
            (false, MeasurementBasis::Diagonal) => QubitState::D,
            (true, MeasurementBasis::Diagonal) => QubitState::A,
        }
    }).collect::<Vec<QubitState>>();
    println!("  Alice sent {} qubits through quantum channel.\n", alice_sent_qubits.len());

    // Simulate Bob receiving and measuring qubits
    println!("--- Scenario 2: Bob Measures Qubits ---");
    let (bob_measured_qubits, bob_chosen_bases) = qkd_engine.bob_measure_qubits_rag(&alice_sent_qubits);
    println!("  Bob measured {} qubits using random bases.\n", bob_measured_qubits.len());

    // Scenario 3: Public discussion, sifting, and error rate check
    println!("--- Scenario 3: Sifting and Error Rate Check ---");
    let (channel_event, alice_final_key, bob_final_key) = qkd_engine.sifting_and_reconciliation_rag(
        &alice_encoding_bases,
        &bob_chosen_bases,
        &alice_sent_qubits,
        &bob_measured_qubits,
    );
    println!("  Sifting complete. Shared raw key length: {}. QBER: {:.2}%. Eavesdropping Detected: {}.\n",
        alice_final_key.len(),
        channel_event.error_rate * 100.0,
        channel_event.eavesdropping_detected
    );

    // Scenario 4: Generate QKD Network Simulation Report
    println!("--- Scenario 4: Generate QKDNS Report ---");
    let qkd_report_summary = qkd_engine.generate_qkd_report_rag(&channel_event, alice_final_key.len() as u32);
    println!("  {}\n", qkd_report_summary);

    println!("\nQuantum Key Distribution (QKD) Network Simulation Demonstration Complete.");
}
