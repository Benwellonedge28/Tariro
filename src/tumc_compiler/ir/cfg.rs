// Control Flow Graph Implementation

use super::*;

/// Control Flow Graph representation
#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    pub nodes: HashMap<BasicBlockId, CfgNode>,
    pub entry: BasicBlockId,
    pub exit: Option<BasicBlockId>,
}

#[derive(Debug, Clone)]
pub struct CfgNode {
    pub block: BasicBlockId,
    pub predecessors: Vec<BasicBlockId>,
    pub successors: Vec<BasicBlockId>,
    pub instructions: Vec<InstId>,
    pub terminator: Terminator,
    pub is_loop_header: bool,
    pub loop_depth: usize,
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            entry: BasicBlockId(0),
            exit: None,
        }
    }

    pub fn from_function(function: &Function) -> Self {
        let mut cfg = Self::new();
        cfg.entry = function.entry_block();

        // Build nodes from basic blocks
        for block in &function.body {
            let successors = block.terminator.successors();
            let cfg_node = CfgNode {
                block: block.id,
                predecessors: Vec::new(),
                successors: successors.clone(),
                instructions: block.instructions.iter().map(|i| i.id).collect(),
                terminator: block.terminator.clone(),
                is_loop_header: false,
                loop_depth: 0,
            };
            cfg.nodes.insert(block.id, cfg_node);
        }

        // Build predecessor lists
        for block in &function.body {
            for &succ in block.terminator.successors() {
                if let Some(node) = cfg.nodes.get_mut(&succ) {
                    node.predecessors.push(block.id);
                }
            }
        }

        // Find exit block
        for block in &function.body {
            if matches!(block.terminator, Terminator::Ret(_)) {
                cfg.exit = Some(block.id);
                break;
            }
        }

        cfg
    }

    pub fn add_block(&mut self, block: BasicBlockId, predecessors: Vec<BasicBlockId>) {
        let node = CfgNode {
            block,
            predecessors,
            successors: Vec::new(),
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
            is_loop_header: false,
            loop_depth: 0,
        };
        self.nodes.insert(block, node);
    }

    pub fn add_edge(&mut self, from: BasicBlockId, to: BasicBlockId) {
        if let Some(node) = self.nodes.get_mut(&from) {
            if !node.successors.contains(&to) {
                node.successors.push(to);
            }
        }
        if let Some(node) = self.nodes.get_mut(&to) {
            if !node.predecessors.contains(&from) {
                node.predecessors.push(from);
            }
        }
    }

    pub fn remove_edge(&mut self, from: BasicBlockId, to: BasicBlockId) {
        if let Some(node) = self.nodes.get_mut(&from) {
            node.successors.retain(|&b| b != to);
        }
        if let Some(node) = self.nodes.get_mut(&to) {
            node.predecessors.retain(|&b| b != from);
        }
    }

    pub fn get_node(&self, block: BasicBlockId) -> Option<&CfgNode> {
        self.nodes.get(&block)
    }

    pub fn get_node_mut(&mut self, block: BasicBlockId) -> Option<&mut CfgNode> {
        self.nodes.get_mut(&block)
    }

    pub fn predecessors(&self, block: BasicBlockId) -> &[BasicBlockId] {
        self.nodes.get(&block).map(|n| n.predecessors.as_slice()).unwrap_or(&[])
    }

    pub fn successors(&self, block: BasicBlockId) -> &[BasicBlockId] {
        self.nodes.get(&block).map(|n| n.successors.as_slice()).unwrap_or(&[])
    }

    pub fn is_dominated_by(&self, block: BasicBlockId, dominator: BasicBlockId, dom_tree: &DominatorTree) -> bool {
        dom_tree.dominates(dominator, block)
    }

    pub fn natural_loops(&self) -> Vec<NaturalLoop> {
        let mut loops = Vec::new();

        for (&header, node) in &self.nodes {
            // Check if any predecessor is a back edge
            let back_edges: Vec<_> = node.predecessors.iter()
                .filter(|&&pred| self.is_back_edge(header, pred))
                .collect();

            if !back_edges.is_empty() {
                let mut loop_blocks = vec![header];
                let mut worklist: Vec<_> = back_edges.iter().map(|&&p| p).collect();

                while let Some(block) = worklist.pop() {
                    if !loop_blocks.contains(&block) {
                        loop_blocks.push(block);
                        if let Some(node) = self.nodes.get(&block) {
                            for &pred in &node.predecessors {
                                if !loop_blocks.contains(&pred) {
                                    worklist.push(pred);
                                }
                            }
                        }
                    }
                }

                loops.push(NaturalLoop {
                    header,
                    blocks: loop_blocks,
                    back_edges: back_edges.into_iter().copied().collect(),
                });
            }
        }

        loops
    }

    fn is_back_edge(&self, header: BasicBlockId, pred: BasicBlockId) -> bool {
        // Simple dominance check
        // A back edge goes from a block to a header that dominates it
        self.nodes.contains_key(&header) && self.nodes.contains_key(&pred)
    }

    pub fn topological_sort(&self) -> Vec<BasicBlockId> {
        let mut visited = HashMap::new();
        let mut order = Vec::new();

        fn visit(
            block: BasicBlockId,
            cfg: &ControlFlowGraph,
            visited: &mut HashMap<BasicBlockId, bool>,
            order: &mut Vec<BasicBlockId>,
        ) {
            if visited.get(&block) == Some(&true) {
                return;
            }
            if visited.get(&block) == Some(&false) {
                // Cycle detected
                return;
            }

            visited.insert(block, false);
            for &succ in cfg.successors(block) {
                visit(succ, cfg, visited, order);
            }
            visited.insert(block, true);
            order.push(block);
        }

        visit(self.entry, self, &mut visited, &mut order);
        order.reverse();
        order
    }

    pub fn reverse_post_order(&self) -> Vec<BasicBlockId> {
        let mut visited = HashMap::new();
        let mut order = Vec::new();

        fn visit(
            block: BasicBlockId,
            cfg: &ControlFlowGraph,
            visited: &mut HashMap<BasicBlockId, bool>,
            order: &mut Vec<BasicBlockId>,
        ) {
            if visited.get(&block) == Some(&true) {
                return;
            }
            visited.insert(block, true);
            for &succ in cfg.successors(block) {
                visit(succ, cfg, visited, order);
            }
            order.push(block);
        }

        visit(self.entry, self, &mut visited, &mut order);
        order
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = HashMap::new();
        let mut rec_stack = HashMap::new();

        fn dfs(
            block: BasicBlockId,
            cfg: &ControlFlowGraph,
            visited: &mut HashMap<BasicBlockId, bool>,
            rec_stack: &mut HashMap<BasicBlockId, bool>,
        ) -> bool {
            visited.insert(block, true);
            rec_stack.insert(block, true);

            for &succ in cfg.successors(block) {
                if visited.get(&succ) != Some(&true) {
                    if dfs(succ, cfg, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.get(&succ) == Some(&true) {
                    return true;
                }
            }

            rec_stack.insert(block, false);
            false
        }

        for (&block, _) in &self.nodes {
            if visited.get(&block) != Some(&true) {
                if dfs(block, self, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    pub fn immediate_dominators(&self) -> HashMap<BasicBlockId, BasicBlockId> {
        let mut idom = HashMap::new();
        idom.insert(self.entry, self.entry);

        // Simple iterative algorithm
        let mut changed = true;
        while changed {
            changed = false;
            for (&block, _) in &self.nodes {
                if block == self.entry {
                    continue;
                }

                let preds: Vec<_> = self.predecessors(block).to_vec();
                if preds.is_empty() {
                    continue;
                }

                // Find first processed predecessor
                let mut new_idom = None;
                for &pred in &preds {
                    if idom.contains_key(&pred) {
                        new_idom = Some(pred);
                        break;
                    }
                }

                if let Some(mut candidate) = new_idom {
                    // Intersect all processed predecessors
                    for &pred in &preds {
                        if idom.contains_key(&pred) && pred != candidate {
                            candidate = self.intersect(&idom, candidate, pred);
                        }
                    }

                    if idom.get(&block) != Some(&candidate) {
                        idom.insert(block, candidate);
                        changed = true;
                    }
                }
            }
        }

        idom
    }

    fn intersect(&self, idom: &HashMap<BasicBlockId, BasicBlockId>, a: BasicBlockId, b: BasicBlockId) -> BasicBlockId {
        let mut finger1 = a;
        let mut finger2 = b;

        while finger1 != finger2 {
            while self.is_strictly_dominated_by(finger1, finger2, idom) {
                finger1 = *idom.get(&finger1).unwrap_or(&finger1);
            }
            while self.is_strictly_dominated_by(finger2, finger1, idom) {
                finger2 = *idom.get(&finger2).unwrap_or(&finger2);
            }
        }

        finger1
    }

    fn is_strictly_dominated_by(&self, block: BasicBlockId, dominator: BasicBlockId, idom: &HashMap<BasicBlockId, BasicBlockId>) -> bool {
        let mut current = block;
        while let Some(&idom_block) = idom.get(&current) {
            if idom_block == dominator {
                return true;
            }
            if idom_block == current {
                break;
            }
            current = idom_block;
        }
        false
    }

    pub fn dominance_frontier(&self, block: BasicBlockId, idom: &HashMap<BasicBlockId, BasicBlockId>) -> Vec<BasicBlockId> {
        let mut frontier = Vec::new();

        for &succ in self.successors(block) {
            // If block doesn't strictly dominate succ, succ is in frontier
            if let Some(&idom_succ) = idom.get(&succ) {
                if idom_succ != block {
                    if !frontier.contains(&succ) {
                        frontier.push(succ);
                    }
                }
            }
        }

        // For each predecessor of successors
        for &succ in self.successors(block) {
            for &pred in self.predecessors(succ) {
                if pred != block && !self.is_strictly_dominated_by(pred, block, idom) {
                    if !frontier.contains(&pred) {
                        frontier.push(pred);
                    }
                }
            }
        }

        frontier
    }

    pub fn iterated_dominance_frontier(&self, mut set: Vec<BasicBlockId>, idom: &HashMap<BasicBlockId, BasicBlockId>) -> Vec<BasicBlockId> {
        let mut result = set.clone();

        loop {
            let mut new = Vec::new();
            for &block in &result {
                for &df in self.dominance_frontier(block, idom) {
                    if !result.contains(&df) {
                        new.push(df);
                    }
                }
            }

            if new.is_empty() {
                break;
            }

            result.extend(new);
        }

        result
    }
}

/// Natural loop representation
#[derive(Debug, Clone)]
pub struct NaturalLoop {
    pub header: BasicBlockId,
    pub blocks: Vec<BasicBlockId>,
    pub back_edges: Vec<BasicBlockId>,
}

impl NaturalLoop {
    pub fn is_header(&self, block: BasicBlockId) -> bool {
        self.header == block
    }

    pub fn contains(&self, block: BasicBlockId) -> bool {
        self.blocks.contains(&block)
    }

    pub fn preheader(&self, cfg: &ControlFlowGraph) -> Option<BasicBlockId> {
        // Find a predecessor of the header that's not in the loop
        for &pred in cfg.predecessors(self.header) {
            if !self.contains(pred) {
                return Some(pred);
            }
        }
        None
    }
}

/// Block ordering utilities
#[derive(Debug, Clone)]
pub enum BlockOrder {
    Preorder,
    Postorder,
    ReversePostorder,
    Topological,
}

impl ControlFlowGraph {
    pub fn blocks_in_order(&self, order: BlockOrder) -> Vec<BasicBlockId> {
        match order {
            BlockOrder::Preorder => self.preorder(),
            BlockOrder::Postorder => self.postorder(),
            BlockOrder::ReversePostorder => self.reverse_post_order(),
            BlockOrder::Topological => self.topological_sort(),
        }
    }

    fn preorder(&self) -> Vec<BasicBlockId> {
        let mut visited = HashMap::new();
        let mut order = Vec::new();

        fn visit(
            block: BasicBlockId,
            cfg: &ControlFlowGraph,
            visited: &mut HashMap<BasicBlockId, bool>,
            order: &mut Vec<BasicBlockId>,
        ) {
            if visited.get(&block) == Some(&true) {
                return;
            }
            visited.insert(block, true);
            order.push(block);
            for &succ in cfg.successors(block) {
                visit(succ, cfg, visited, order);
            }
        }

        visit(self.entry, self, &mut visited, &mut order);
        order
    }

    fn postorder(&self) -> Vec<BasicBlockId> {
        let mut visited = HashMap::new();
        let mut order = Vec::new();

        fn visit(
            block: BasicBlockId,
            cfg: &ControlFlowGraph,
            visited: &mut HashMap<BasicBlockId, bool>,
            order: &mut Vec<BasicBlockId>,
        ) {
            if visited.get(&block) == Some(&true) {
                return;
            }
            visited.insert(block, true);
            for &succ in cfg.successors(block) {
                visit(succ, cfg, visited, order);
            }
            order.push(block);
        }

        visit(self.entry, self, &mut visited, &mut order);
        order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg_construction() {
        let mut cfg = ControlFlowGraph::new();
        cfg.add_block(BasicBlockId(0), vec![]);
        cfg.add_block(BasicBlockId(1), vec![BasicBlockId(0)]);
        cfg.add_block(BasicBlockId(2), vec![BasicBlockId(1)]);

        cfg.add_edge(BasicBlockId(0), BasicBlockId(1));
        cfg.add_edge(BasicBlockId(1), BasicBlockId(2));

        assert_eq!(cfg.successors(BasicBlockId(0)), &[BasicBlockId(1)]);
        assert_eq!(cfg.predecessors(BasicBlockId(1)), &[BasicBlockId(0)]);
    }

    #[test]
    fn test_cycle_detection() {
        let mut cfg = ControlFlowGraph::new();
        cfg.add_block(BasicBlockId(0), vec![]);
        cfg.add_block(BasicBlockId(1), vec![BasicBlockId(0), BasicBlockId(2)]);
        cfg.add_block(BasicBlockId(2), vec![BasicBlockId(1)]);

        cfg.add_edge(BasicBlockId(0), BasicBlockId(1));
        cfg.add_edge(BasicBlockId(1), BasicBlockId(2));
        cfg.add_edge(BasicBlockId(2), BasicBlockId(1)); // Back edge

        assert!(cfg.has_cycle());
    }

    #[test]
    fn test_topological_sort() {
        let mut cfg = ControlFlowGraph::new();
        cfg.add_block(BasicBlockId(0), vec![]);
        cfg.add_block(BasicBlockId(1), vec![BasicBlockId(0)]);
        cfg.add_block(BasicBlockId(2), vec![BasicBlockId(1)]);

        cfg.add_edge(BasicBlockId(0), BasicBlockId(1));
        cfg.add_edge(BasicBlockId(1), BasicBlockId(2));

        let order = cfg.topological_sort();
        assert_eq!(order, vec![BasicBlockId(0), BasicBlockId(1), BasicBlockId(2)]);
    }
}
