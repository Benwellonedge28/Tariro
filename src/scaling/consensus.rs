//! Distributed Consensus Protocol
//!
//! This module implements Byzantine fault-tolerant consensus for
//! Tariro AGI's distributed deployments.
//!
//! # Consensus Protocols
//!
//! | Protocol | Fault Tolerance | Latency | Use Case |
//! |----------|-----------------|---------|----------|
//! | Raft | Crash-only (f) | Low | Edge clusters |
//! | PBFT | Byzantine (f) | Medium | Cloud |
//! | PoET | Byzantine (f) | High | Planet-scale |

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Arc, RwLock};

/// Node identifier
pub type NodeId = u64;

/// Consensus state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusState {
    /// Follower - passive, waiting for leader
    Follower,

    /// Candidate - election in progress
    Candidate,

    /// Leader - accepting requests
    Leader,

    /// Observing - read-only mode
    Observing,
}

impl Default for ConsensusState {
    fn default() -> Self {
        ConsensusState::Follower
    }
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Total number of nodes in cluster
    pub n: usize,

    /// Byzantine fault tolerance threshold (n > 3f)
    pub f: usize,

    /// Consensus timeout (ms)
    pub timeout_ms: u64,

    /// Heartbeat interval (ms)
    pub heartbeat_ms: u32,

    /// Minimum quorum size
    pub quorum: usize,

    /// Enable Byzantine fault tolerance
    pub byzantine: bool,

    /// Leader lease duration (ms)
    pub leader_lease_ms: u64,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            n: 5,
            f: 1,
            timeout_ms: 5000,
            heartbeat_ms: 1000,
            quorum: 3,
            byzantine: false,
            leader_lease_ms: 10000,
        }
    }
}

impl ConsensusConfig {
    /// Create configuration for edge cluster (crash fault tolerance)
    pub fn for_edge_cluster(node_count: usize) -> Self {
        let f = (node_count - 1) / 3;
        Self {
            n: node_count,
            f,
            quorum: node_count / 2 + 1,
            byzantine: false,
            ..Default::default()
        }
    }

    /// Create configuration for cloud (Byzantine fault tolerance)
    pub fn for_cloud() -> Self {
        Self {
            n: 7,
            f: 2,
            quorum: 5,
            byzantine: true,
            ..Default::default()
        }
    }

    /// Create configuration for planet-scale
    pub fn for_planet(node_count: usize) -> Self {
        let f = node_count / 4;
        Self {
            n: node_count,
            f,
            quorum: node_count * 2 / 3 + 1,
            byzantine: true,
            timeout_ms: 30000,
            heartbeat_ms: 5000,
            leader_lease_ms: 60000,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConsensusError> {
        if self.n < 3 {
            return Err(ConsensusError::InvalidConfig(
                "n must be at least 3".to_string(),
            ));
        }
        if self.f >= self.n / 3 {
            return Err(ConsensusError::InvalidConfig(
                "f must be less than n/3 for safety".to_string(),
            ));
        }
        if self.quorum * 2 <= self.n {
            return Err(ConsensusError::InvalidConfig(
                "quorum must be more than n/2 for majority".to_string(),
            ));
        }
        Ok(())
    }
}

/// AGI value for consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgiValue {
    /// Unique identifier
    pub id: u64,

    /// Content type
    pub content_type: ValueType,

    /// Serialized content
    pub content: Vec<u8>,

    /// Hash for integrity
    pub hash: [u8; 32],

    /// Timestamp
    pub timestamp_ms: u64,

    /// Origin node
    pub origin: NodeId,
}

/// Value type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    /// Knowledge update
    Knowledge,

    /// Goal state
    Goal,

    /// Plan proposal
    Plan,

    /// Configuration change
    Config,

    /// Governance decision
    Governance,
}

