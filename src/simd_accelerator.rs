/// Module for SIMD-Accelerated Data Processing & Vector Operations in Rust.
///
/// This module conceptually outlines a Rust-based implementation for SIMD-accelerated
/// data processing and vector operations. It aims to leverage Rust's SIMD intrinsics
/// and libraries to perform parallel computations on arrays and vectors, which is
/// crucial for high-performance RAG operations (e.g., embedding comparisons,
/// numerical computations in QML) and general data manipulation.

use crate::prelude::*;
use std::arch::x86_64::{_mm256_add_ps, _mm256_mul_ps, _mm256_set1_ps, _mm255_loadu_ps, _mm255_storeu_ps, _mm256_fmadd_ps, _mm256_hadd_ps, _mm256_extract_f32x8};
use std::time::Duration;
use rand::Rng;

// NOTE: For actual SIMD, you would typically use `packed_simd` crate or directly
// `std::arch` intrinsics with appropriate `#[target_feature]` attributes and
// runtime CPU feature detection. This example provides a conceptual illustration.

/// Performs vector addition using SIMD (conceptual).
/// Adds corresponding elements of two input vectors `a` and `b` and stores the result in `output`.
#[cfg(target_feature = "avx2")] // Assuming AVX2 for _mm256 operations
pub fn add_vectors_simd(a: &[f32], b: &[f32], output: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), output.len());
    assert_eq!(a.len() % 8, 0, "Vector length must be a multiple of 8 for AVX2");

    unsafe {
        for i in (0..a.len()).step_by(8) {
            let vec_a = _mm256_loadu_ps(a.as_ptr().add(i));
            let vec_b = _mm256_loadu_ps(b.as_ptr().add(i));
            let vec_res = _mm256_add_ps(vec_a, vec_b);
            _mm256_storeu_ps(output.as_mut_ptr().add(i), vec_res);
        }
    }
}

/// Performs vector addition using scalar operations.
pub fn add_vectors_scalar(a: &[f32], b: &[f32], output: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), output.len());

    for i in 0..a.len() {
        output[i] = a[i] + b[i];
    }
}

/// Computes the dot product of two vectors using SIMD (conceptual).
/// This version accumulates sum using horizontal add and extracts the final sum.
#[cfg(target_feature = "avx2")]
pub fn dot_product_simd(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len() % 8, 0, "Vector length must be a multiple of 8 for AVX2");

    let mut sum_vec = unsafe { _mm256_set1_ps(0.0) };

    unsafe {
        for i in (0..a.len()).step_by(8) {
            let vec_a = _mm256_loadu_ps(a.as_ptr().add(i));
            let vec_b = _mm256_loadu_ps(b.as_ptr().add(i));
            sum_vec = _mm256_fmadd_ps(vec_a, vec_b, sum_vec); // Fused Multiply-Add
        }

        // Sum the elements of the 256-bit register
        let mut temp_array = [0.0f32; 8];
        _mm256_storeu_ps(temp_array.as_mut_ptr(), sum_vec);
        temp_array.iter().sum()

        // More efficient horizontal sum with AVX2:
        // let sum_h = _mm256_hadd_ps(sum_vec, sum_vec);
        // let sum_final = _mm256_hadd_ps(sum_h, sum_h);
        // _mm256_extract_f32x8(sum_final, 0).as_array()[0]
    }
}

/// Computes the dot product of two vectors using scalar operations.
pub fn dot_product_scalar(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    sum
}

/// Computes the cosine similarity between two embedding vectors using SIMD (conceptual).
/// Cosine similarity = (A . B) / (||A|| * ||B||)
#[cfg(target_feature = "avx2")]
pub fn cosine_similarity_simd(a: &[f32], b: &[f32]) -> f32 {
    let dot = dot_product_simd(a, b);
    let norm_a_sq = dot_product_simd(a, a);
    let norm_b_sq = dot_product_simd(b, b);

    if norm_a_sq == 0.0 || norm_b_sq == 0.0 { return 0.0; }
    dot / (norm_a_sq.sqrt() * norm_b_sq.sqrt())
}

/// Computes the cosine similarity between two embedding vectors using scalar operations.
pub fn cosine_similarity_scalar(a: &[f32], b: &[f32]) -> f32 {
    let dot = dot_product_scalar(a, b);
    let norm_a_sq = dot_product_scalar(a, a);
    let norm_b_sq = dot_product_scalar(b, b);

    if norm_a_sq == 0.0 || norm_b_sq == 0.0 { return 0.0; }
    dot / (norm_a_sq.sqrt() * norm_b_sq.sqrt())
}

