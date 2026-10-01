/// Module for Quantum Circuit Design & Simulation Framework in Rust.
///
/// This module conceptually outlines a Rust-based framework for designing,
/// simulating, and visualizing quantum circuits. It includes support for
/// common quantum gates, entanglement, and measurement operations, enabling
/// initial exploration and verification of quantum algorithms.

use crate::prelude::*;
use std::fmt;
use std::ops::{Add, Mul};
use std::collections::HashMap;
use rand::Rng;

/// Represents a complex number (a + bi).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    pub fn conjugate(&self) -> Self {
        Complex::new(self.re, -self.im)
    }

    pub fn modulus_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Complex::new(self.re * other.re - self.im * other.im, self.re * other.im + self.im * other.re)
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Complex::new(self.re * scalar, self.im * scalar)
    }
}

/// Represents a quantum bit in superposition. State is a vector of complex amplitudes.
/// For N qubits, the state vector has 2^N elements.
#[derive(Debug, Clone)]
pub struct QubitState {
    pub amplitudes: Vec<Complex>,
}

impl QubitState {
    /// Initializes a single qubit in the |0> state.
    pub fn new_zero_state() -> Self {
        QubitState { amplitudes: vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)] }
    }

    /// Initializes a single qubit in the |1> state.
    pub fn new_one_state() -> Self {
        QubitState { amplitudes: vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)] }
    }

    /// Initializes an N-qubit system in the |0...0> state.
    pub fn new_n_qubit_zero_state(n_qubits: usize) -> Self {
        let mut amplitudes = vec![Complex::new(0.0, 0.0); 1 << n_qubits];
        amplitudes[0] = Complex::new(1.0, 0.0);
        QubitState { amplitudes }
    }

    /// Returns the number of qubits this state represents (log2 of amplitude count).
    pub fn num_qubits(&self) -> usize {
        (self.amplitudes.len() as f64).log2() as usize
    }
}

/// Represents common quantum gates.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumGate {
    Hadamard,
    PauliX,
    PauliY,
    PauliZ,
    CNOT(usize, usize), // Control Qubit Index, Target Qubit Index
    /// More gates (e.g., Phase, T, S, Toffoli) would be added here.
}

impl fmt::Display for QuantumGate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuantumGate::Hadamard => write!(f, "H"),
            QuantumGate::PauliX => write!(f, "X"),
            QuantumGate::PauliY => write!(f, "Y"),
            QuantumGate::PauliZ => write!(f, "Z"),
            QuantumGate::CNOT(c, t) => write!(f, "CNOT(c:{}, t:{})", c, t),
        }
    }
}

