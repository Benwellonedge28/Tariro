//! Derive/Attribute System for TUMC
//! Implements procedural macros for derive macros like Clone, Debug, etc.

use crate::prelude::*;
use super::ast::*;
use std::collections::HashMap;

/// Built-in derive macros
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeriveMacro {
    /// Clone trait
    Clone,
    /// Debug trait
    Debug,
    /// Default trait
    Default,
    /// PartialEq trait
    PartialEq,
    /// Eq trait
    Eq,
    /// PartialOrd trait
    PartialOrd,
    /// Ord trait
    Ord,
    /// Hash trait
    Hash,
    /// Serialize (for serialization)
    Serialize,
    /// Deserialize (for deserialization)
    Deserialize,
    /// Display trait
    Display,
    /// Error trait
    Error,
    /// From trait
    From,
    /// Into trait
    Into,
    /// AsRef trait
    AsRef,
    /// AsMut trait
    AsMut,
    /// Borrow trait
    Borrow,
    /// Custom derive
    Custom(String),
}

impl DeriveMacro {
    /// Parse from string
    pub fn parse(name: &str) -> Option<DeriveMacro> {
        match name {
            "Clone" => Some(DeriveMacro::Clone),
            "Debug" => Some(DeriveMacro::Debug),
            "Default" => Some(DeriveMacro::Default),
            "PartialEq" => Some(DeriveMacro::PartialEq),
            "Eq" => Some(DeriveMacro::Eq),
            "PartialOrd" => Some(DeriveMacro::PartialOrd),
            "Ord" => Some(DeriveMacro::Ord),
            "Hash" => Some(DeriveMacro::Hash),
            "Serialize" => Some(DeriveMacro::Serialize),
            "Deserialize" => Some(DeriveMacro::Deserialize),
            "Display" => Some(DeriveMacro::Display),
            "Error" => Some(DeriveMacro::Error),
            "From" => Some(DeriveMacro::From),
            "Into" => Some(DeriveMacro::Into),
            "AsRef" => Some(DeriveMacro::AsRef),
            "AsMut" => Some(DeriveMacro::AsMut),
            "Borrow" => Some(DeriveMacro::Borrow),
            _ => Some(DeriveMacro::Custom(name.to_string())),
        }
    }

    /// Get the trait name
    pub fn trait_name(&self) -> String {
        match self {
            DeriveMacro::Clone => "Clone",
            DeriveMacro::Debug => "Debug",
            DeriveMacro::Default => "Default",
            DeriveMacro::PartialEq => "PartialEq",
            DeriveMacro::Eq => "Eq",
            DeriveMacro::PartialOrd => "PartialOrd",
            DeriveMacro::Ord => "Ord",
            DeriveMacro::Hash => "Hash",
            DeriveMacro::Serialize => "Serialize",
            DeriveMacro::Deserialize => "Deserialize",
            DeriveMacro::Display => "Display",
            DeriveMacro::Error => "Error",
            DeriveMacro::From => "From",
            DeriveMacro::Into => "Into",
            DeriveMacro::AsRef => "AsRef",
            DeriveMacro::AsMut => "AsMut",
            DeriveMacro::Borrow => "Borrow",
            DeriveMacro::Custom(name) => name,
        }.to_string()
    }
}

/// Attribute kind
#[derive(Debug, Clone)]
pub enum AttributeKind {
    /// #[derive(...)]
    Derive(Vec<DeriveMacro>),
    /// #[attr(...)]
    Custom(String, Vec<AttributeArg>),
    /// #[attr]
    Flag(String),
    /// #[attr = value]
    KeyValue(String, String),
}

/// Attribute argument
#[derive(Debug, Clone)]
pub struct AttributeArg {
    pub name: Option<String>,
    pub value: AttributeValue,
}

/// Attribute value
#[derive(Debug, Clone)]
pub enum AttributeValue {
    /// String literal
    String(String),
    /// Integer literal
    Int(i64),
    /// Boolean
    Bool(bool),
    /// Path
    Path(String),
    /// List
    List(Vec<AttributeArg>),
}

/// Derive implementation generator
pub struct DeriveGenerator {
    /// Custom derive handlers
    custom_derives: HashMap<String, DeriveHandler>,
}

/// Derive handler function type
pub type DeriveHandler = fn(&Struct, &[AttributeArg]) -> Result<Vec<Item>, DeriveError>;

impl DeriveGenerator {
    /// Create new derive generator
    pub fn new() -> Self {
        let mut gen = DeriveGenerator {
            custom_derives: HashMap::new(),
        };

        // Register default derives
        gen.register_default_derives();

        gen
    }

