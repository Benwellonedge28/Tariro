// Points-to Analysis

use super::*;

/// Points-to analysis implementation
pub struct PointsToAnalysis {
    pub level: PointsToLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointsToLevel {
    /// Field-insensitive, flow-insensitive
    Insensitive,
    /// Field-sensitive, flow-insensitive
    FieldSensitive,
    /// Field-insensitive, flow-sensitive
    FlowSensitive,
    /// Field-sensitive, flow-sensitive
    FlowSensitiveFieldSensitive,
    /// Steensgaard's unification-based (almost linear)
    Unification,
    /// Andersen's inclusion-based (cubic worst case)
    Inclusion,
}

impl Default for PointsToAnalysis {
    fn default() -> Self {
        Self {
            level: PointsToLevel::FlowInsensitive,
        }
    }
}

impl PointsToAnalysis {
    pub fn new(level: PointsToLevel) -> Self {
        Self { level }
    }

    pub fn analyze(&self, module: &Module) -> PointsToResult {
        match self.level {
            PointsToLevel::Unification => self.unification_analysis(module),
            PointsToLevel::Inclusion => self.inclusion_analysis(module),
            _ => self.flow_insensitive_analysis(module),
        }
    }

    fn unification_analysis(&self, module: &Module) -> PointsToResult {
        let mut result = PointsToResult::new();

        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    match &inst.op {
                        Operation::Alloca(ty, _) => {
                            // Points to heap or stack allocation
                            let loc = PointsToLocation::Stack(LocationId(result.locations.len()));
                            result.locations.push(Location::new(ty.clone()));
                            result.points_to.insert(PointsToKey::Value(inst.id), vec![loc]);
                        }
                        Operation::Load(_, ptr, _, _) => {
                            // Copy points-to set
                            if let Some(src_pts) = result.points_to.get(&PointsToKey::Value(*ptr)) {
                                result.points_to.insert(PointsToKey::Value(inst.id), src_pts.clone());
                            }
                        }
                        Operation::Store(_, ptr, val, _) => {
                            // Points-to update
                            if let Some(val_pts) = result.points_to.get(&PointsToKey::Value(*val)) {
                                // Would add points-to for *ptr = val
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        result
    }

    fn inclusion_analysis(&self, module: &Module) -> PointsToResult {
        // Andersen's style: compute inclusion constraints
        let mut result = PointsToResult::new();

        // Generate constraints
        let mut constraints = Vec::new();

        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    match &inst.op {
                        Operation::Alloca(ty, _) => {
                            let loc = PointsToLocation::Heap(LocationId(result.locations.len()));
                            result.locations.push(Location::new(ty.clone()));
                            constraints.push(Constraint::Allocate(inst.id, loc));
                        }
                        Operation::Load(_, ptr, _, _) => {
                            constraints.push(Constraint::Copy(inst.id, *ptr));
                        }
                        Operation::Store(_, ptr, val, _) => {
                            constraints.push(Constraint::Store(*ptr, *val));
                        }
                        Operation::Bitcast(val, _) => {
                            constraints.push(Constraint::Copy(inst.id, *val));
                        }
                        _ => {}
                    }
                }
            }
        }

        // Solve constraints iteratively
        let mut changed = true;
        while changed {
            changed = false;
            for constraint in &constraints {
                if self.solve_constraint(constraint, &mut result) {
                    changed = true;
                }
            }
        }

        result
    }

    fn flow_insensitive_analysis(&self, module: &Module) -> PointsToResult {
        let mut result = PointsToResult::new();

        // Simplified: just track allocations
        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    if let Operation::Alloca(ty, _) = &inst.op {
                        let loc = PointsToLocation::Stack(LocationId(result.locations.len()));
                        result.locations.push(Location::new(ty.clone()));
                        result.points_to.insert(PointsToKey::Value(inst.id), vec![loc]);
                    }
                }
            }
        }

        result
    }

    fn solve_constraint(&self, constraint: &Constraint, result: &mut PointsToResult) -> bool {
        match constraint {
            Constraint::Allocate(val, loc) => {
                let pts = result.points_to.entry(PointsToKey::Value(*val)).or_default();
                if !pts.contains(loc) {
                    pts.push(*loc);
                    return true;
                }
            }
            Constraint::Copy(dst, src) => {
                if let Some(src_pts) = result.points_to.get(&PointsToKey::Value(*src)) {
                    let pts = result.points_to.entry(PointsToKey::Value(*dst)).or_default();
                    let initial_len = pts.len();
                    pts.extend(src_pts.iter().cloned());
                    return pts.len() > initial_len;
                }
            }
            Constraint::Store(ptr, val) => {
                // Would handle field-sensitive stores
            }
            Constraint::Load(ptr, loc) => {
                // Would handle field-sensitive loads
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct PointsToResult {
    pub points_to: HashMap<PointsToKey, Vec<PointsToLocation>>,
    pub locations: Vec<Location>,
}

impl PointsToResult {
    pub fn new() -> Self {
        Self {
            points_to: HashMap::new(),
            locations: Vec::new(),
        }
    }

    pub fn may_alias(&self, a: ValueId, b: ValueId) -> bool {
        let pts_a = self.points_to.get(&PointsToKey::Value(a));
        let pts_b = self.points_to.get(&PointsToKey::Value(b));

        if let (Some(a_pts), Some(b_pts)) = (pts_a, pts_b) {
            for loc_a in a_pts {
                for loc_b in b_pts {
                    if loc_a == loc_b {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn points_to_set(&self, value: ValueId) -> &[PointsToLocation] {
        self.points_to.get(&PointsToKey::Value(value)).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

impl Default for PointsToResult {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PointsToKey {
    Value(ValueId),
    Location(LocationId, FieldId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct LocationId(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct FieldId(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum PointsToLocation {
    Stack(LocationId),
    Heap(LocationId),
    Null,
    Unknown,
    Global(GlobalId),
}

#[derive(Debug, Clone)]
pub struct Location {
    pub id: LocationId,
    pub ty: IrType,
    pub fields: Vec<FieldLocation>,
}

impl Location {
    pub fn new(ty: IrType) -> Self {
        Self {
            id: LocationId(0),
            ty,
            fields: Vec::new(),
        }
    }

    pub fn field(&self, idx: FieldId) -> Option<&FieldLocation> {
        self.fields.get(idx.0)
    }
}

#[derive(Debug, Clone)]
pub struct FieldLocation {
    pub id: FieldId,
    pub offset: usize,
    pub ty: IrType,
}

#[derive(Debug, Clone)]
enum Constraint {
    Allocate(ValueId, PointsToLocation),
    Copy(ValueId, ValueId),
    Store(ValueId, ValueId),
    Load(ValueId, PointsToLocation),
}

use std::collections::HashMap;
