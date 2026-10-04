// Abstract Interpretation for Static Analysis

use crate::tumc_compiler::ir::{Instruction, IrType, Operation, BinOp};
use std::collections::HashMap;

/// Abstract domain type
#[derive(Debug, Clone, Clone)]
pub enum AbstractDomainType {
    /// Interval domain for numeric values
    Interval(Interval),
    /// Sign domain
    Sign(Sign),
    /// Constant domain
    Constant(i64),
    /// Pointer domain
    Pointer(PointerInfo),
    /// Set of possible values
    Set(HashMap<i64>),
    /// Top (unknown)
    Top,
    /// Bottom (unreachable)
    Bottom,
}

#[derive(Debug, Clone, Clone)]
pub struct Interval {
    pub lower: i64,
    pub upper: i64,
}

impl Interval {
    pub fn top() -> Self {
        Self {
            lower: i64::MIN,
            upper: i64::MAX,
        }
    }

    pub fn bottom() -> Self {
        Self {
            lower: 1,
            upper: 0, // lower > upper indicates bottom
        }
    }

    pub fn constant(c: i64) -> Self {
        Self { lower: c, upper: c }
    }

    pub fn is_bottom(&self) -> bool {
        self.lower > self.upper
    }

    pub fn join(&self, other: &Interval) -> Interval {
        if self.is_bottom() {
            return other.clone();
        }
        if other.is_bottom() {
            return self.clone();
        }
        Interval {
            lower: self.lower.min(other.lower),
            upper: self.upper.max(other.upper),
        }
    }

    pub fn meet(&self, other: &Interval) -> Interval {
        if self.is_bottom() || other.is_bottom() {
            return Self::bottom();
        }
        Interval {
            lower: self.lower.max(other.lower),
            upper: self.upper.min(other.upper),
        }
    }

    pub fn widening(&self, other: &Interval) -> Interval {
        let lower = if other.lower < self.lower {
            i64::MIN
        } else {
            self.lower
        };
        let upper = if other.upper > self.upper {
            i64::MAX
        } else {
            self.upper
        };
        Interval { lower, upper }
    }

    pub fn add(&self, other: &Interval) -> Interval {
        if self.is_bottom() || other.is_bottom() {
            return Self::bottom();
        }
        let (low, overflow_low) = self.lower.overflowing_add(other.lower);
        let (high, overflow_high) = self.upper.overflowing_add(other.upper);
        if overflow_low || overflow_high {
            return Self::top();
        }
        Interval { lower: low, upper: high }
    }

    pub fn sub(&self, other: &Interval) -> Interval {
        if self.is_bottom() || other.is_bottom() {
            return Self::bottom();
        }
        let (low, overflow_low) = self.lower.overflowing_sub(other.upper);
        let (high, overflow_high) = self.upper.overflowing_sub(other.lower);
        if overflow_low || overflow_high {
            return Self::top();
        }
        Interval { lower: low, upper: high }
    }

    pub fn mul(&self, other: &Interval) -> Interval {
        if self.is_bottom() || other.is_bottom() {
            return Self::bottom();
        }
        let candidates = [
            self.lower * other.lower,
            self.lower * other.upper,
            self.upper * other.lower,
            self.upper * other.upper,
        ];
        Interval {
            lower: candidates.iter().cloned().min().unwrap_or(i64::MIN),
            upper: candidates.iter().cloned().max().unwrap_or(i64::MAX),
        }
    }

    pub fn div(&self, other: &Interval) -> Interval {
        if self.is_bottom() || other.is_bottom() {
            return Self::bottom();
        }
        // Simplified: assume non-zero divisor
        let candidates = [
            self.lower / other.lower.max(1),
            self.lower / other.upper.min(-1).max(1),
            self.upper / other.lower.max(1),
            self.upper / other.upper.min(-1).max(1),
        ];
        Interval {
            lower: candidates.iter().cloned().filter(|&x| x != i64::MIN).min().unwrap_or(i64::MIN),
            upper: candidates.iter().cloned().filter(|&x| x != i64::MAX).max().unwrap_or(i64::MAX),
        }
    }

    pub fn contains(&self, val: i64) -> bool {
        !self.is_bottom() && self.lower <= val && val <= self.upper
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
    Unknown,
    Bottom,
}

impl Sign {
    pub fn from_interval(interval: &Interval) -> Self {
        if interval.is_bottom() {
            return Sign::Bottom;
        }
        if interval.lower > 0 {
            Sign::Positive
        } else if interval.upper < 0 {
            Sign::Negative
        } else if interval.lower == 0 && interval.upper == 0 {
            Sign::Zero
        } else {
            Sign::Unknown
        }
    }
}

#[derive(Debug, Clone)]
pub struct PointerInfo {
    pub may_be_null: bool,
    pub may_alias: Vec<String>,
    pub points_to: Vec<AbstractValue>,
    pub offset: Interval,
}

impl PointerInfo {
    pub fn unknown() -> Self {
        Self {
            may_be_null: true,
            may_alias: Vec::new(),
            points_to: Vec::new(),
            offset: Interval::top(),
        }
    }