    /// Register default derive handlers
    fn register_default_derives(&mut self) {
        // Clone
        self.custom_derives.insert("Clone".to_string(), |struct_, _| {
            Ok(vec![Item::Impl(ImplBlock {
                trait_name: None,
                type_name: struct_.name.clone(),
                ty_params: struct_.ty_params.clone(),
                functions: vec![
                    Function {
                        name: "clone".to_string(),
                        params: vec![],
                        return_type: AnnType::Named(struct_.name.clone()),
                        body: vec![Stmt::new_expr(Expr::new_identifier(struct_.name.to_lowercase()))],
                        span: Span::default(),
                    }
                ],
                span: Span::default(),
            })])
        });
    }

    /// Register custom derive
    pub fn register_derive(&mut self, name: String, handler: DeriveHandler) {
        self.custom_derives.insert(name, handler);
    }

    /// Process derive attribute
    pub fn process_derive(&self, derive_attr: &Attribute, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        let mut items = Vec::new();

        for derive_macro in &derive_attr.derives {
            let trait_name = derive_macro.trait_name();

            // Generate implementation based on derive type
            let impl_items = match derive_macro {
                DeriveMacro::Clone => self.derive_clone(struct_)?,
                DeriveMacro::Debug => self.derive_debug(struct_)?,
                DeriveMacro::Default => self.derive_default(struct_)?,
                DeriveMacro::PartialEq => self.derive_partial_eq(struct_)?,
                DeriveMacro::Eq => self.derive_eq(struct_)?,
                DeriveMacro::Hash => self.derive_hash(struct_)?,
                DeriveMacro::Serialize => self.derive_serialize(struct_)?,
                DeriveMacro::Deserialize => self.derive_deserialize(struct_)?,
                DeriveMacro::Display => self.derive_display(struct_)?,
                DeriveMacro::From => self.derive_from(struct_)?,
                _ => {
                    // Check if custom handler exists
                    if let Some(handler) = self.custom_derives.get(&trait_name) {
                        handler(struct_, &derive_attr.args)?
                    } else {
                        return Err(DeriveError::UnknownDerive(trait_name));
                    }
                }
            };

            items.extend(impl_items);
        }

        Ok(items)
    }