/// Consensus message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMessage {
    /// Request vote (election)
    RequestVote {
        term: u64,
        candidate_id: NodeId,
        last_log_index: u64,
        last_log_term: u64,
    },

    /// Vote response
    VoteResponse {
        term: u64,
        voter_id: NodeId,
        vote_granted: bool,
    },

    /// Append entries (replication)
    AppendEntries {
        term: u64,
        leader_id: NodeId,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: u64,
    },

    /// Append entries response
    AppendResponse {
        term: u64,
        success: bool,
        match_index: u64,
    },

    /// Pre-prepare (PBFT)
    PrePrepare {
        view: u64,
        sequence: u64,
        digest: [u8; 32],
        value: AgiValue,
    },

    /// Prepare (PBFT)
    Prepare {
        view: u64,
        sequence: u64,
        digest: [u8; 32],
        node_id: NodeId,
    },

    /// Commit (PBFT)
    Commit {
        view: u64,
        sequence: u64,
        digest: [u8; 32],
        node_id: NodeId,
    },

    /// Heartbeat
    Heartbeat {
        term: u64,
        leader_id: NodeId,
        commit_index: u64,
    },
}

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Log index
    pub index: u64,

    /// Term number
    pub term: u64,

    /// Value
    pub value: AgiValue,

    /// Timestamp
    pub timestamp_ms: u64,
}

/// Commit result
#[derive(Debug, Clone)]
pub struct CommitResult {
    /// Commit index
    pub index: u64,

    /// Term
    pub term: u64,

    /// Committed value
    pub value: AgiValue,
}

/// Distributed consensus engine
pub struct DistributedConsensus {
    /// This node's ID
    pub node_id: NodeId,

    /// Configuration
    pub config: ConsensusConfig,

    /// Current state
    pub state: ConsensusState,

    /// Current term
    current_term: u64,

    /// Voted for
    voted_for: Option<NodeId>,

    /// Log
    log: VecDeque<LogEntry>,

    /// Commit index
    commit_index: u64,

    /// Last applied index
    last_applied: u64,

    /// Peer nodes
    peers: HashMap<NodeId, PeerState>,

    /// Match indices (for leader)
    match_indices: HashMap<NodeId, u64>,

    /// Next indices (for leader)
    next_indices: HashMap<NodeId, u64>,

    /// Message queue
    message_queue: VecDeque<ConsensusMessage>,

    /// Vote count
    votes_received: usize,

    /// PBFT view
    pbft_view: u64,

    /// PBFT sequence number
    pbft_sequence: u64,

    /// PBFT pre-prepare messages
    pbft_pre_prepares: HashMap<[u8; 32], ConsensusMessage>,

    /// PBFT prepare messages
    pbft_prepares: HashMap<([u8; 32], NodeId), bool>,

    /// PBFT commit messages
    pbft_commits: HashMap<([u8; 32], NodeId), bool>,
}

/// Peer state
#[derive(Debug, Clone)]
pub struct PeerState {
    pub id: NodeId,
    pub connected: bool,
    pub last_contact_ms: u64,
    pub match_index: u64,
    pub next_index: u64,
}

impl Default for DistributedConsensus {
    fn default() -> Self {
        Self::new(0, ConsensusConfig::default())
    }
}

impl DistributedConsensus {
    /// Create a new consensus instance
    pub fn new(node_id: NodeId, config: ConsensusConfig) -> Result<Self, ConsensusError> {
        config.validate()?;

        Ok(Self {
            node_id,
            config,
            state: ConsensusState::Follower,
            current_term: 0,
            voted_for: None,
            log: VecDeque::new(),
            commit_index: 0,
            last_applied: 0,
            peers: HashMap::new(),
            match_indices: HashMap::new(),
            next_indices: HashMap::new(),
            message_queue: VecDeque::new(),
            votes_received: 0,
            pbft_view: 0,
            pbft_sequence: 0,
            pbft_pre_prepares: HashMap::new(),
            pbft_prepares: HashMap::new(),
            pbft_commits: HashMap::new(),
        })
    }