/// Generates a random vector of f32.
fn generate_random_vec(len: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_feature = "avx2")]
    fn test_add_vectors_simd() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let b = vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let mut output = vec![0.0; 8];
        add_vectors_simd(&a, &b, &mut output);
        assert_eq!(output, vec![9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0]);
    }

    #[test]
    fn test_add_vectors_scalar() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![4.0, 3.0, 2.0, 1.0];
        let mut output = vec![0.0; 4];
        add_vectors_scalar(&a, &b, &mut output);
        assert_eq!(output, vec![5.0, 5.0, 5.0, 5.0]);
    }

    #[test]
    #[cfg(target_feature = "avx2")]
    fn test_dot_product_simd() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let b = vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let result = dot_product_simd(&a, &b);
        assert_eq!(result, 120.0);
    }

    #[test]
    fn test_dot_product_scalar() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![4.0, 3.0, 2.0, 1.0];
        let result = dot_product_scalar(&a, &b);
        assert_eq!(result, 20.0);
    }

    #[test]
    #[cfg(target_feature = "avx2")]
    fn test_cosine_similarity_simd() {
        let v1 = vec![1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let v2 = vec![1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let result = cosine_similarity_simd(&v1, &v2);
        // (1*1 + 0*1 + 1*0 + 0*0) / (sqrt(1^2+0^2+1^2+0^2) * sqrt(1^2+1^2+0^2+0^2))
        // = 1 / (sqrt(2) * sqrt(2)) = 1 / 2 = 0.5
        assert!((result - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_scalar() {
        let v1 = vec![1.0, 0.0, 1.0, 0.0];
        let v2 = vec![1.0, 1.0, 0.0, 0.0];
        let result = cosine_similarity_scalar(&v1, &v2);
        assert!((result - 0.5).abs() < 1e-6);
    }
}

fn main() {
    println!("\n--- SIMD-Accelerated Data Processing & Vector Operations Demonstration ---");

    // Vector length (must be a multiple of 8 for AVX2 example)
    let vec_len = 8192;

    // Generate random vectors
    let vec_a = generate_random_vec(vec_len);
    let vec_b = generate_random_vec(vec_len);
    let mut output_scalar = vec![0.0; vec_len];
    let mut output_simd = vec![0.0; vec_len];

    // --- Vector Addition Performance Comparison ---
    println!("\n--- Vector Addition ---");

    let start_scalar = Utc::now();
    add_vectors_scalar(&vec_a, &vec_b, &mut output_scalar);
    let duration_scalar = start_scalar.elapsed();
    println!("  Scalar Addition: {:?}", duration_scalar);

    #[cfg(target_feature = "avx2")]
    {
        let start_simd = Utc::now();
        add_vectors_simd(&vec_a, &vec_b, &mut output_simd);
        let duration_simd = start_simd.elapsed();
        println!("  SIMD (AVX2) Addition: {:?}", duration_simd);
        assert_eq!(output_scalar, output_simd);
    }

    // --- Dot Product Performance Comparison ---
    println!("\n--- Dot Product ---");

    let start_scalar = Utc::now();
    let dot_scalar = dot_product_scalar(&vec_a, &vec_b);
    let duration_scalar = start_scalar.elapsed();
    println!("  Scalar Dot Product: {:?} (Result: {:.4})", duration_scalar, dot_scalar);

    #[cfg(target_feature = "avx2")]
    {
        let start_simd = Utc::now();
        let dot_simd = dot_product_simd(&vec_a, &vec_b);
        let duration_simd = start_simd.elapsed();
        println!("  SIMD (AVX2) Dot Product: {:?} (Result: {:.4})", duration_simd, dot_simd);
        assert!((dot_scalar - dot_simd).abs() < 1e-3);
    }

    // --- Cosine Similarity (Embedding Comparison) Performance Comparison ---
    println!("\n--- Cosine Similarity (Embedding Comparison) ---");

    let start_scalar = Utc::now();
    let cos_sim_scalar = cosine_similarity_scalar(&vec_a, &vec_b);
    let duration_scalar = start_scalar.elapsed();
    println!("  Scalar Cosine Similarity: {:?} (Result: {:.4})", duration_scalar, cos_sim_scalar);

    #[cfg(target_feature = "avx2")]
    {
        let start_simd = Utc::now();
        let cos_sim_simd = cosine_similarity_simd(&vec_a, &vec_b);
        let duration_simd = start_simd.elapsed();
        println!("  SIMD (AVX2) Cosine Similarity: {:?} (Result: {:.4})", duration_simd, cos_sim_simd);
        assert!((cos_sim_scalar - cos_sim_simd).abs() < 1e-3);
    }

    println!("\nSIMD-Accelerated Data Processing & Vector Operations Demonstration Complete.");
}