    /// Derive Clone implementation
    fn derive_clone(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        let mut stmts = Vec::new();

        for field in &struct_.fields {
            stmts.push(Stmt::new_expr(Expr::new_method_call(
                Expr::new_identifier(format!("self.{}", field.name)),
                "clone",
                vec![],
                Span::default(),
            )));
        }

        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("Clone".to_string()),
            type_name: struct_.name.clone(),
            ty_params: struct_.ty_params.clone(),
            functions: vec![Function {
                name: "clone".to_string(),
                params: vec![],
                return_type: AnnType::Named(struct_.name.clone()),
                body: stmts,
                span: Span::default(),
            }],
            span: Span::default(),
        })])
    }

    /// Derive Debug implementation
    fn derive_debug(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        let mut stmts = Vec::new();

        // Write struct name
        stmts.push(Stmt::new_expr(Expr::new_method_call(
            Expr::new_identifier("f"),
            "debug_struct",
            vec![Expr::new_literal(Literal::String(struct_.name.clone()), Span::default())],
            Span::default(),
        )));

        // Write each field
        for field in &struct_.fields {
            stmts.push(Stmt::new_expr(Expr::new_method_call(
                Expr::new_identifier("field"),
                "field",
                vec![
                    Expr::new_literal(Literal::String(field.name.clone()), Span::default()),
                    Expr::new_identifier(format!("&self.{}", field.name)),
                ],
                Span::default(),
            )));
        }

        // Finish
        stmts.push(Stmt::new_expr(Expr::new_method_call(
            Expr::new_identifier("builder"),
            "finish",
            vec![],
            Span::default(),
        )));

        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("Debug".to_string()),
            type_name: struct_.name.clone(),
            ty_params: struct_.ty_params.clone(),
            functions: vec![Function {
                name: "fmt".to_string(),
                params: vec![
                    FunctionParam { name: "f".to_string(), ann_type: AnnType::Named("Formatter".to_string()) }
                ],
                return_type: AnnType::Named("Result".to_string()),
                body: vec![Stmt::new_expr(Expr::new_identifier("builder"))],
                span: Span::default(),
            }],
            span: Span::default(),
        })])
    }

    /// Derive Default implementation
    fn derive_default(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        let mut stmts = Vec::new();

        for field in &struct_.fields {
            stmts.push(Stmt::new_expr(Expr::new_method_call(
                Expr::new_identifier(field.name.to_lowercase()),
                "default",
                vec![],
                Span::default(),
            )));
        }

        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("Default".to_string()),
            type_name: struct_.name.clone(),
            ty_params: struct_.ty_params.clone(),
            functions: vec![Function {
                name: "default".to_string(),
                params: vec![],
                return_type: AnnType::Named(struct_.name.clone()),
                body: vec![Stmt::new_expr(Expr::new_identifier(struct_.name.to_lowercase()))],
                span: Span::default(),
            }],
            span: Span::default(),
        })])
    }

    /// Derive PartialEq implementation
    fn derive_partial_eq(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        let mut stmts = Vec::new();

        // Compare each field
        for field in &struct_.fields {
            stmts.push(Stmt::new(StmtKind::If(
                Expr::new_binary(
                    BinOp::Ne,
                    Expr::new_identifier(format!("self.{}", field.name)),
                    Expr::new_identifier(format!("other.{}", field.name)),
                    Span::default(),
                ),
                vec![Stmt::new_return(Some(Expr::new_literal(Literal::Bool(false), Span::default())))],
                None,
                Span::default(),
            )));
        }

        stmts.push(Stmt::new_return(Some(Expr::new_literal(Literal::Bool(true), Span::default()))));

        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("PartialEq".to_string()),
            type_name: struct_.name.clone(),
            ty_params: struct_.ty_params.clone(),
            functions: vec![Function {
                name: "eq".to_string(),
                params: vec![
                    FunctionParam {
                        name: "other".to_string(),
                        ann_type: AnnType::Named(format!("&{}", struct_.name))
                    }
                ],
                return_type: AnnType::Named("bool".to_string()),
                body: stmts,
                span: Span::default(),
            }],
            span: Span::default(),
        })])
    }

    /// Derive Eq implementation
    fn derive_eq(&self, _struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        // Eq is marker trait, no methods needed
        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("Eq".to_string()),
            type_name: "Self".to_string(),
            ty_params: vec![],
            functions: vec![],
            span: Span::default(),
        })])
    }

    /// Derive Hash implementation
    fn derive_hash(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        let mut stmts = Vec::new();

        for field in &struct_.fields {
            stmts.push(Stmt::new_expr(Expr::new_method_call(
                Expr::new_identifier("state"),
                "write_u64",
                vec![Expr::new_identifier(format!("self.{}", field.name))],
                Span::default(),
            )));
        }

        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("Hash".to_string()),
            type_name: struct_.name.clone(),
            ty_params: struct_.ty_params.clone(),
            functions: vec![Function {
                name: "hash".to_string(),
                params: vec![
                    FunctionParam {
                        name: "state".to_string(),
                        ann_type: AnnType::Named("Hasher".to_string())
                    }
                ],
                return_type: AnnType::Unit,
                body: stmts,
                span: Span::default(),
            }],
            span: Span::default(),
        })])
    }

    /// Derive Serialize implementation
    fn derive_serialize(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        let mut stmts = Vec::new();

        // Serialize struct start
        stmts.push(Stmt::new_expr(Expr::new_method_call(
            Expr::new_identifier("serializer"),
            "serialize_struct",
            vec![Expr::new_literal(Literal::String(struct_.name.clone()), Span::default())],
            Span::default(),
        )));

        // Serialize each field
        for field in &struct_.fields {
            stmts.push(Stmt::new_expr(Expr::new_method_call(
                Expr::new_identifier("state"),
                "serialize_field",
                vec![
                    Expr::new_literal(Literal::String(field.name.clone()), Span::default()),
                    Expr::new_identifier(format!("&self.{}", field.name)),
                ],
                Span::default(),
            )));
        }

        // End serialization
        stmts.push(Stmt::new_expr(Expr::new_identifier("state")));

        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("Serialize".to_string()),
            type_name: struct_.name.clone(),
            ty_params: struct_.ty_params.clone(),
            functions: vec![Function {
                name: "serialize".to_string(),
                params: vec![
                    FunctionParam {
                        name: "serializer".to_string(),
                        ann_type: AnnType::Named("S".to_string())
                    }
                ],
                return_type: AnnType::Named("Result<S::Ok, S::Error>".to_string()),
                body: stmts,
                span: Span::default(),
            }],
            span: Span::default(),
        })])
    }

    /// Derive Deserialize implementation
    fn derive_deserialize(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        let mut stmts = Vec::new();

        // Deserialize struct
        stmts.push(Stmt::new_expr(Expr::new_method_call(
            Expr::new_identifier("deserializer"),
            "deserialize_struct",
            vec![Expr::new_literal(Literal::String(struct_.name.clone()), Span::default())],
            Span::default(),
        )));

        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("Deserialize".to_string()),
            type_name: struct_.name.clone(),
            ty_params: struct_.ty_params.clone(),
            functions: vec![Function {
                name: "deserialize".to_string(),
                params: vec![
                    FunctionParam {
                        name: "deserializer".to_string(),
                        ann_type: AnnType::Named("D".to_string())
                    }
                ],
                return_type: AnnType::Named("Result<Self, D::Error>".to_string()),
                body: stmts,
                span: Span::default(),
            }],
            span: Span::default(),
        })])
    }

    /// Derive Display implementation
    fn derive_display(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        // Display delegates to Debug
        self.derive_display_via_debug(struct_)
    }

    /// Derive Display via Debug
    fn derive_display_via_debug(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        Ok(vec![Item::Impl(ImplBlock {
            trait_name: Some("Display".to_string()),
            type_name: struct_.name.clone(),
            ty_params: struct_.ty_params.clone(),
            functions: vec![Function {
                name: "fmt".to_string(),
                params: vec![
                    FunctionParam {
                        name: "f".to_string(),
                        ann_type: AnnType::Named("Formatter".to_string())
                    }
                ],
                return_type: AnnType::Named("fmt::Result".to_string()),
                body: vec![
                    Stmt::new_expr(Expr::new_method_call(
                        Expr::new_identifier("self"),
                        "debug_fmt",
                        vec![Expr::new_identifier("f")],
                        Span::default(),
                    ))
                ],
                span: Span::default(),
            }],
            span: Span::default(),
        })])
    }

    /// Derive From implementation
    fn derive_from(&self, struct_: &Struct) -> Result<Vec<Item>, DeriveError> {
        // From requires specific source type
        Err(DeriveError::RequiresArgument {
            derive_name: "From".to_string(),
            message: "From requires specifying source type: #[derive(From<Source>)]".to_string(),
        })
    }
}