    pub fn non_null() -> Self {
        Self {
            may_be_null: false,
            may_alias: Vec::new(),
            points_to: Vec::new(),
            offset: Interval::top(),
        }
    }
}

/// Abstract value
#[derive(Debug, Clone)]
pub struct AbstractValue {
    pub domain_type: AbstractDomainType,
}

impl AbstractValue {
    pub fn top() -> Self {
        Self {
            domain_type: AbstractDomainType::Top,
        }
    }

    pub fn bottom() -> Self {
        Self {
            domain_type: AbstractDomainType::Bottom,
        }
    }

    pub fn interval(lower: i64, upper: i64) -> Self {
        Self {
            domain_type: AbstractDomainType::Interval(Interval { lower, upper }),
        }
    }

    pub fn constant(c: i64) -> Self {
        Self {
            domain_type: AbstractDomainType::Constant(c),
        }
    }

    pub fn sign(sign: Sign) -> Self {
        Self {
            domain_type: AbstractDomainType::Sign(sign),
        }
    }

    pub fn is_bottom(&self) -> bool {
        match &self.domain_type {
            AbstractDomainType::Bottom => true,
            AbstractDomainType::Interval(i) => i.is_bottom(),
            AbstractDomainType::Sign(s) => *s == Sign::Bottom,
            _ => false,
        }
    }

    pub fn join(&self, other: &AbstractValue) -> AbstractValue {
        match (&self.domain_type, &other.domain_type) {
            (AbstractDomainType::Bottom, _) => other.clone(),
            (_, AbstractDomainType::Bottom) => self.clone(),
            (AbstractDomainType::Interval(a), AbstractDomainType::Interval(b)) => {
                AbstractValue {
                    domain_type: AbstractDomainType::Interval(a.join(b)),
                }
            }
            (AbstractDomainType::Constant(a), AbstractDomainType::Constant(b)) if a == b => {
                self.clone()
            }
            _ => AbstractValue::top(),
        }
    }

    pub fn meet(&self, other: &AbstractValue) -> AbstractValue {
        match (&self.domain_type, &other.domain_type) {
            (AbstractDomainType::Bottom, _) | (_, AbstractDomainType::Bottom) => {
                AbstractValue::bottom()
            }
            (AbstractDomainType::Interval(a), AbstractDomainType::Interval(b)) => {
                AbstractValue {
                    domain_type: AbstractDomainType::Interval(a.meet(b)),
                }
            }
            _ => AbstractValue::top(),
        }
    }

    pub fn widening(&self, other: &AbstractValue) -> AbstractValue {
        match (&self.domain_type, &other.domain_type) {
            (AbstractDomainType::Interval(a), AbstractDomainType::Interval(b)) => {
                AbstractValue {
                    domain_type: AbstractDomainType::Interval(a.widening(b)),
                }
            }
            _ => AbstractValue::top(),
        }
    }
}

/// Abstract domain state
#[derive(Debug, Clone)]
pub struct AbstractDomain {
    pub variables: HashMap<String, AbstractValue>,
    pub memory: HashMap<String, AbstractValue>,
}

impl AbstractDomain {
    pub fn top() -> Self {
        Self {
            variables: HashMap::new(),
            memory: HashMap::new(),
        }
    }

