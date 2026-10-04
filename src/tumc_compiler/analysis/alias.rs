// Alias Analysis

use super::*;

/// Alias analysis implementation
pub struct AliasAnalysis {
    pub level: AliasAnalysisLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AliasAnalysisLevel {
    /// No alias analysis
    None,
    /// Type-based alias analysis (TBAA)
    TypeBased,
    /// Basic flow-insensitive
    FlowInsensitive,
    /// Flow-sensitive
    FlowSensitive,
    /// Context-sensitive
    ContextSensitive,
}

impl Default for AliasAnalysis {
    fn default() -> Self {
        Self {
            level: AliasAnalysisLevel::TypeBased,
        }
    }
}

impl AliasAnalysis {
    pub fn new(level: AliasAnalysisLevel) -> Self {
        Self { level }
    }

    pub fn analyze(&self, module: &Module) -> AliasResult {
        match self.level {
            AliasAnalysisLevel::None => AliasResult::no_analysis(),
            AliasAnalysisLevel::TypeBased => self.type_based_analysis(module),
            AliasAnalysisLevel::FlowInsensitive => self.flow_insensitive_analysis(module),
            AliasAnalysisLevel::FlowSensitive => self.flow_sensitive_analysis(module),
            AliasAnalysisLevel::ContextSensitive => self.context_sensitive_analysis(module),
        }
    }

    /// Type-based alias analysis (TBAA)
    fn type_based_analysis(&self, module: &Module) -> AliasResult {
        let mut result = AliasResult::new();

        // Build type hierarchy
        let mut type_hierarchy = TypeHierarchy::new();
        type_hierarchy.build_from_module(module);
        result.type_hierarchy = Some(type_hierarchy);

        result
    }

    fn flow_insensitive_analysis(&self, module: &Module) -> AliasResult {
        let mut result = AliasResult::new();

        // Simplified: use points-to analysis
        let pts_analysis = PointsToAnalysis::new(PointsToLevel::Unification);
        let pts_result = pts_analysis.analyze(module);
        result.points_to = Some(pts_result);

        result
    }

    fn flow_sensitive_analysis(&self, module: &Module) -> AliasResult {
        let mut result = AliasResult::new();

        for func in &module.functions {
            let mut func_result = FlowSensitiveAliasResult::new();

            let cfg = ControlFlowGraph::from_function(func);

            // Iterate until fixed point
            let mut changed = true;
            while changed {
                changed = false;

                for block in &func.body {
                    for inst in &block.instructions {
                        self.update_alias_info(inst, &mut func_result, &cfg);
                    }
                }
            }

            result.function_results.insert(func.id, func_result);
        }

        result
    }

    fn context_sensitive_analysis(&self, module: &Module) -> AliasResult {
        // Context-sensitive: track calling contexts separately
        self.flow_sensitive_analysis(module)
    }

    fn update_alias_info(
        &self,
        inst: &Instruction,
        result: &mut FlowSensitiveAliasResult,
        cfg: &ControlFlowGraph,
    ) {
        match &inst.op {
            Operation::Load(_, ptr, _, _) => {
                // Load may create alias with previous loads from same location
                result.may_alias.insert((inst.id, *ptr));
            }
            Operation::Store(_, ptr, val, _) => {
                // Store may create alias
                result.may_alias.insert((*ptr, *val));
            }
            Operation::Bitcast(val, _) => {
                // Bitcast preserves aliasing
                result.may_alias.insert((inst.id, *val));
            }
            _ => {}
        }
    }

    /// Check alias relationship between two memory locations
    pub fn alias(&self, result: &AliasResult, a: MemoryLocation, b: MemoryLocation) -> AliasResultType {
        // Use points-to information if available
        if let Some(pts) = &result.points_to {
            let pts_a = pts.points_to_set(a.value);
            let pts_b = pts.points_to_set(b.value);

            let mut intersect = false;
            for loc_a in pts_a {
                for loc_b in pts_b {
                    if loc_a == loc_b {
                        intersect = true;
                        break;
                    }
                }
            }

            if intersect {
                return AliasResultType::MayAlias;
            }
        }

        AliasResultType::NoAlias
    }
}

#[derive(Debug, Clone)]
pub struct AliasResult {
    pub points_to: Option<PointsToResult>,
    pub type_hierarchy: Option<TypeHierarchy>,
    pub function_results: HashMap<FunctionId, FlowSensitiveAliasResult>,
}

impl AliasResult {
    pub fn new() -> Self {
        Self {
            points_to: None,
            type_hierarchy: None,
            function_results: HashMap::new(),
        }
    }

    pub fn no_analysis() -> Self {
        Self::new()
    }

    pub fn may_alias(&self, a: ValueId, b: ValueId) -> bool {
        if let Some(pts) = &self.points_to {
            pts.may_alias(a, b)
        } else {
            // Conservative: assume may alias
            true
        }
    }
}

impl Default for AliasResult {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct FlowSensitiveAliasResult {
    pub may_alias: HashSet<(ValueId, ValueId)>,
    pub must_alias: HashSet<(ValueId, ValueId)>,
    pub no_alias: HashSet<(ValueId, ValueId)>,
}

impl FlowSensitiveAliasResult {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AliasResultType {
    NoAlias,
    MayAlias,
    MustAlias,
    PartialAlias,
}

#[derive(Debug, Clone)]
pub struct MemoryLocation {
    pub value: ValueId,
    pub offset: Option<i64>,
    pub size: Option<usize>,
}

impl MemoryLocation {
    pub fn new(value: ValueId) -> Self {
        Self {
            value,
            offset: None,
            size: None,
        }
    }

    pub fn with_offset(value: ValueId, offset: i64) -> Self {
        Self {
            value,
            offset: Some(offset),
            size: None,
        }
    }
}

/// Type hierarchy for TBAA
#[derive(Debug, Clone)]
pub struct TypeHierarchy {
    pub types: HashMap<String, TypeInfo>,
    pub hierarchy: HashMap<String, Vec<String>>,
}

impl TypeHierarchy {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            hierarchy: HashMap::new(),
        }
    }

    pub fn build_from_module(&mut self, module: &Module) {
        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    if let Operation::Alloca(ty, _) = &inst.op {
                        let type_name = format!("{:?}", ty);
                        self.types.insert(type_name.clone(), TypeInfo {
                            name: type_name,
                            size: ty.size(),
                            fields: Vec::new(),
                        });
                    }
                }
            }
        }
    }

    pub fn may_alias_types(&self, a: &str, b: &str) -> bool {
        // Types alias if one is a subtype of the other
        if a == b {
            return true;
        }

        if let Some(parents_a) = self.hierarchy.get(a) {
            for parent in parents_a {
                if self.may_alias_types(parent, b) {
                    return true;
                }
            }
        }

        if let Some(parents_b) = self.hierarchy.get(b) {
            for parent in parents_b {
                if self.may_alias_types(a, parent) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub size: Option<usize>,
    pub fields: Vec<FieldInfo>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub offset: usize,
    pub ty: String,
}

use std::collections::HashMap;