    /// Create with peer list
    pub fn with_peers(
        node_id: NodeId,
        config: ConsensusConfig,
        peer_ids: Vec<NodeId>,
    ) -> Result<Self, ConsensusError> {
        let mut consensus = Self::new(node_id, config)?;

        for peer_id in peer_ids {
            consensus.peers.insert(
                peer_id,
                PeerState {
                    id: peer_id,
                    connected: false,
                    last_contact_ms: 0,
                    match_index: 0,
                    next_index: 0,
                },
            );
            consensus.next_indices.insert(peer_id, 1);
        }

        Ok(consensus)
    }

    /// Propose a value for consensus
    pub fn propose(&mut self, value: AgiValue) -> Result<u64, ConsensusError> {
        // Must be leader (or in PBFT mode)
        if self.state != ConsensusState::Leader {
            if self.config.byzantine {
                // In PBFT, any node can propose
                return self.pbft_propose(value);
            }
            return Err(ConsensusError::NotLeader(self.current_leader()));
        }

        // Append to log
        let index = self.append_to_log(value)?;

        // Replicate to followers
        self.replicate(index)?;

        Ok(index)
    }

    /// PBFT propose (Byzantine consensus)
    fn pbft_propose(&mut self, value: AgiValue) -> Result<u64, ConsensusError> {
        let digest = self.hash_value(&value);
        let sequence = self.pbft_sequence;

        // Send pre-prepare
        let pre_prepare = ConsensusMessage::PrePrepare {
            view: self.pbft_view,
            sequence,
            digest,
            value: value.clone(),
        };

        self.pbft_pre_prepares.insert(digest, pre_prepare.clone());
        self.message_queue.push_back(pre_prepare);

        // Record own prepare
        self.pbft_prepares.insert((digest, self.node_id), true);

        // Check if we have enough prepares
        if self.count_prepares(&digest) >= self.config.quorum {
            // Send commit
            let commit = ConsensusMessage::Commit {
                view: self.pbft_view,
                sequence,
                digest,
                node_id: self.node_id,
            };
            self.pbft_commits.insert((digest, self.node_id), true);
            self.message_queue.push_back(commit);
        }

        self.pbft_sequence += 1;

        Ok(value.id)
    }

    /// Handle incoming message
    pub fn handle_message(&mut self, message: ConsensusMessage) -> Result<Option<CommitResult>, ConsensusError> {
        match message {
            ConsensusMessage::RequestVote { term, candidate_id, .. } => {
                self.handle_request_vote(term, candidate_id)
            }
            ConsensusMessage::VoteResponse { term, voter_id, vote_granted } => {
                self.handle_vote_response(term, voter_id, vote_granted)
            }
            ConsensusMessage::AppendEntries { term, leader_id, entries, .. } => {
                self.handle_append_entries(term, leader_id, entries)
            }
            ConsensusMessage::AppendResponse { term, success, match_index } => {
                self.handle_append_response(term, success, match_index)
            }
            ConsensusMessage::PrePrepare { view, sequence, digest, value } => {
                self.handle_pre_prepare(view, sequence, digest, value)
            }
            ConsensusMessage::Prepare { view, digest, node_id, .. } => {
                self.handle_prepare(view, digest, node_id)
            }
            ConsensusMessage::Commit { view, digest, node_id, .. } => {
                self.handle_commit(view, digest, node_id)
            }
            ConsensusMessage::Heartbeat { term, leader_id, commit_index } => {
                self.handle_heartbeat(term, leader_id, commit_index)
            }
        }
    }

