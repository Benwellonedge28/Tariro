// Sea of Nodes IR Implementation
// Based on the HotSpot/C2 compiler approach

use super::*;
use std::collections::VecDeque;

/// Sea of Nodes representation - graph-based IR
#[derive(Debug, Clone)]
pub struct SeaOfNodes {
    pub nodes: HashMap<NodeId, SeaNode>,
    pub inputs: HashMap<NodeId, Vec<NodeId>>,
    pub outputs: HashMap<NodeId, Vec<NodeId>>,
    pub start: NodeId,
    pub end: NodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        NodeId(id)
    }

    pub fn idx(&self) -> usize {
        self.0
    }
}

/// Base node in the sea of nodes graph
#[derive(Debug, Clone)]
pub struct SeaNode {
    pub id: NodeId,
    pub opcode: SeaOpcode,
    pub inputs: Vec<NodeId>,
    pub output: Option<NodeId>,
    pub properties: NodeProperties,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeaOpcode {
    // Control nodes
    Start,
    End,
    Proj,
    Region,
    Phi,
    Goto,
    If,
    Switch,
    Return,
    Throw,
    Call,
    Intrinsic,

    // Memory nodes
    MemBar,
    Load,
    Store,
    LoadP,
    StoreP,
    Prefetch,
    Allocate,
    ArrayLength,

    // Constant nodes
    Const,
    ConstPtr,
    Null,

    // Arithmetic nodes
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Abs,
    Sqrt,

    // Bitwise nodes
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
    UShr,

    // Comparison nodes
    Cmp,
    CmpI,
    Bool,

    // Type conversion nodes
    Cast,
    Convert,
    Reinterpret,

    // Floating point nodes
    Sin,
    Cos,
    Exp,
    Log,
    Pow,

    // Control flow modifiers
    Branch,
    Merge,

    // Vector nodes
    VecLoad,
    VecStore,
    VecAdd,
    VecMul,
    VecOp,

    // Other
    Comment,
    Root,
}

#[derive(Debug, Clone, Default)]
pub struct NodeProperties {
    pub is_memory: bool,
    pub is_control: bool,
    pub is_pinned: bool,
    pub is_schedulable: bool,
    pub align: Option<usize>,
    pub latencies: Vec<usize>,
}

impl SeaOfNodes {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        let start = NodeId::new(0);
        let end = NodeId::new(1);

        // Create start node
        nodes.insert(start, SeaNode {
            id: start,
            opcode: SeaOpcode::Start,
            inputs: Vec::new(),
            output: None,
            properties: NodeProperties {
                is_control: true,
                is_pinned: true,
                is_schedulable: false,
            },
        });

        // Create end node
        nodes.insert(end, SeaNode {
            id: end,
            opcode: SeaOpcode::End,
            inputs: Vec::new(),
            output: None,
            properties: NodeProperties {
                is_control: true,
                is_pinned: true,
                is_schedulable: false,
            },
        });

        Self {
            nodes,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            start,
            end,
        }
    }

    pub fn next_id(&self) -> NodeId {
        NodeId::new(self.nodes.len())
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: SeaNode) -> NodeId {
        let id = node.id;
        self.nodes.insert(id, node);
        self.inputs.insert(id, Vec::new());
        self.outputs.insert(id, Vec::new());
        id
    }

    /// Connect two nodes (input -> output)
    pub fn connect(&mut self, input: NodeId, output: NodeId) {
        // Add output to input's outputs
        self.outputs.get_mut(&input).unwrap_or(&mut Vec::new()).push(output);

        // Add input to output's inputs
        self.inputs.get_mut(&output).unwrap_or(&mut Vec::new()).push(input);
    }

    /// Disconnect two nodes
    pub fn disconnect(&mut self, input: NodeId, output: NodeId) {
        if let Some(outputs) = self.outputs.get_mut(&input) {
            outputs.retain(|&n| n != output);
        }
        if let Some(inputs) = self.inputs.get_mut(&output) {
            inputs.retain(|&n| n != input);
        }
    }

