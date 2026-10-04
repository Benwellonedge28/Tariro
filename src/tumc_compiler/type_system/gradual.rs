// Gradual Typing Support

use super::*;

/// Gradual type for typed/untyped boundary
#[derive(Debug, Clone)]
pub enum GradualType {
    Static(MonoType),
    Dynamic,
    Unknown,
}

impl GradualType {
    pub fn is_dynamic(&self) -> bool {
        matches!(self, GradualType::Dynamic)
    }

    pub fn is_static(&self) -> bool {
        matches!(self, GradualType::Static(_))
    }

    pub fn to_mono(&self) -> Option<&MonoType> {
        match self {
            GradualType::Static(ty) => Some(ty),
            _ => None,
        }
    }
}

/// Cast between gradual types
pub struct GradualCast;

impl GradualCast {
    /// Generate casts at typed/untyped boundary
    pub fn generate_cast(from: &GradualType, to: &GradualType) -> Option<CastOp> {
        match (from, to) {
            (GradualType::Static(_), GradualType::Dynamic) => {
                Some(CastOp::ToDynamic)
            }
            (GradualType::Dynamic, GradualType::Static(_)) => {
                Some(CastOp::FromDynamic)
            }
            (GradualType::Static(a), GradualType::Static(b)) => {
                // Type-safe cast
                None
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CastOp {
    ToDynamic,
    FromDynamic,
    TypeCheck(MonoType),
}

/// Blame tracking for gradual typing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blame {
    Positive,  // Cast was inserted by typed side
    Negative,  // Cast was inserted by untyped side
}

impl Blame {
    pub fn flip(self) -> Self {
        match self {
            Blame::Positive => Blame::Negative,
            Blame::Negative => Blame::Positive,
        }
    }
}

/// Blame tracking context
pub struct BlameTracker {
    pub casts: Vec<CastInfo>,
}

#[derive(Debug, Clone)]
pub struct CastInfo {
    pub source: Location,
    pub target: Location,
    pub blame: Blame,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl BlameTracker {
    pub fn new() -> Self {
        Self {
            casts: Vec::new(),
        }
    }

    pub fn record_cast(&mut self, source: Location, target: Location, blame: Blame) {
        self.casts.push(CastInfo {
            source,
            target,
            blame,
            success: true,
        });
    }

    pub fn record_failure(&mut self, blame: Blame) {
        if let Some(cast) = self.casts.iter_mut().rev().find(|c| c.blame == blame) {
            cast.success = false;
        }
    }
}

impl Default for BlameTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Consistent-subtype relation for gradual types
pub fn consistent_subtype(a: &GradualType, b: &GradualType) -> bool {
    match (a, b) {
        (GradualType::Unknown, _) => true,
        (_, GradualType::Unknown) => true,
        (GradualType::Dynamic, _) => true,
        (_, GradualType::Dynamic) => true,
        (GradualType::Static(a), GradualType::Static(b)) => {
            // Static types must match exactly
            a == b
        }
    }
}

/// Static gradual typing ground types
pub static GROUND_TYPES: [&str; 8] = [
    "int", "float", "bool", "string", "char",
    "Dynamic", "Unknown", "any",
];

/// Check if type is a ground type
pub fn is_ground_type(ty: &GradualType) -> bool {
    match ty {
        GradualType::Static(ty) => {
            matches!(
                ty,
                MonoType::Int(_) | MonoType::Float(_) |
                MonoType::Bool | MonoType::Char | MonoType::String |
                MonoType::Never
            )
        }
        GradualType::Dynamic => true,
        GradualType::Unknown => true,
    }
}

/// Untyped hole representation
pub const UNKNOWN_TYPE: GradualType = GradualType::Unknown;

/// Dynamic type representation
pub const DYNAMIC_TYPE: GradualType = GradualType::Dynamic;