    /// Handle request vote
    fn handle_request_vote(
        &mut self,
        term: u64,
        candidate_id: NodeId,
    ) -> Result<Option<CommitResult>, ConsensusError> {
        if term > self.current_term {
            self.current_term = term;
            self.state = ConsensusState::Follower;
            self.voted_for = None;
        }

        let vote_granted = self.voted_for.is_none()
            || self.voted_for == Some(candidate_id);

        if vote_granted {
            self.voted_for = Some(candidate_id);
        }

        // Send vote response
        self.message_queue.push_back(ConsensusMessage::VoteResponse {
            term: self.current_term,
            voter_id: self.node_id,
            vote_granted,
        });

        Ok(None)
    }

    /// Handle vote response
    fn handle_vote_response(
        &mut self,
        term: u64,
        voter_id: NodeId,
        vote_granted: bool,
    ) -> Result<Option<CommitResult>, ConsensusError> {
        if term != self.current_term || self.state != ConsensusState::Candidate {
            return Ok(None);
        }

        if vote_granted {
            self.votes_received += 1;
            if self.votes_received >= self.config.quorum {
                // Become leader
                self.become_leader();
            }
        }

        Ok(None)
    }

    /// Handle append entries
    fn handle_append_entries(
        &mut self,
        term: u64,
        leader_id: NodeId,
        entries: Vec<LogEntry>,
    ) -> Result<Option<CommitResult>, ConsensusError> {
        if term > self.current_term {
            self.current_term = term;
        }

        self.state = ConsensusState::Follower;
        self.voted_for = None;

        let success = if entries.is_empty() {
            true
        } else {
            // Check log consistency
            entries.is_empty() || self.log.len() >= entries[0].index as usize
        };

        if success && !entries.is_empty() {
            for entry in entries {
                self.append_to_log(entry.value)?;
            }
        }

        // Send response
        self.message_queue.push_back(ConsensusMessage::AppendResponse {
            term: self.current_term,
            success,
            match_index: self.commit_index,
        });

        Ok(None)
    }

    /// Handle append response
    fn handle_append_response(
        &mut self,
        term: u64,
        success: bool,
        match_index: u64,
    ) -> Result<Option<CommitResult>, ConsensusError> {
        if term != self.current_term || self.state != ConsensusState::Leader {
            return Ok(None);
        }

        if success {
            self.match_indices.insert(term as u64, match_index); // Simplified
            self.update_commit_index()?;
        }

        Ok(None)
    }

    /// Handle PBFT pre-prepare
    fn handle_pre_prepare(
        &mut self,
        view: u64,
        sequence: u64,
        digest: [u8; 32],
        value: AgiValue,
    ) -> Result<Option<CommitResult>, ConsensusError> {
        if view != self.pbft_view {
            return Ok(None);
        }

        // Validate
        if self.pbft_pre_prepares.contains_key(&digest) {
            return Ok(None);
        }

        // Store pre-prepare
        self.pbft_pre_prepares.insert(digest, ConsensusMessage::PrePrepare {
            view,
            sequence,
            digest,
            value,
        });

        // Send prepare
        self.pbft_prepares.insert((digest, self.node_id), true);
        self.message_queue.push_back(ConsensusMessage::Prepare {
            view,
            sequence,
            digest,
            node_id: self.node_id,
        });

        // Check if we have enough prepares
        if self.count_prepares(&digest) >= self.config.quorum {
            self.pbft_commits.insert((digest, self.node_id), true);
            self.message_queue.push_back(ConsensusMessage::Commit {
                view,
                sequence,
                digest,
                node_id: self.node_id,
            });
        }

        Ok(None)
    }

    /// Handle PBFT prepare
    fn handle_prepare(
        &mut self,
        view: u64,
        digest: [u8; 32],
        node_id: NodeId,
    ) -> Result<Option<CommitResult>, ConsensusError> {
        if view != self.pbft_view {
            return Ok(None);
        }

        self.pbft_prepares.insert((digest, node_id), true);

        // Check threshold
        if self.count_prepares(&digest) >= self.config.quorum {
            if !self.pbft_commits.contains_key(&(digest, self.node_id)) {
                self.pbft_commits.insert((digest, self.node_id), true);
                self.message_queue.push_back(ConsensusMessage::Commit {
                    view,
                    sequence: self.pbft_sequence,
                    digest,
                    node_id: self.node_id,
                });
            }
        }

        Ok(None)
    }