    /// Get inputs of a node
    pub fn get_inputs(&self, node: NodeId) -> &[NodeId] {
        self.inputs.get(&node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get outputs of a node
    pub fn get_outputs(&self, node: NodeId) -> &[NodeId] {
        self.outputs.get(&node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&SeaNode> {
        self.nodes.get(&id)
    }

    /// Get a node by ID (mutable)
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut SeaNode> {
        self.nodes.get_mut(&id)
    }

    /// Create a constant node
    pub fn make_const(&mut self, value: ConstValue) -> NodeId {
        let id = self.next_id();
        let node = SeaNode {
            id,
            opcode: SeaOpcode::Const,
            inputs: Vec::new(),
            output: None,
            properties: NodeProperties::default(),
        };
        let const_id = self.add_node(node);

        // Store constant value in metadata
        self.nodes.get_mut(&const_id).unwrap().properties.align = Some(const_id.0);

        const_id
    }

    /// Create a region node (control flow merge)
    pub fn make_region(&mut self, inputs: Vec<NodeId>) -> NodeId {
        let id = self.next_id();
        let node = SeaNode {
            id,
            opcode: SeaOpcode::Region,
            inputs: inputs.clone(),
            output: None,
            properties: NodeProperties {
                is_control: true,
                is_pinned: true,
                is_schedulable: true,
            },
        };
        let region_id = self.add_node(node);

        for input in inputs {
            self.connect(input, region_id);
        }

        region_id
    }

    /// Create a phi node
    pub fn make_phi(&mut self, region: NodeId, inputs: Vec<NodeId>, ty: IrType) -> NodeId {
        let id = self.next_id();
        let node = SeaNode {
            id,
            opcode: SeaOpcode::Phi,
            inputs: vec![region].into_iter().chain(inputs.clone()).collect(),
            output: None,
            properties: NodeProperties::default(),
        };
        let phi_id = self.add_node(node);

        self.connect(region, phi_id);
        for input in inputs {
            self.connect(input, phi_id);
        }

        phi_id
    }

    /// Create an addition node
    pub fn make_add(&mut self, lhs: NodeId, rhs: NodeId) -> NodeId {
        let id = self.next_id();
        let node = SeaNode {
            id,
            opcode: SeaOpcode::Add,
            inputs: vec![lhs, rhs],
            output: None,
            properties: NodeProperties::default(),
        };
        let add_id = self.add_node(node);

        self.connect(lhs, add_id);
        self.connect(rhs, add_id);

        add_id
    }

    /// Create a load node
    pub fn make_load(&mut self, address: NodeId, ty: IrType) -> NodeId {
        let id = self.next_id();
        let node = SeaNode {
            id,
            opcode: SeaOpcode::Load,
            inputs: vec![address],
            output: None,
            properties: NodeProperties {
                is_memory: true,
                is_schedulable: true,
            },
        };
        let load_id = self.add_node(node);

        self.connect(address, load_id);

        load_id
    }

    /// Create a store node
    pub fn make_store(&mut self, address: NodeId, value: NodeId) -> NodeId {
        let id = self.next_id();
        let node = SeaNode {
            id,
            opcode: SeaOpcode::Store,
            inputs: vec![address, value],
            output: None,
            properties: NodeProperties {
                is_memory: true,
                is_schedulable: true,
            },
        };
        let store_id = self.add_node(node);

        self.connect(address, store_id);
        self.connect(value, store_id);

        store_id
    }

    /// Create a comparison node
    pub fn make_cmp(&mut self, lhs: NodeId, rhs: NodeId) -> NodeId {
        let id = self.next_id();
        let node = SeaNode {
            id,
            opcode: SeaOpcode::Cmp,
            inputs: vec![lhs, rhs],
            output: None,
            properties: NodeProperties::default(),
        };
        let cmp_id = self.add_node(node);

        self.connect(lhs, cmp_id);
        self.connect(rhs, cmp_id);

        cmp_id
    }

    /// Create a conditional branch
    pub fn make_if(&mut self, cond: NodeId, then: NodeId, else_: NodeId) -> NodeId {
        let id = self.next_id();
        let node = SeaNode {
            id,
            opcode: SeaOpcode::If,
            inputs: vec![cond, then, else_],
            output: None,
            properties: NodeProperties {
                is_control: true,
                is_schedulable: true,
            },
        };
        let if_id = self.add_node(node);

        self.connect(cond, if_id);
        self.connect(then, if_id);
        self.connect(else_, if_id);

        if_id
    }

    /// Create a return node
    pub fn make_return(&mut self, value: Option<NodeId>) -> NodeId {
        let id = self.next_id();
        let mut inputs = Vec::new();
        if let Some(v) = value {
            inputs.push(v);
            self.connect(v, id);
        }

        let node = SeaNode {
            id,
            opcode: SeaOpcode::Return,
            inputs,
            output: Some(self.end),
            properties: NodeProperties {
                is_control: true,
                is_pinned: true,
                is_schedulable: true,
            },
        };
        let ret_id = self.add_node(node);

        self.connect(ret_id, self.end);

        ret_id
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, id: NodeId) {
        // Disconnect all inputs
        if let Some(inputs) = self.inputs.get(&id) {
            for &input in inputs {
                if let Some(outputs) = self.outputs.get_mut(&input) {
                    outputs.retain(|&n| n != id);
                }
            }
        }

        // Disconnect all outputs
        if let Some(outputs) = self.outputs.get(&id) {
            for &output in outputs {
                if let Some(inputs) = self.inputs.get_mut(&output) {
                    inputs.retain(|&n| n != id);
                }
            }
        }

        self.nodes.remove(&id);
        self.inputs.remove(&id);
        self.outputs.remove(&id);
    }

    /// Substitute one node for another
    pub fn substitute(&mut self, old: NodeId, new: NodeId) {
        // Update all outputs of old to point to new
        if let Some(outputs) = self.outputs.get(&old).cloned() {
            for &output in &outputs {
                if let Some(inputs) = self.inputs.get_mut(&output) {
                    for input in inputs.iter_mut() {
                        if *input == old {
                            *input = new;
                        }
                    }
                }

                if let Some(outputs_new) = self.outputs.get_mut(&new) {
                    if !outputs_new.contains(&output) {
                        outputs_new.push(output);
                    }
                }
            }
        }

        // Remove old from the graph
        self.remove_node(old);
    }

    /// Schedule nodes for execution (topological order)
    pub fn schedule(&self) -> Vec<NodeId> {
        let mut in_degree = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        // Compute in-degrees
        for (&id, _) in &self.nodes {
            let deg = self.inputs.get(&id).map(|v| v.len()).unwrap_or(0);
            in_degree.insert(id, deg);

            if deg == 0 {
                queue.push_back(id);
            }
        }

        // Topological sort (Kahn's algorithm)
        while let Some(node) = queue.pop_front() {
            result.push(node);

            if let Some(outputs) = self.outputs.get(&node) {
                for &output in outputs {
                    if let Some(deg) = in_degree.get_mut(&output) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(output);
                        }
                    }
                }
            }
        }

        result
    }

    /// Find cycles in the graph
    pub fn find_cycles(&self) -> Vec<Vec<NodeId>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut cycles = Vec::new();
        let mut path = Vec::new();

        fn dfs(
            node: NodeId,
            graph: &SeaOfNodes,
            visited: &mut HashSet<NodeId>,
            rec_stack: &mut HashSet<NodeId>,
            path: &mut Vec<NodeId>,
            cycles: &mut Vec<Vec<NodeId>>,
        ) {
            visited.insert(node);
            rec_stack.insert(node);
            path.push(node);

            for &output in graph.get_outputs(node) {
                if !visited.contains(&output) {
                    dfs(output, graph, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(&output) {
                    // Found cycle
                    if let Some(start) = path.iter().position(|&n| n == output) {
                        cycles.push(path[start..].to_vec());
                    }
                }
            }

            path.pop();
            rec_stack.remove(&node);
        }

        for (&id, _) in &self.nodes {
            if !visited.contains(&id) {
                dfs(id, self, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    /// Compute dominators
    pub fn dominators(&self, root: NodeId) -> HashMap<NodeId, HashSet<NodeId>> {
        let mut doms = HashMap::new();
        doms.insert(root, vec![root].into_iter().collect());

        let mut changed = true;
        while changed {
            changed = false;

            for &node in self.nodes.keys() {
                if node == root {
                    continue;
                }

                let preds: Vec<_> = self.get_inputs(node).to_vec();
                if preds.is_empty() {
                    continue;
                }

                let mut new_dom = HashSet::new();
                new_dom.insert(node);

                for &pred in &preds {
                    if let Some(pred_dom) = doms.get(&pred) {
                        if new_dom.is_empty() {
                            new_dom = pred_dom.clone();
                        } else {
                            new_dom = new_dom.intersection(pred_dom).cloned().collect();
                        }
                    }
                }

                if doms.get(&node) != Some(&new_dom) {
                    doms.insert(node, new_dom);
                    changed = true;
                }
            }
        }

        doms
    }
}

impl Default for SeaOfNodes {
    fn default() -> Self {
        Self::new()
    }
}

/// Constant value representation
#[derive(Debug, Clone)]
pub enum ConstValue {
    Int(i64, IrType),
    Float(f64, IrType),
    Bool(bool),
    Null,
    Ptr(u64),
}

/// Graphviz dot output for visualization
impl SeaOfNodes {
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph SeaOfNodes {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box];\n\n");

        for (&id, node) in &self.nodes {
            let label = format!("{:?}: {:?}", id, node.opcode);
            dot.push_str(&format!("  N{:?} [label=\"{}\"];\n", id.0, label));
        }

        dot.push_str("\n");

        for (&id, outputs) in &self.outputs {
            for &output in outputs {
                dot.push_str(&format!("  N{:?} -> N{:?};\n", id.0, output.0));
            }
        }

        dot.push_str("}\n");
        dot
    }
}

/// Simple scheduling algorithm
pub struct ListScheduler {
    pub latency_model: HashMap<SeaOpcode, usize>,
}

impl ListScheduler {
    pub fn new() -> Self {
        let mut latency_model = HashMap::new();
        latency_model.insert(SeaOpcode::Add, 1);
        latency_model.insert(SeaOpcode::Sub, 1);
        latency_model.insert(SeaOpcode::Mul, 3);
        latency_model.insert(SeaOpcode::Div, 20);
        latency_model.insert(SeaOpcode::Load, 4);
        latency_model.insert(SeaOpcode::Store, 1);

        Self { latency_model }
    }

    /// Schedule nodes using list scheduling
    pub fn schedule(&self, graph: &SeaOfNodes) -> Vec<(NodeId, usize)> {
        let schedule_order = graph.schedule();
        let mut result = Vec::new();
        let mut ready: VecDeque<(NodeId, usize)> = VecDeque::new();
        let mut scheduled = HashSet::new();
        let mut next_cycle = 0;

        // Find start node
        let start = graph.start;
        ready.push_back((start, 0));

        while !ready.is_empty() || !scheduled.contains(&start) {
            // Remove unschedulable nodes
            while let Some((node, cycle)) = ready.pop_front() {
                if scheduled.contains(&node) {
                    continue;
                }

                // Check if all inputs are scheduled
                let inputs = graph.get_inputs(node);
                let all_scheduled = inputs.iter().all(|&i| scheduled.contains(&i));

                if all_scheduled {
                    result.push((node, cycle));
                    scheduled.insert(node);

                    // Update ready list with successors
                    for &output in graph.get_outputs(node) {
                        if !scheduled.contains(&output) {
                            let latency = self.latency_model
                                .get(&graph.get_node(node).unwrap().opcode)
                                .copied()
                                .unwrap_or(1);
                            ready.push_back((output, cycle + latency));
                        }
                    }
                } else {
                    // Put back in queue
                    ready.push_back((node, cycle));
                }
            }

            // If nothing scheduled this cycle, advance
            if result.len() == scheduled.len().saturating_sub(1) {
                next_cycle += 1;
                if next_cycle > 10000 {
                    break; // Safety limit
                }
            }
        }

        result
    }
}

impl Default for ListScheduler {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let mut graph = SeaOfNodes::new();

        let a = graph.make_const(ConstValue::Int(42, IrType::Int32));
        let b = graph.make_const(ConstValue::Int(1, IrType::Int32));
        let sum = graph.make_add(a, b);

        assert!(graph.get_node(a).is_some());
        assert!(graph.get_node(b).is_some());
        assert!(graph.get_node(sum).is_some());
    }

    #[test]
    fn test_substitution() {
        let mut graph = SeaOfNodes::new();

        let a = graph.make_const(ConstValue::Int(10, IrType::Int32));
        let b = graph.make_const(ConstValue::Int(20, IrType::Int32));
        let c = graph.make_add(a, b);
        let d = graph.make_add(a, b);

        graph.substitute(d, c);

        assert!(graph.get_node(d).is_none());
    }

    #[test]
    fn test_scheduling() {
        let graph = SeaOfNodes::new();
        let schedule = graph.schedule();

        // Start should be first
        assert_eq!(schedule.first(), Some(&graph.start));
    }

    #[test]
    fn test_dot_output() {
        let graph = SeaOfNodes::new();
        let dot = graph.to_dot();

        assert!(dot.contains("digraph"));
        assert!(dot.contains("N0"));
        assert!(dot.contains("->"));
    }
}