/// The quantum circuit manager.
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub state: QubitState,
    pub history: Vec<(QuantumGate, Vec<usize>)>, // (Gate, affected_qubit_indices)
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        QuantumCircuit {
            num_qubits,
            state: QubitState::new_n_qubit_zero_state(num_qubits),
            history: Vec::new(),
        }
    }

    /// Applies a quantum gate to the specified qubit(s).
    pub fn apply_gate(&mut self, gate: QuantumGate, target_qubits: Vec<usize>) {
        println!("  [QuantumCircuit]: Applying gate: {} to qubits {:?}.", gate, target_qubits);
        self.history.push((gate.clone(), target_qubits.clone()));

        let num_amplitudes = self.state.amplitudes.len();
        let mut new_amplitudes = self.state.amplitudes.clone();

        match gate {
            QuantumGate::Hadamard => {
                let t_idx = target_qubits[0];
                for i in 0..num_amplitudes {
                    if (i >> t_idx) & 1 == 0 { // If target qubit is |0>
                        let idx1 = i;
                        let idx2 = i | (1 << t_idx); // Index where target qubit is |1>
                        new_amplitudes[idx1] = (self.state.amplitudes[idx1] + self.state.amplitudes[idx2]) * (1.0 / (2.0f64).sqrt());
                        new_amplitudes[idx2] = (self.state.amplitudes[idx1] - self.state.amplitudes[idx2]) * (1.0 / (2.0f64).sqrt());
                    }
                }
            },
            QuantumGate::PauliX => { // NOT gate
                let t_idx = target_qubits[0];
                for i in 0..num_amplitudes {
                    if (i >> t_idx) & 1 == 0 { // If target qubit is |0>
                        let idx1 = i;
                        let idx2 = i | (1 << t_idx); // Index where target qubit is |1>
                        new_amplitudes[idx1] = self.state.amplitudes[idx2];
                        new_amplitudes[idx2] = self.state.amplitudes[idx1];
                    }
                }
            },
            QuantumGate::CNOT(control_idx, target_idx) => {
                for i in 0..num_amplitudes {
                    if (i >> control_idx) & 1 == 1 { // If control qubit is |1>
                        if (i >> target_idx) & 1 == 0 { // And target qubit is |0>
                            let idx0 = i;
                            let idx1 = i | (1 << target_idx); // Flip target to |1>
                            new_amplitudes[idx0] = self.state.amplitudes[idx1];
                            new_amplitudes[idx1] = self.state.amplitudes[idx0];
                        }
                    }
                }
            },
            _ => { /* Other gates would have their matrix multiplication applied here */ }
        }

        self.state.amplitudes = new_amplitudes;
        // Normalize if necessary due to floating point inaccuracies, though basic gates maintain norm
        let total_prob: f64 = self.state.amplitudes.iter().map(|c| c.modulus_squared()).sum();
        if (total_prob - 1.0).abs() > 1e-9 {
            let factor = 1.0 / total_prob.sqrt();
            for amp in &mut self.state.amplitudes { *amp = *amp * factor; }
        }
    }

    /// Measures a specific qubit, collapsing the superposition.
    pub fn measure_qubit(&mut self, target_qubit: usize) -> u8 {
        println!("  [QuantumCircuit]: Measuring qubit {}.", target_qubit);
        let mut rng = rand::thread_rng();
        let mut probabilities = HashMap::new();

        for i in 0..self.state.amplitudes.len() {
            let state_val = (i >> target_qubit) & 1;
            let prob = self.state.amplitudes[i].modulus_squared();
            *probabilities.entry(state_val).or_insert(0.0) += prob;
        }

        let mut cumulative_prob = 0.0;
        let random_value: f64 = rng.gen();

        let measurement_result = if random_value < *probabilities.get(&0).unwrap_or(&0.0) { 0 } else { 1 };

        // Collapse the state based on measurement result
        let mut new_amplitudes = vec![Complex::new(0.0, 0.0); self.state.amplitudes.len()];
        let mut norm_factor = 0.0;

        for i in 0..self.state.amplitudes.len() {
            if ((i >> target_qubit) & 1) == measurement_result {
                new_amplitudes[i] = self.state.amplitudes[i];
                norm_factor += self.state.amplitudes[i].modulus_squared();
            }
        }

        let sqrt_norm_factor = norm_factor.sqrt();
        for amp in &mut new_amplitudes {
            *amp = *amp * (1.0 / sqrt_norm_factor);
        }
        self.state.amplitudes = new_amplitudes;

        measurement_result as u8
    }

    /// Prints the current state vector for visualization.
    pub fn visualize_state(&self) {
        println!("\n--- Current Quantum State ({} qubits) ---", self.num_qubits);
        for (i, amp) in self.state.amplitudes.iter().enumerate() {
            let state_str = format!("|{:0width$b}>", i, width = self.num_qubits);
            println!("  {}: {:.4} + {:.4}i (Probability: {:.4})", state_str, amp.re, amp.im, amp.modulus_squared());
        }
        println!("-------------------------------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qubit_zero_state() {
        let state = QubitState::new_zero_state();
        assert_eq!(state.amplitudes[0], Complex::new(1.0, 0.0));
        assert_eq!(state.amplitudes[1], Complex::new(0.0, 0.0));
    }

    #[test]
    fn test_hadamard_gate() {
        let mut circuit = QuantumCircuit::new(1);
        circuit.apply_gate(QuantumGate::Hadamard, vec![0]);
        // After H gate on |0>, state should be (|0> + |1>)/sqrt(2)
        let expected_amp = Complex::new(1.0 / (2.0f64).sqrt(), 0.0);
        assert!((circuit.state.amplitudes[0].re - expected_amp.re).abs() < 1e-9);
        assert!((circuit.state.amplitudes[1].re - expected_amp.re).abs() < 1e-9);
    }

    #[test]
    fn test_cnot_gate() {
        let mut circuit = QuantumCircuit::new(2);
        // Set initial state to |01> (amplitude[1] is 1.0)
        circuit.state.amplitudes = vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)];
        circuit.state.amplitudes[1] = Complex::new(1.0, 0.0);
        // Apply CNOT with control 0, target 1. State should remain |01> as control is 0.
        circuit.apply_gate(QuantumGate::CNOT(0, 1), vec![0, 1]);
        assert_eq!(circuit.state.amplitudes[1], Complex::new(1.0, 0.0));

        // Reset to |10>
        circuit.state.amplitudes = vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)];
        // Apply CNOT with control 0, target 1. State should become |11>.
        circuit.apply_gate(QuantumGate::CNOT(0, 1), vec![0, 1]);
        assert_eq!(circuit.state.amplitudes[3], Complex::new(1.0, 0.0));
    }
}