    /// Handle PBFT commit
    fn handle_commit(
        &mut self,
        view: u64,
        digest: [u8; 32],
        node_id: NodeId,
    ) -> Result<Option<CommitResult>, ConsensusError> {
        if view != self.pbft_view {
            return Ok(None);
        }

        self.pbft_commits.insert((digest, node_id), true);

        // Check if committed
        if self.count_commits(&digest) >= self.config.quorum {
            if let Some(pre_prepare) = self.pbft_pre_prepares.remove(&digest) {
                if let ConsensusMessage::PrePrepare { value, .. } = pre_prepare {
                    let result = CommitResult {
                        index: value.id,
                        term: self.current_term,
                        value,
                    };

                    // Apply value
                    self.last_applied += 1;

                    return Ok(Some(result));
                }
            }
        }

        Ok(None)
    }

    /// Handle heartbeat
    fn handle_heartbeat(
        &mut self,
        term: u64,
        leader_id: NodeId,
        commit_index: u64,
    ) -> Result<Option<CommitResult>, ConsensusError> {
        if term >= self.current_term {
            self.current_term = term;
            self.state = ConsensusState::Follower;
            self.commit_index = commit_index;
        }

        Ok(None)
    }

    /// Count prepare messages
    fn count_prepares(&self, digest: &[u8; 32]) -> usize {
        self.pbft_prepares
            .iter()
            .filter(|(d, _), &&v| v && d.0 == *digest)
            .count()
    }

    /// Count commit messages
    fn count_commits(&self, digest: &[u8; 32]) -> usize {
        self.pbft_commits
            .iter()
            .filter(|(d, _), &&v| v && d.0 == *digest)
            .count()
    }

    /// Become leader
    fn become_leader(&mut self) {
        self.state = ConsensusState::Leader;

        // Initialize next indices
        let last_index = self.log.len() as u64;
        for peer_id in self.peers.keys() {
            self.next_indices.insert(*peer_id, last_index + 1);
            self.match_indices.insert(*peer_id, 0);
        }
    }

    /// Append to log
    fn append_to_log(&mut self, value: AgiValue) -> Result<u64, ConsensusError> {
        let index = self.log.len() as u64 + 1;
        let entry = LogEntry {
            index,
            term: self.current_term,
            value,
            timestamp_ms: 0, // Would be actual timestamp
        };
        self.log.push_back(entry);
        Ok(index)
    }

    /// Replicate to followers
    fn replicate(&mut self, index: u64) -> Result<(), ConsensusError> {
        for (peer_id, peer) in &mut self.peers {
            let entries: Vec<LogEntry> = self
                .log
                .iter()
                .skip(peer.next_index as usize)
                .cloned()
                .collect();

            self.message_queue.push_back(ConsensusMessage::AppendEntries {
                term: self.current_term,
                leader_id: self.node_id,
                prev_log_index: peer.next_index.saturating_sub(1),
                prev_log_term: 0, // Simplified
                entries,
                leader_commit: self.commit_index,
            });
        }

        Ok(())
    }

    /// Update commit index
    fn update_commit_index(&mut self) -> Result<(), ConsensusError> {
        // Find majority
        let mut counts: BTreeMap<u64, usize> = BTreeMap::new();
        for &idx in self.match_indices.values() {
            *counts.entry(idx).or_insert(0) += 1;
            *counts.entry(0).or_insert(0) += 1;
        }

        // Include own log
        let own_index = self.log.len() as u64;
        *counts.entry(own_index).or_insert(0) += 1;

        for (index, count) in counts.iter().rev() {
            if *count >= self.config.quorum && *index > self.commit_index {
                self.commit_index = *index;
                break;
            }
        }

        Ok(())
    }

