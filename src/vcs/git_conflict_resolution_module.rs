// src/vcs/git_conflict_resolution_module.rs

//! Git Conflict Resolution Module for Tariro AGI
//! Provides intelligent conflict resolution for version control operations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a git conflict that needs resolution
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitConflict {
    pub id: String,
    pub file_path: String,
    pub base_content: String,
    pub ours_content: String,
    pub theirs_content: String,
    pub conflict_markers: Vec<ConflictRegion>,
    pub resolved_content: Option<String>,
    pub resolution_strategy: Option<ResolutionStrategy>,
    pub confidence_score: f32,
}

/// Represents a specific region of conflict within a file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConflictRegion {
    pub start_line: usize,
    pub end_line: usize,
    pub ours_lines: Vec<String>,
    pub theirs_lines: Vec<String>,
    pub common_ancestor_lines: Vec<String>,
}

/// Strategies for resolving conflicts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ResolutionStrategy {
    UseOurs,
    UseTheirs,
    MergeBoth,
    ManualResolution,
    ThreeWayMerge,
    Diff3Style,
    Custom(String),
}

/// Result of a conflict resolution attempt
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResolutionResult {
    pub conflict_id: String,
    pub success: bool,
    pub resolved_content: Option<String>,
    pub conflicts_remaining: usize,
    pub auto_resolved: bool,
    pub strategy_used: ResolutionStrategy,
    pub human_review_required: bool,
    pub confidence_score: f32,
}

/// Represents a diff between two content versions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentDiff {
    pub additions: Vec<DiffLine>,
    pub deletions: Vec<DiffLine>,
    pub modifications: Vec<ModificationPair>,
}

/// A single line in a diff
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffLine {
    pub line_number: usize,
    pub content: String,
    pub context: Option<String>,
}

/// A pair of modifications (before/after)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModificationPair {
    pub original_lines: Vec<String>,
    pub modified_lines: Vec<String>,
    pub line_range: (usize, usize),
}

/// GitConflictResolver provides intelligent conflict resolution
pub struct GitConflictResolver {
    pub enable_semantic_merge: bool,
    pub enable_structural_analysis: bool,
    pub confidence_threshold: f32,
    pub learned_resolutions: HashMap<String, ResolutionStrategy>,
}

impl GitConflictResolver {
    /// Creates a new GitConflictResolver
    pub fn new() -> Self {
        GitConflictResolver {
            enable_semantic_merge: true,
            enable_structural_analysis: true,
            confidence_threshold: 0.75,
            learned_resolutions: HashMap::new(),
        }
    }