fn main() {
    println!("\n--- Quantum Circuit Design & Simulation Framework Demonstration ---");

    // 1. Initialize a 2-qubit circuit
    let mut circuit = QuantumCircuit::new(2);
    circuit.visualize_state(); // Initial state |00>

    // 2. Apply Hadamard gate to Qubit 0
    println!("\n--- Applying Hadamard to Qubit 0 ---");
    circuit.apply_gate(QuantumGate::Hadamard, vec![0]);
    circuit.visualize_state(); // Qubit 0 in superposition: (|0>+|1>)/sqrt(2)

    // 3. Apply CNOT gate (Control Qubit 0, Target Qubit 1)
    println!("\n--- Applying CNOT (Control Q0, Target Q1) ---");
    circuit.apply_gate(QuantumGate::CNOT(0, 1), vec![0, 1]);
    circuit.visualize_state(); // Entangled Bell State: (|00>+|11>)/sqrt(2)

    // 4. Measure Qubit 0 multiple times to demonstrate probabilistic outcomes and state collapse
    println!("\n--- Measuring Qubit 0 (Multiple Times) ---");
    let mut rng = rand::thread_rng();
    let mut results_q0 = Vec::new();
    for _ in 0..5 { // Perform 5 measurements
        // Re-create the entangled state for each measurement simulation
        let mut temp_circuit = QuantumCircuit::new(2);
        temp_circuit.apply_gate(QuantumGate::Hadamard, vec![0]);
        temp_circuit.apply_gate(QuantumGate::CNOT(0, 1), vec![0, 1]);

        let measurement = temp_circuit.measure_qubit(0);
        results_q0.push(measurement);
        println!("  Measurement of Qubit 0: {}", measurement);
        // If Qubit 0 is measured as 0, Qubit 1 must also be 0
        // If Qubit 0 is measured as 1, Qubit 1 must also be 1
        // (this would be observable if we measured Qubit 1 immediately after Qubit 0)
    }
    println!("  All Qubit 0 Measurements: {:?}", results_q0);

    // 5. Demonstrate State Collapse: Measure Qubit 0, then observe the state.
    println!("\n--- Demonstrating State Collapse ---");
    let mut circuit_collapse = QuantumCircuit::new(2);
    circuit_collapse.apply_gate(QuantumGate::Hadamard, vec![0]);
    circuit_collapse.apply_gate(QuantumGate::CNOT(0, 1), vec![0, 1]);
    circuit_collapse.visualize_state();

    let measured_val = circuit_collapse.measure_qubit(0);
    println!("  Measured Qubit 0 as: {}", measured_val);
    circuit_collapse.visualize_state(); // State should now be |00> or |11> fully

    println!("\nQuantum Circuit Design & Simulation Framework Demonstration Complete.");
}