    pub fn bottom() -> Self {
        Self {
            variables: HashMap::new(),
            memory: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> AbstractValue {
        self.variables.get(name).cloned().unwrap_or(AbstractValue::top())
    }

    pub fn set(&mut self, name: String, value: AbstractValue) {
        self.variables.insert(name, value);
    }

    pub fn join(&self, other: &AbstractDomain) -> AbstractDomain {
        let mut result = AbstractDomain::top();
        for (name, val) in &self.variables {
            if let Some(other_val) = other.variables.get(name) {
                result.variables.insert(name.clone(), val.join(other_val));
            } else {
                result.variables.insert(name.clone(), val.clone());
            }
        }
        for (name, val) in &other.variables {
            if !result.variables.contains_key(name) {
                result.variables.insert(name.clone(), val.clone());
            }
        }
        result
    }

    pub fn meet(&self, other: &AbstractDomain) -> AbstractDomain {
        let mut result = AbstractDomain::bottom();
        for (name, val) in &self.variables {
            if let Some(other_val) = other.variables.get(name) {
                result.variables.insert(name.clone(), val.meet(other_val));
            }
        }
        result
    }

    pub fn widening(&self, other: &AbstractDomain) -> AbstractDomain {
        let mut result = AbstractDomain::top();
        for (name, val) in &self.variables {
            if let Some(other_val) = other.variables.get(name) {
                result.variables.insert(name.clone(), val.widening(other_val));
            } else {
                result.variables.insert(name.clone(), val.clone());
            }
        }
        for (name, val) in &other.variables {
            if !result.variables.contains_key(name) {
                result.variables.insert(name.clone(), val.clone());
            }
        }
        result
    }

    pub fn is_bottom(&self) -> bool {
        self.variables.values().all(|v| v.is_bottom())
    }
}

/// Abstract interpreter
pub struct AbstractInterpreter {
    max_iterations: usize,
}

impl AbstractInterpreter {
    pub fn new(max_iterations: usize) -> Self {
        Self { max_iterations }
    }

    /// Analyze a function
    pub fn analyze(&self, instructions: &[Instruction]) -> AbstractDomain {
        let mut state = AbstractDomain::top();
        let mut new_state = AbstractDomain::top();
        let mut iterations = 0;

        loop {
            new_state = state.clone();
            for inst in instructions {
                new_state = self.analyze_instruction(inst, &new_state);
            }

            if new_state.join(&state) == state || iterations >= self.max_iterations {
                break;
            }
            state = state.join(&new_state);
            iterations += 1;
        }

        state
    }

    /// Analyze a single instruction
    fn analyze_instruction(&self, inst: &Instruction, state: &AbstractDomain) -> AbstractDomain {
        let mut new_state = state.clone();

        match &inst.op {
            Operation::Binary(op, lhs, rhs) => {
                let left_val = state.get(lhs);
                let right_val = state.get(rhs);
                let result = self.compute_binary_op(op, &left_val, &right_val);
                new_state.set(inst.result_name.clone().unwrap_or_default(), result);
            }
            Operation::Const(val, ty) => {
                let result = match val {
                    crate::tumc_compiler::ir::Const::Int(i) => AbstractValue::constant(*i),
                    _ => AbstractValue::top(),
                };
                new_state.set(inst.result_name.clone().unwrap_or_default(), result);
            }
            Operation::Load(_, ptr, ty, offset) => {
                let ptr_val = state.get(ptr);
                // Simplified: return top for loads
                new_state.set(inst.result_name.clone().unwrap_or_default(), AbstractValue::top());
            }
            Operation::Store(val, ptr, _) => {
                let val_val = state.get(val);
                new_state.set(ptr.clone(), val_val);
            }
            Operation::Alloc(ty, size) => {
                // Allocation returns non-null pointer
                new_state.set(
                    inst.result_name.clone().unwrap_or_default(),
                    AbstractValue {
                        domain_type: AbstractDomainType::Pointer(PointerInfo::non_null()),
                    },
                );
            }
            _ => {}
        }

        new_state
    }

    /// Compute binary operation in abstract domain
    fn compute_binary_op(&self, op: &BinOp, lhs: &AbstractValue, rhs: &AbstractValue) -> AbstractValue {
        match (&lhs.domain_type, &rhs.domain_type) {
            (AbstractDomainType::Interval(a), AbstractDomainType::Interval(b)) => {
                let result = match op {
                    BinOp::Add => a.add(b),
                    BinOp::Sub => a.sub(b),
                    BinOp::Mul => a.mul(b),
                    BinOp::Div => a.div(b),
                    _ => Interval::top(),
                };
                AbstractValue {
                    domain_type: AbstractDomainType::Interval(result),
                }
            }
            _ => AbstractValue::top(),
        }
    }

    /// Check if a condition is always true
    pub fn is_always_true(&self, cond: &str, state: &AbstractDomain) -> bool {
        // Simplified: check if condition variable is constant > 0
        let val = state.get(cond);
        match &val.domain_type {
            AbstractDomainType::Interval(i) => i.lower > 0,
            _ => false,
        }
    }

    /// Check if a condition is always false
    pub fn is_always_false(&self, cond: &str, state: &AbstractDomain) -> bool {
        let val = state.get(cond);
        match &val.domain_type {
            AbstractDomainType::Interval(i) => i.upper < 0 || i.is_bottom(),
            _ => false,
        }
    }
}

impl Default for AbstractInterpreter {
    fn default() -> Self {
        Self::new(1000)
    }
}