    /// Detects conflicts in a given diff output
    pub fn detect_conflicts(&self, diff_content: &str) -> Vec<GitConflict> {
        let mut conflicts = Vec::new();
        let lines: Vec<&str> = diff_content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            if lines[i].starts_with("<<<<<<<") {
                let conflict = self.parse_conflict_block(&lines, i);
                if let Some(c) = conflict {
                    conflicts.push(c);
                    i += c.conflict_markers.iter().map(|m| m.end_line - m.start_line + 3).sum::<usize>();
                }
            }
            i += 1;
        }
        conflicts
    }

    fn parse_conflict_block(&self, lines: &[&str], start: usize) -> Option<GitConflict> {
        let file_path = self.extract_file_path(lines, start)?;
        let mut conflict = GitConflict {
            id: format!("conflict_{}", Uuid::new_v4()),
            file_path,
            base_content: String::new(),
            ours_content: String::new(),
            theirs_content: String::new(),
            conflict_markers: Vec::new(),
            resolved_content: None,
            resolution_strategy: None,
            confidence_score: 0.0,
        };

        let mut marker_start = start;
        let mut ours_lines = Vec::new();
        let mut theirs_lines = Vec::new();
        let mut current_section = Section::OursStart;

        for (idx, line) in lines[start..].iter().enumerate() {
            match line {
                s if s.starts_with("<<<<<<<") => {
                    current_section = Section::OursStart;
                }
                s if s.starts_with("=======") => {
                    current_section = Section::TheirsStart;
                }
                s if s.starts_with(">>>>>>>") => {
                    conflict.conflict_markers.push(ConflictRegion {
                        start_line: marker_start,
                        end_line: start + idx,
                        ours_lines: ours_lines.clone(),
                        theirs_lines: theirs_lines.clone(),
                        common_ancestor_lines: Vec::new(),
                    });
                    break;
                }
                _ => {
                    match current_section {
                        Section::OursStart | Section::OursContent => {
                            ours_lines.push(line.to_string());
                            current_section = Section::OursContent;
                        }
                        Section::TheirsStart | Section::TheirsContent => {
                            theirs_lines.push(line.to_string());
                            current_section = Section::TheirsContent;
                        }
                    }
                }
            }
        }

        conflict.ours_content = ours_lines.join("\n");
        conflict.theirs_content = theirs_lines.join("\n");
        Some(conflict)
    }

    fn extract_file_path(&self, lines: &[&str], start: usize) -> Option<String> {
        // Look backwards for "a/" or "b/" prefix
        for i in (0..start).rev() {
            if lines[i].contains("diff --git") {
                // Extract path from diff header
                if let Some(path) = lines[i].split(" b/").nth(1) {
                    return Some(path.to_string());
                }
            }
        }
        Some("unknown_file".to_string())
    }

    /// Analyzes conflicts and determines the best resolution strategy
    pub fn analyze_conflict(&self, conflict: &GitConflict) -> ResolutionStrategy {
        // Check learned resolutions first
        if let Some(strategy) = self.learned_resolutions.get(&conflict.file_path) {
            return strategy.clone();
        }

        // Analyze the conflict characteristics
        let ours_complexity = self.calculate_complexity(&conflict.ours_content);
        let theirs_complexity = self.calculate_complexity(&conflict.theirs_content);

        // Determine strategy based on analysis
        if conflict.ours_content.contains("<<<<<<<") || conflict.theirs_content.contains("<<<<<<<") {
            // Nested conflict - require manual resolution
            ResolutionStrategy::ManualResolution
        } else if self.is_structural_conflict(&conflict.ours_content, &conflict.theirs_content) {
            // Similar structure - try three-way merge
            ResolutionStrategy::ThreeWayMerge
        } else if ours_complexity > theirs_complexity * 2.0 {
            ResolutionStrategy::UseOurs
        } else if theirs_complexity > ours_complexity * 2.0 {
            ResolutionStrategy::UseTheirs
        } else {
            ResolutionStrategy::MergeBoth
        }
    }

    fn calculate_complexity(&self, content: &str) -> f32 {
        let lines = content.lines().count();
        let chars = content.len();
        let braces: usize = content.chars().filter(|c| *c == '{' || *c == '}').count();
        let parens: usize = content.chars().filter(|c| *c == '(' || *c == ')').count();

        // Weighted complexity score
        (lines as f32 * 1.0) + (chars as f32 * 0.1) + (braces as f32 * 2.0) + (parens as f32 * 0.5)
    }

    fn is_structural_conflict(&self, ours: &str, theirs: &str) -> bool {
        // Check if both have similar code structure (classes, functions, etc.)
        let our_structures = self.extract_structures(ours);
        let their_structures = self.extract_structures(theirs);

        // If both have similar structure, likely a merge conflict
        let common: Vec<_> = our_structures.intersection(&their_structures).collect();
        common.len() > 2
    }

    fn extract_structures(&self, content: &str) -> std::collections::HashSet<String> {
        let mut structures = std::collections::HashSet::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("fn ") ||
               trimmed.starts_with("pub fn ") ||
               trimmed.starts_with("class ") ||
               trimmed.starts_with("struct ") ||
               trimmed.starts_with("impl ") {
                structures.insert(trimmed.lines().next().unwrap_or("").to_string());
            }
        }
        structures
    }

    /// Attempts to resolve a conflict automatically
    pub fn resolve_conflict(&mut self, conflict: &mut GitConflict) -> ResolutionResult {
        let strategy = self.analyze_conflict(conflict);
        conflict.resolution_strategy = Some(strategy.clone());

        let (content, confidence) = match &strategy {
            ResolutionStrategy::UseOurs => {
                (conflict.ours_content.clone(), 0.95)
            }
            ResolutionStrategy::UseTheirs => {
                (conflict.theirs_content.clone(), 0.95)
            }
            ResolutionStrategy::MergeBoth => {
                self.merge_contents(&conflict.ours_content, &conflict.theirs_content)
            }
            ResolutionStrategy::ThreeWayMerge => {
                self.three_way_merge(&conflict.base_content, &conflict.ours_content, &conflict.theirs_content)
            }
            ResolutionStrategy::ManualResolution => {
                return ResolutionResult {
                    conflict_id: conflict.id.clone(),
                    success: false,
                    resolved_content: None,
                    conflicts_remaining: 1,
                    auto_resolved: false,
                    strategy_used: strategy,
                    human_review_required: true,
                    confidence_score: 0.0,
                };
            }
            ResolutionStrategy::Diff3Style => {
                self.diff3_style_merge(&conflict.ours_content, &conflict.theirs_content)
            }
            ResolutionStrategy::Custom(name) => {
                self.custom_merge(&conflict.ours_content, &conflict.theirs_content, name)
            }
        };

        conflict.resolved_content = Some(content);
        conflict.confidence_score = confidence;

        ResolutionResult {
            conflict_id: conflict.id.clone(),
            success: true,
            resolved_content: conflict.resolved_content.clone(),
            conflicts_remaining: 0,
            auto_resolved: true,
            strategy_used: strategy,
            human_review_required: confidence < self.confidence_threshold,
            confidence_score: confidence,
        }
    }

    fn merge_contents(&self, ours: &str, theirs: &str) -> (String, f32) {
        // Simple line-by-line merge
        let our_lines: Vec<&str> = ours.lines().collect();
        let their_lines: Vec<&str> = theirs.lines().collect();
        let mut merged = Vec::new();

        let max_len = our_lines.len().max(their_lines.len());
        for i in 0..max_len {
            let our_line = our_lines.get(i).copied();
            let their_line = their_lines.get(i).copied();

            match (our_line, their_line) {
                (Some(o), Some(t)) if o == t => {
                    merged.push(o.to_string());
                }
                (Some(o), Some(t)) => {
                    // Conflict on this line
                    merged.push(format!("// ====== MERGED CONFLICT ======"));
                    merged.push(format!("// ---OURS---"));
                    merged.push(o.to_string());
                    merged.push(format!("// ---THEIRS---"));
                    merged.push(t.to_string());
                    merged.push(format!("// ====== END CONFLICT ======"));
                }
                (Some(o), None) => merged.push(o.to_string()),
                (None, Some(t)) => merged.push(t.to_string()),
                _ => {}
            }
        }

        (merged.join("\n"), 0.6)
    }

    fn three_way_merge(&self, base: &str, ours: &str, theirs: &str) -> (String, f32) {
        // Three-way merge algorithm
        let base_lines: Vec<&str> = base.lines().collect();
        let our_lines: Vec<&str> = ours.lines().collect();
        let their_lines: Vec<&str> = theirs.lines().collect();

        let mut merged = Vec::new();
        let mut confidence = 1.0;

        // Simple LCS-based merge
        let lcs = self.longest_common_subsequence(&base_lines, &our_lines);
        let their_lcs = self.longest_common_subsequence(&base_lines, &their_lines);

        // Use the LCS with fewer changes
        if lcs.len() >= their_lcs.len() {
            merged.extend(lcs.iter().map(|s| s.to_string()));
        } else {
            merged.extend(their_lcs.iter().map(|s| s.to_string()));
        }

        (merged.join("\n"), 0.7)
    }

    fn longest_common_subsequence<'a>(&self, a: &[&'a str], b: &[&'a str]) -> Vec<&'a str> {
        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];

        for i in 1..=a.len() {
            for j in 1..=b.len() {
                if a[i-1] == b[j-1] {
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
                }
            }
        }

        // Backtrack to find LCS
        let mut result = Vec::new();
        let mut i = a.len();
        let mut j = b.len();

        while i > 0 && j > 0 {
            if a[i-1] == b[j-1] {
                result.push(a[i-1]);
                i -= 1;
                j -= 1;
            } else if dp[i-1][j] > dp[i][j-1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }

        result.reverse();
        result
    }

    fn diff3_style_merge(&self, ours: &str, theirs: &str) -> (String, f32) {
        let mut result = Vec::new();

        result.push("<<<<<<< OURS".to_string());
        result.extend(ours.lines().map(String::from));
        result.push("||||||| BASE".to_string());
        // Base would need to be provided separately for true diff3
        result.push("=======".to_string());
        result.extend(theirs.lines().map(String::from));
        result.push(">>>>>>> THEIRS".to_string());

        (result.join("\n"), 0.5)
    }

    fn custom_merge(&self, ours: &str, theirs: &str, strategy: &str) -> (String, f32) {
        match strategy {
            "ours_prefer" => {
                let mut merged = ours.lines().collect::<Vec<_>>();
                for line in theirs.lines() {
                    if !merged.contains(&line) {
                        merged.push(line);
                    }
                }
                (merged.join("\n"), 0.8)
            }
            "theirs_prefer" => {
                let mut merged = theirs.lines().collect::<Vec<_>>();
                for line in ours.lines() {
                    if !merged.contains(&line) {
                        merged.push(line);
                    }
                }
                (merged.join("\n"), 0.8)
            }
            _ => self.merge_contents(ours, theirs),
        }
    }

    /// Learns from a manual resolution
    pub fn learn_resolution(&mut self, file_path: &str, strategy: ResolutionStrategy) {
        self.learned_resolutions.insert(file_path.to_string(), strategy);
    }

    /// Generates a diff between two content versions
    pub fn generate_diff(&self, old: &str, new: &str) -> ContentDiff {
        let old_lines: Vec<&str> = old.lines().collect();
        let new_lines: Vec<&str> = new.lines().collect();

        let mut additions = Vec::new();
        let mut deletions = Vec::new();
        let mut modifications = Vec::new();

        let lcs = self.longest_common_subsequence(&old_lines, &new_lines);
        let lcs_set: std::collections::HashSet<_> = lcs.iter().collect();

        for (idx, line) in new_lines.iter().enumerate() {
            if !lcs_set.contains(line) {
                additions.push(DiffLine {
                    line_number: idx + 1,
                    content: line.to_string(),
                    context: None,
                });
            }
        }

        for (idx, line) in old_lines.iter().enumerate() {
            if !lcs_set.contains(line) {
                deletions.push(DiffLine {
                    line_number: idx + 1,
                    content: line.to_string(),
                    context: None,
                });
            }
        }

        ContentDiff { additions, deletions, modifications }
    }

    /// Batch resolves multiple conflicts
    pub fn resolve_batch(&mut self, conflicts: &mut Vec<GitConflict>) -> Vec<ResolutionResult> {
        conflicts
            .iter_mut()
            .map(|c| self.resolve_conflict(c))
            .collect()
    }
}

impl Default for GitConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq)]
enum Section {
    OursStart,
    OursContent,
    TheirsStart,
    TheirsContent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_simple_conflict() {
        let resolver = GitConflictResolver::new();
        let content = r#"<<<<<<< HEAD
function test() {
    return "ours";
}
=======
function test() {
    return "theirs";
}
>>>>>>> feature-branch"#;

        let conflicts = resolver.detect_conflicts(content);
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn test_merge_contents() {
        let resolver = GitConflictResolver::new();
        let ours = "line1\nline2\nline3";
        let theirs = "line1\nmodified\nline3";

        let (merged, confidence) = resolver.merge_contents(ours, theirs);
        assert!(merged.contains("MERGED CONFLICT"));
        assert!(confidence < 1.0);
    }

    #[test]
    fn test_longest_common_subsequence() {
        let resolver = GitConflictResolver::new();
        let a = ["a", "b", "c", "d", "e"];
        let b = ["b", "c", "e", "f"];

        let lcs = resolver.longest_common_subsequence(&a, &b);
        assert_eq!(lcs, vec!["b", "c", "e"]);
    }
}