    /// Get current leader
    pub fn current_leader(&self) -> Option<NodeId> {
        match self.state {
            ConsensusState::Leader => Some(self.node_id),
            _ => self.peers.values().find(|p| p.connected).map(|p| p.id),
        }
    }

    /// Get pending messages
    pub fn pending_messages(&mut self) -> Vec<ConsensusMessage> {
        self.message_queue.drain(..).collect()
    }

    /// Get consensus statistics
    pub fn statistics(&self) -> ConsensusStats {
        ConsensusStats {
            node_id: self.node_id,
            state: self.state,
            term: self.current_term,
            log_size: self.log.len(),
            commit_index: self.commit_index,
            leader: self.current_leader(),
            connected_peers: self.peers.values().filter(|p| p.connected).count(),
            total_peers: self.peers.len(),
        }
    }

    /// Hash a value
    fn hash_value(&self, value: &AgiValue) -> [u8; 32] {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        value.id.hash(&mut hasher);
        value.timestamp_ms.hash(&mut hasher);
        let hash = hasher.finish();
        let mut result = [0u8; 32];
        result[..8].copy_from_slice(&hash.to_le_bytes());
        result
    }
}

/// Consensus statistics
#[derive(Debug, Clone)]
pub struct ConsensusStats {
    pub node_id: NodeId,
    pub state: ConsensusState,
    pub term: u64,
    pub log_size: usize,
    pub commit_index: u64,
    pub leader: Option<NodeId>,
    pub connected_peers: usize,
    pub total_peers: usize,
}

/// Consensus errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusError {
    /// Invalid configuration
    InvalidConfig(String),

    /// Not the leader
    NotLeader(Option<NodeId>),

    /// Term mismatch
    TermMismatch { expected: u64, actual: u64 },

    /// Log inconsistency
    LogInconsistency,

    /// Timeout
    Timeout,
}

impl std::fmt::Display for ConsensusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsensusError::InvalidConfig(s) => write!(f, "Invalid config: {}", s),
            ConsensusError::NotLeader(leader) => write!(
                f,
                "Not leader. Current leader: {:?}",
                leader
            ),
            ConsensusError::TermMismatch { expected, actual } => {
                write!(f, "Term mismatch: expected {}, got {}", expected, actual)
            }
            ConsensusError::LogInconsistency => write!(f, "Log inconsistency detected"),
            ConsensusError::Timeout => write!(f, "Consensus timeout"),
        }
    }
}

impl std::error::Error for ConsensusError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let valid = ConsensusConfig::default();
        assert!(valid.validate().is_ok());

        let invalid = ConsensusConfig {
            n: 2,
            f: 1,
            quorum: 1,
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_consensus_creation() {
        let config = ConsensusConfig::for_edge_cluster(5);
        let consensus = DistributedConsensus::new(1, config).unwrap();

        assert_eq!(consensus.state, ConsensusState::Follower);
        assert_eq!(consensus.current_term, 0);
    }

    #[test]
    fn test_propose_requires_leader() {
        let config = ConsensusConfig::for_edge_cluster(3);
        let mut consensus = DistributedConsensus::with_peers(1, config, vec![2, 3]).unwrap();

        let value = AgiValue {
            id: 1,
            content_type: ValueType::Knowledge,
            content: vec![1, 2, 3],
            hash: [0; 32],
            timestamp_ms: 0,
            origin: 1,
        };

        let result = consensus.propose(value);
        assert!(result.is_err());
    }

    #[test]
    fn test_statistics() {
        let config = ConsensusConfig::for_edge_cluster(3);
        let consensus = DistributedConsensus::with_peers(1, config, vec![2, 3]).unwrap();

        let stats = consensus.statistics();
        assert_eq!(stats.total_peers, 2);
    }
}