/// Derive error
#[derive(Debug, Clone)]
pub enum DeriveError {
    /// Unknown derive macro
    UnknownDerive(String),
    /// Derive requires additional argument
    RequiresArgument { derive_name: String, message: String },
    /// Cannot derive for this type
    CannotDerive { type_name: String, reason: String },
    /// Invalid attribute
    InvalidAttribute(String),
}

impl fmt::Display for DeriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeriveError::UnknownDerive(name) => {
                write!(f, "Unknown derive macro: {}", name)
            }
            DeriveError::RequiresArgument { derive_name, message } => {
                write!(f, "{}: {}", derive_name, message)
            }
            DeriveError::CannotDerive { type_name, reason } => {
                write!(f, "Cannot derive for '{}': {}", type_name, reason)
            }
            DeriveError::InvalidAttribute(msg) => {
                write!(f, "Invalid attribute: {}", msg)
            }
        }
    }
}

impl std::error::Error for DeriveError {}

/// Processed derive attribute
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The derives to apply
    pub derives: Vec<DeriveMacro>,
    /// Additional arguments
    pub args: Vec<AttributeArg>,
}

impl Attribute {
    /// Parse from attribute syntax
    pub fn parse(attr: &super::ast::Attribute) -> Result<Self, DeriveError> {
        let mut derives = Vec::new();
        let mut args = Vec::new();

        // Parse derive list
        for token in &attr.tokens {
            if let TokenKind::Identifier(name) = &token.kind {
                if let Some(derive) = DeriveMacro::parse(name) {
                    derives.push(derive);
                }
            }
        }

        Ok(Attribute { derives, args })
    }
}

/// Token kinds for attribute parsing
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier(String),
    String(String),
    Integer(i64),
    Punct(char),
    EOF,
}

/// Attribute transformer for processing attributes
pub struct AttributeTransformer {
    /// Built-in attributes
    builtins: HashMap<String, AttributeHandler>,
}

type AttributeHandler = fn(&[AttributeArg]) -> Result<Vec<Transformation>, AttributeError>;

/// Transformation to apply
#[derive(Debug, Clone)]
pub enum Transformation {
    /// Add field
    AddField(String, AnnType),
    /// Remove field
    RemoveField(String),
    /// Add impl block
    AddImpl(Item),
    /// Add function
    AddFunction(String, Function),
    /// Add trait bound
    AddTraitBound(String, String),
    /// Validate constraint
    Validate(String),
}

/// Attribute error
#[derive(Debug, Clone)]
pub enum AttributeError {
    InvalidAttribute(String),
    ValidationFailed(String),
}

