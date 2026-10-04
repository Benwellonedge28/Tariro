// Call Graph Analysis

use super::*;

/// Call graph analysis
pub struct CallGraphAnalysis {
    pub context_sensitive: bool,
}

impl Default for CallGraphAnalysis {
    fn default() -> Self {
        Self {
            context_sensitive: false,
        }
    }
}

impl CallGraphAnalysis {
    pub fn new(context_sensitive: bool) -> Self {
        Self { context_sensitive }
    }

    pub fn analyze(&self, module: &Module) -> CallGraph {
        let mut graph = CallGraph::new();

        // Add nodes
        for func in &module.functions {
            graph.add_node(func.id, func.name.clone());
        }

        // Build edges
        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    if let Operation::Call(callee, _) = &inst.op {
                        graph.add_edge(func.id, *callee);
                    } else if let Operation::CallIndirect(ptr, _) = &inst.op {
                        // Indirect call - add edges to all functions
                        for func_id in graph.nodes.keys() {
                            graph.add_edge(func.id, *func_id);
                        }
                    }
                }
            }
        }

        // Add edges for function pointers
        self.add_function_pointer_edges(module, &mut graph);

        graph
    }

    fn add_function_pointer_edges(&self, module: &Module, graph: &mut CallGraph) {
        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    // Check for function pointer assignments
                    if matches!(inst.op, Operation::Bitcast(_, _)) {
                        // Function pointer - connect to all potential targets
                        for target_id in graph.nodes.keys() {
                            graph.add_edge(func.id, *target_id);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CallGraph {
    pub nodes: HashMap<FunctionId, CallGraphNode>,
    pub edges: HashMap<FunctionId, Vec<FunctionId>>,
    pub reverse_edges: HashMap<FunctionId, Vec<FunctionId>>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, id: FunctionId, name: String) {
        self.nodes.entry(id).or_insert_with(|| CallGraphNode {
            id,
            name,
            func_ptr_targets: Vec::new(),
        });
        self.edges.entry(id).or_insert_with(Vec::new);
        self.reverse_edges.entry(id).or_insert_with(Vec::new);
    }

    pub fn add_edge(&mut self, caller: FunctionId, callee: FunctionId) {
        self.edges.entry(caller).or_default().push(callee);
        self.reverse_edges.entry(callee).or_default().push(caller);
    }

    pub fn add_func_ptr_edge(&mut self, source: FunctionId, target: FunctionId) {
        if let Some(node) = self.nodes.get_mut(&source) {
            if !node.func_ptr_targets.contains(&target) {
                node.func_ptr_targets.push(target);
            }
        }
        self.add_edge(source, target);
    }

    /// Get all functions called by this function
    pub fn callees(&self, func: FunctionId) -> Vec<FunctionId> {
        self.edges.get(&func).cloned().unwrap_or_default()
    }

    /// Get all functions that call this function
    pub fn callers(&self, func: FunctionId) -> Vec<FunctionId> {
        self.reverse_edges.get(&func).cloned().unwrap_or_default()
    }

    /// Check if function is a potential callee (direct or via function pointer)
    pub fn is_potential_callee(&self, caller: FunctionId, callee: FunctionId) -> bool {
        if let Some(node) = self.nodes.get(&caller) {
            node.func_ptr_targets.contains(&callee)
        } else {
            false
        }
    }

    /// Compute strongly connected components
    pub fn sccs(&self) -> Vec<Vec<FunctionId>> {
        let mut sccs = Vec::new();
        let mut index = 0;
        let mut stack = Vec::new();
        let mut indices = HashMap::new();
        let mut lowlinks = HashMap::new();
        let mut on_stack = HashSet::new();

        fn strongconnect(
            graph: &CallGraph,
            v: FunctionId,
            &mut index: &mut usize,
            stack: &mut Vec<FunctionId>,
            indices: &mut HashMap<FunctionId, usize>,
            lowlinks: &mut HashMap<FunctionId, usize>,
            on_stack: &mut HashSet<FunctionId>,
            sccs: &mut Vec<Vec<FunctionId>>,
        ) {
            indices.insert(v, index);
            lowlinks.insert(v, index);
            index += 1;
            stack.push(v);
            on_stack.insert(v);

            for &w in graph.callees(v) {
                if !indices.contains_key(&w) {
                    strongconnect(graph, w, &mut index, stack, indices, lowlinks, on_stack, sccs);
                    let w_low = *lowlinks.get(&w).unwrap();
                    *lowlinks.entry(v).or_insert(0) = w_low.min(*lowlinks.get(&v).unwrap());
                } else if on_stack.contains(&w) {
                    let w_idx = *indices.get(&w).unwrap();
                    *lowlinks.entry(v).or_insert(0) = w_idx.min(*lowlinks.get(&v).unwrap());
                }
            }

            if *lowlinks.get(&v).unwrap() == *indices.get(&v).unwrap() {
                let mut scc = Vec::new();
                loop {
                    let w = stack.pop().unwrap();
                    on_stack.remove(&w);
                    scc.push(w);
                    if w == v {
                        break;
                    }
                }
                sccs.push(scc);
            }
        }

        for &v in self.nodes.keys() {
            if !indices.contains_key(&v) {
                strongconnect(self, v, &mut index, &mut stack, &mut indices, &mut lowlinks, &mut on_stack, &mut sccs);
            }
        }

        sccs
    }

    /// Compute postorder of call graph
    pub fn postorder(&self) -> Vec<FunctionId> {
        let mut visited = HashSet::new();
        let mut order = Vec::new();

        fn dfs(
            graph: &CallGraph,
            node: FunctionId,
            visited: &mut HashSet<FunctionId>,
            order: &mut Vec<FunctionId>,
        ) {
            if visited.contains(&node) {
                return;
            }
            visited.insert(node);

            for &callee in graph.callees(node) {
                dfs(graph, callee, visited, order);
            }

            order.push(node);
        }

        for &node in self.nodes.keys() {
            dfs(self, node, &mut visited, &mut order);
        }

        order
    }

    /// Compute reverse postorder (topological order for call graph)
    pub fn reverse_postorder(&self) -> Vec<FunctionId> {
        let mut order = self.postorder();
        order.reverse();
        order
    }
}

#[derive(Debug, Clone)]
pub struct CallGraphNode {
    pub id: FunctionId,
    pub name: String,
    pub func_ptr_targets: Vec<FunctionId>,
}

/// Call graph edge iterator
pub struct CallGraphEdges<'a> {
    graph: &'a CallGraph,
    current: FunctionId,
    edge_idx: usize,
}

impl<'a> Iterator for CallGraphEdges<'a> {
    type Item = (FunctionId, FunctionId);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(edges) = self.graph.edges.get(&self.current) {
                if self.edge_idx < edges.len() {
                    let callee = edges[self.edge_idx];
                    self.edge_idx += 1;
                    return Some((self.current, callee));
                }
            }

            // Move to next node
            self.edge_idx = 0;
            let keys: Vec<_> = self.graph.edges.keys().collect();
            let mut found = false;
            for &key in &keys {
                if *key > self.current {
                    self.current = key;
                    found = true;
                    break;
                }
            }

            if !found {
                return None;
            }
        }
    }
}

impl<'a> IntoIterator for &'a CallGraph {
    type Item = (FunctionId, FunctionId);
    type IntoIter = CallGraphEdges<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CallGraphEdges {
            graph: self,
            current: FunctionId(0),
            edge_idx: 0,
        }
    }
}

use std::collections::{HashMap, HashSet};