impl fmt::Display for AttributeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeError::InvalidAttribute(msg) => write!(f, "Invalid attribute: {}", msg),
            AttributeError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
        }
    }
}

impl std::error::Error for AttributeError {}

impl AttributeTransformer {
    /// Create new attribute transformer
    pub fn new() -> Self {
        let mut transformer = AttributeTransformer {
            builtins: HashMap::new(),
        };
        transformer.register_builtins();
        transformer
    }

    /// Register built-in attributes
    fn register_builtins(&mut self) {
        // #[inline]
        self.builtins.insert("inline".to_string(), |_| {
            Ok(vec![Transformation::Validate("inline".to_string())])
        });

        // #[cold]
        self.builtins.insert("cold".to_string(), |_| {
            Ok(vec![Transformation::Validate("cold".to_string())])
        });

        // #[deprecated]
        self.builtins.insert("deprecated".to_string(), |_| {
            Ok(vec![Transformation::Validate("deprecated".to_string())])
        });

        // #[must_use]
        self.builtins.insert("must_use".to_string(), |_| {
            Ok(vec![Transformation::Validate("must_use".to_string())])
        });

        // #[repr]
        self.builtins.insert("repr".to_string(), |args| {
            if let Some(arg) = args.first() {
                Ok(vec![Transformation::Validate(format!("repr({})", arg.value))])
            } else {
                Err(AttributeError::InvalidAttribute("repr requires argument".to_string()))
            }
        });
    }

    /// Register custom attribute
    pub fn register(&mut self, name: String, handler: AttributeHandler) {
        self.builtins.insert(name, handler);
    }

    /// Transform item based on attributes
    pub fn transform(&self, attrs: &[Attribute], item: &mut Item) -> Result<Vec<Transformation>, AttributeError> {
        let mut transformations = Vec::new();

        for attr in attrs {
            if let Some(handler) = self.builtins.get(&attr.name) {
                let result = handler(&attr.args)?;
                transformations.extend(result);
            }
        }

        Ok(transformations)
    }
}

/// Built-in derive attributes
#[derive(Debug, Clone)]
pub struct BuiltinDerive {
    pub name: &'static str,
    pub description: &'static str,
    pub generates_trait: Option<&'static str>,
}

pub const BUILTIN_DERIVES: &[BuiltinDerive] = &[
    BuiltinDerive {
        name: "Clone",
        description: "Clone trait implementation",
        generates_trait: Some("Clone"),
    },
    BuiltinDerive {
        name: "Debug",
        description: "Debug trait implementation",
        generates_trait: Some("Debug"),
    },
    BuiltinDerive {
        name: "Default",
        description: "Default trait implementation",
        generates_trait: Some("Default"),
    },
    BuiltinDerive {
        name: "PartialEq",
        description: "PartialEq trait implementation",
        generates_trait: Some("PartialEq"),
    },
    BuiltinDerive {
        name: "Eq",
        description: "Eq trait implementation (marker)",
        generates_trait: Some("Eq"),
    },
    BuiltinDerive {
        name: "PartialOrd",
        description: "PartialOrd trait implementation",
        generates_trait: Some("PartialOrd"),
    },
    BuiltinDerive {
        name: "Ord",
        description: "Ord trait implementation",
        generates_trait: Some("Ord"),
    },
    BuiltinDerive {
        name: "Hash",
        description: "Hash trait implementation",
        generates_trait: Some("Hash"),
    },
    BuiltinDerive {
        name: "Serialize",
        description: "Serialize trait implementation",
        generates_trait: Some("Serialize"),
    },
    BuiltinDerive {
        name: "Deserialize",
        description: "Deserialize trait implementation",
        generates_trait: Some("Deserialize"),
    },
    BuiltinDerive {
        name: "Display",
        description: "Display trait implementation",
        generates_trait: Some("Display"),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_macro_parsing() {
        assert_eq!(DeriveMacro::parse("Clone"), Some(DeriveMacro::Clone));
        assert_eq!(DeriveMacro::parse("Debug"), Some(DeriveMacro::Debug));
        assert_eq!(DeriveMacro::parse("Custom"), Some(DeriveMacro::Custom("Custom".to_string())));
    }

    #[test]
    fn test_derive_generator() {
        let gen = DeriveGenerator::new();
        assert!(gen.custom_derives.contains_key("Clone"));
    }

    #[test]
    fn test_attribute_transformer() {
        let transformer = AttributeTransformer::new();
        assert!(transformer.builtins.contains_key("inline"));
        assert!(transformer.builtins.contains_key("deprecated"));
    }
}
