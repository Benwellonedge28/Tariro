//! TUMC WebAssembly Component Model
//!
//! Implementation of the WASM Component Model for modular, composable
//! WebAssembly applications.

use std::collections::HashMap;

/// Component model configuration
#[derive(Debug, Clone)]
pub struct ComponentConfig {
    /// Component name
    pub name: String,
    /// Component version
    pub version: String,
    /// Component world
    pub world: World,
    /// Resources
    pub resources: Vec<ResourceType>,
    /// Functions
    pub functions: Vec<ComponentFunction>,
}

impl Default for ComponentConfig {
    fn default() -> Self {
        Self {
            name: "component".to_string(),
            version: "0.1.0".to_string(),
            world: World::default(),
            resources: Vec::new(),
            functions: Vec::new(),
        }
    }
}

/// WASM Component
#[derive(Debug, Clone)]
pub struct Component {
    /// Component ID
    pub id: ComponentId,
    /// Binary representation
    pub binary: Vec<u8>,
    /// Exports
    pub exports: HashMap<String, Export>,
    /// Imports
    pub imports: HashMap<String, Import>,
    /// Types
    pub types: Vec<ComponentType>,
}

/// Component ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub String);

/// World (interface set)
#[derive(Debug, Clone, Default)]
pub struct World {
    /// World name
    pub name: String,
    /// Exports
    pub exports: Vec<InterfaceItem>,
    /// Imports
    pub imports: Vec<InterfaceItem>,
}

/// Interface item
#[derive(Debug, Clone)]
pub struct InterfaceItem {
    /// Name
    pub name: String,
    /// Type
    pub item_type: InterfaceType,
}

/// Interface type
#[derive(Debug, Clone)]
pub enum InterfaceType {
    /// Function
    Func(FunctionType),
    /// Resource
    Resource(ResourceType),
    /// Type alias
    Type(TypeRef),
}

/// Resource type
#[derive(Debug, Clone)]
pub struct ResourceType {
    /// Resource name
    pub name: String,
    /// Destructor (if owned)
    pub destructor: Option<String>,
    /// Methods
    pub methods: Vec<ComponentFunction>,
}

/// Component function
#[derive(Debug, Clone)]
pub struct ComponentFunction {
    /// Function name
    pub name: String,
    /// Parameters
    pub params: Vec<(String, ComponentValType)>,
    /// Results
    pub results: Vec<(String, ComponentValType)>,
}

/// Component value type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentValType {
    Unit,
    Bool,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    Float32,
    Float64,
    Char,
    String,
    List(Box<ComponentValType>),
    Option(Box<ComponentValType>),
    Result(Box<ComponentValType>, Box<ComponentValType>),
    Tuple(Vec<ComponentValType>),
    Flags(Vec<String>),
    Enum(Vec<String>),
    Union(Vec<ComponentValType>),
}

/// Component type
#[derive(Debug, Clone)]
pub enum ComponentType {
    /// Primitive type
    Primitive(ComponentValType),
    /// Record type
    Record(Vec<(String, ComponentValType)>),
    /// Variant type
    Variant(Vec<(String, Option<ComponentValType>)>),
    /// List type
    List(Box<ComponentValType>),
    /// Function type
    Func(Box<FunctionType>),
    /// Resource type
    Resource(ResourceType),
}

/// Function type
#[derive(Debug, Clone)]
pub struct FunctionType {
    /// Parameters
    pub params: Vec<ComponentValType>,
    /// Results
    pub results: Vec<ComponentValType>,
}

/// Type reference
#[derive(Debug, Clone)]
pub enum TypeRef {
    /// Index reference
    Index(u32),
    /// Name reference
    Name(String),
}

/// Export
#[derive(Debug, Clone)]
pub struct Export {
    /// Export name
    pub name: String,
    /// Export type
    pub kind: ExportKind,
}

/// Export kind
#[derive(Debug, Clone)]
pub enum ExportKind {
    /// Function
    Function(u32),
    /// Value
    Value(ComponentValType),
    /// Type
    Type(u32),
    /// Instance
    Instance(u32),
    /// Component
    Component(u32),
}

/// Import
#[derive(Debug, Clone)]
pub struct Import {
    /// Module name
    pub module: String,
    /// Name
    pub name: String,
    /// Import type
    pub kind: ImportKind,
}

/// Import kind
#[derive(Debug, Clone)]
pub enum ImportKind {
    /// Function
    Function(FunctionType),
    /// Value
    Value(ComponentValType),
    /// Type
    Type(ComponentType),
    /// Module
    Module(ModuleType),
    /// Instance
    Instance(InstanceType),
    /// Component
    Component(ComponentType),
}

/// Module type
#[derive(Debug, Clone)]
pub struct ModuleType {
    /// Imports
    pub imports: Vec<Import>,
    /// Exports
    pub exports: Vec<String>,
}

/// Instance type
#[derive(Debug, Clone)]
pub struct InstanceType {
    /// Exports
    pub exports: Vec<(String, ExportKind)>,
}

/// Component type definition
#[derive(Debug, Clone)]
pub struct ComponentTypeDef {
    /// Type index
    pub index: u32,
    /// Type kind
    pub kind: ComponentTypeDefKind,
}

/// Type definition kind
#[derive(Debug, Clone)]
pub enum ComponentTypeDefKind {
    /// Defined type
    Defined(ComponentType),
    /// Function type
    Func(FunctionType),
    /// Component type
    Component(ComponentType),
    /// Instance type
    Instance(InstanceType),
    /// Resource type
    Resource(ResourceType),
}

/// Component builder
pub struct ComponentBuilder {
    /// Configuration
    config: ComponentConfig,
    /// Types
    types: Vec<ComponentTypeDef>,
    /// Functions
    functions: Vec<Vec<u8>>,
    /// Core modules
    core_modules: Vec<CoreModule>,
    /// Component definitions
    component_defs: Vec<ComponentTypeDef>,
}

impl ComponentBuilder {
    /// Create new builder
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            config: ComponentConfig {
                name: name.to_string(),
                version: version.to_string(),
                ..Default::default()
            },
            types: Vec::new(),
            functions: Vec::new(),
            core_modules: Vec::new(),
            component_defs: Vec::new(),
        }
    }

    /// Add type
    pub fn add_type(&mut self, name: &str, type_def: ComponentType) -> &mut Self {
        self.types.push(ComponentTypeDef {
            index: self.types.len() as u32,
            kind: ComponentTypeDefKind::Defined(type_def),
        });
        self
    }

    /// Add function type
    pub fn add_func_type(&mut self, params: Vec<ComponentValType>, results: Vec<ComponentValType>) -> &mut Self {
        self.types.push(ComponentTypeDef {
            index: self.types.len() as u32,
            kind: ComponentTypeDefKind::Func(FunctionType { params, results }),
        });
        self
    }

    /// Add resource type
    pub fn add_resource(&mut self, name: &str) -> &mut Self {
        self.types.push(ComponentTypeDef {
            index: self.types.len() as u32,
            kind: ComponentTypeDefKind::Resource(ResourceType {
                name: name.to_string(),
                destructor: None,
                methods: Vec::new(),
            }),
        });
        self
    }

    /// Add core module
    pub fn add_core_module(&mut self, name: &str, binary: Vec<u8>) -> &mut Self {
        self.core_modules.push(CoreModule {
            name: name.to_string(),
            binary,
        });
        self
    }

    /// Add export
    pub fn export(&mut self, name: &str, kind: ExportKind) -> &mut Self {
        // Would add to component type definition
        self
    }

    /// Build component
    pub fn build(&self) -> Component {
        // Generate component binary
        let binary = self.encode_component();

        Component {
            id: ComponentId(format!("{}:{}", self.config.name, self.config.version)),
            binary,
            exports: HashMap::new(),
            imports: HashMap::new(),
            types: Vec::new(),
        }
    }

    /// Encode component to binary
    fn encode_component(&self) -> Vec<u8> {
        let mut binary = Vec::new();

        // Magic number
        binary.extend_from_slice(b"\0wasm");

        // Component magic
        binary.extend_from_slice(b"\0\x61\x73\x6d"); // \0asm

        // Version
        binary.extend_from_slice(&1u32.to_le_bytes());

        // Encode types section
        self.encode_types(&mut binary);

        // Encode code section
        self.encode_code(&mut binary);

        binary
    }

    /// Encode types
    fn encode_types(&self, binary: &mut Vec<u8>) {
        let mut type_section = Vec::new();
        for type_def in &self.types {
            self.encode_type_def(type_def, &mut type_section);
        }

        // Section header
        binary.push(0x01); // Type section
        binary.extend_from_slice(&(type_section.len() as u32).to_le_bytes());
        binary.extend_from_slice(&type_section);
    }

    /// Encode single type
    fn encode_type_def(&self, type_def: &ComponentTypeDef, binary: &mut Vec<u8>) {
        match &type_def.kind {
            ComponentTypeDefKind::Defined(ty) => {
                self.encode_component_type(ty, binary);
            }
            ComponentTypeDefKind::Func(func) => {
                binary.push(0x01); // Func type
                self.encode_function_type(func, binary);
            }
            _ => {}
        }
    }

    /// Encode component type
    fn encode_component_type(&self, ty: &ComponentType, binary: &mut Vec<u8>) {
        match ty {
            ComponentType::Primitive(val_type) => {
                self.encode_val_type(val_type, binary);
            }
            ComponentType::Record(fields) => {
                binary.push(0x05); // Record
                binary.push(fields.len() as u8);
                for (name, ty) in fields {
                    binary.extend_from_slice(name.as_bytes());
                    self.encode_val_type(ty, binary);
                }
            }
            ComponentType::Variant(cases) => {
                binary.push(0x06); // Variant
                binary.push(cases.len() as u8);
                for (name, ty) in cases {
                    binary.extend_from_slice(name.as_bytes());
                    if let Some(t) = ty {
                        binary.push(1);
                        self.encode_val_type(t, binary);
                    } else {
                        binary.push(0);
                    }
                }
            }
            ComponentType::List(elem_type) => {
                binary.push(0x07); // List
                self.encode_val_type(elem_type, binary);
            }
            ComponentType::Func(func_type) => {
                binary.push(0x01); // Function
                self.encode_function_type(func_type, binary);
            }
            _ => {}
        }
    }

    /// Encode value type
    fn encode_val_type(&self, ty: &ComponentValType, binary: &mut Vec<u8>) {
        match ty {
            ComponentValType::Unit => binary.push(0x7F),
            ComponentValType::Bool => binary.push(0x7E),
            ComponentValType::U8 => binary.push(0x7D),
            ComponentValType::U16 => binary.push(0x7C),
            ComponentValType::U32 => binary.push(0x7B),
            ComponentValType::U64 => binary.push(0x7A),
            ComponentValType::S8 => binary.push(0x79),
            ComponentValType::S16 => binary.push(0x78),
            ComponentValType::S32 => binary.push(0x77),
            ComponentValType::S64 => binary.push(0x76),
            ComponentValType::Float32 => binary.push(0x75),
            ComponentValType::Float64 => binary.push(0x74),
            ComponentValType::Char => binary.push(0x73),
            ComponentValType::String => binary.push(0x72),
            ComponentValType::List(inner) => {
                binary.push(0x71);
                self.encode_val_type(inner, binary);
            }
            ComponentValType::Option(inner) => {
                binary.push(0x70);
                self.encode_val_type(inner, binary);
            }
            ComponentValType::Result(inner) => {
                binary.push(0x6F);
                self.encode_val_type(inner, binary);
            }
            ComponentValType::Tuple(types) => {
                binary.push(0x6B);
                binary.push(types.len() as u8);
                for ty in types {
                    self.encode_val_type(ty, binary);
                }
            }
            ComponentValType::Flags(names) => {
                binary.push(0x68);
                binary.push(names.len() as u8);
                for name in names {
                    binary.extend_from_slice(name.as_bytes());
                }
            }
            ComponentValType::Enum(variants) => {
                binary.push(0x69);
                binary.push(variants.len() as u8);
                for variant in variants {
                    binary.extend_from_slice(variant.as_bytes());
                }
            }
            ComponentValType::Union(types) => {
                binary.push(0x6A);
                binary.push(types.len() as u8);
                for ty in types {
                    self.encode_val_type(ty, binary);
                }
            }
        }
    }

    /// Encode function type
    fn encode_function_type(&self, func: &FunctionType, binary: &mut Vec<u8>) {
        binary.push(func.params.len() as u8);
        for param in &func.params {
            self.encode_val_type(param, binary);
        }
        binary.push(func.results.len() as u8);
        for result in &func.results {
            self.encode_val_type(result, binary);
        }
    }

    /// Encode code section
    fn encode_code(&self, binary: &mut Vec<u8>) {
        let mut code_section = Vec::new();

        // Encode each function
        for _func in &self.functions {
            // Simple no-op function
            code_section.push(0x01); // Size
            code_section.push(0x00); // Local count
            code_section.push(0x0B); // End
        }

        binary.push(0x0A); // Code section
        binary.extend_from_slice(&(code_section.len() as u32).to_le_bytes());
        binary.extend_from_slice(&code_section);
    }
}

/// Core WASM module wrapper
#[derive(Debug, Clone)]
pub struct CoreModule {
    /// Module name
    pub name: String,
    /// Binary
    pub binary: Vec<u8>,
}

/// Component linker
pub struct ComponentLinker {
    /// Resolved instances
    instances: HashMap<String, Component>,
    /// Arguments for instantiation
    args: HashMap<String, Vec<(String, ImportKind)>>,
}

impl ComponentLinker {
    /// Create new linker
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            args: HashMap::new(),
        }
    }

    /// Add instance
    pub fn instance(&mut self, name: &str, component: Component) -> &mut Self {
        self.instances.insert(name.to_string(), component);
        self
    }

    /// Add argument
    pub fn arg(&mut self, instance: &str, name: &str, kind: ImportKind) -> &mut Self {
        self.args
            .entry(instance.to_string())
            .or_default()
            .push((name.to_string(), kind));
        self
    }

    /// Link components
    pub fn link(&self) -> Result<Component, LinkError> {
        // Simplified linking
        Ok(Component {
            id: ComponentId("linked".to_string()),
            binary: Vec::new(),
            exports: HashMap::new(),
            imports: HashMap::new(),
            types: Vec::new(),
        })
    }
}

/// Link error
#[derive(Debug)]
pub enum LinkError {
    MissingImport(String),
    TypeMismatch { expected: String, found: String },
    CyclicDependency,
}

/// Component adapter
pub struct ComponentAdapter {
    /// Source type
    pub source: ComponentValType,
    /// Target type
    pub target: ComponentValType,
    /// Lift function (core → component)
    pub lift: Vec<u8>,
    /// Lower function (component → core)
    pub lower: Vec<u8>,
}

/// WIT parser for component interfaces
pub struct WitParser {
    /// Parsed packages
    packages: Vec<WitPackage>,
}

/// WIT package
#[derive(Debug, Clone)]
pub struct WitPackage {
    /// Package name
    pub name: String,
    /// Version
    pub version: String,
    /// World
    pub world: World,
    /// Types
    pub types: Vec<WitTypeDef>,
}

/// WIT type definition
#[derive(Debug, Clone)]
pub struct WitTypeDef {
    pub name: String,
    pub kind: WitTypeKind,
}

/// WIT type kind
#[derive(Debug, Clone)]
pub enum WitTypeKind {
    Record(Vec<WitField>),
    Variant(Vec<WitVariantCase>),
    Enum(Vec<String>),
    Flags(Vec<String>),
    Resource,
    Alias(WitType),
}

/// WIT field
#[derive(Debug, Clone)]
pub struct WitField {
    pub name: String,
    pub ty: WitType,
}

/// WIT variant case
#[derive(Debug, Clone)]
pub struct WitVariantCase {
    pub name: String,
    pub ty: Option<WitType>,
}

/// WIT type
#[derive(Debug, Clone)]
pub enum WitType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    Float32,
    Float64,
    Char,
    String,
    List(Box<WitType>),
    Option(Box<WitType>),
    Result(Box<WitType>, Box<WitType>),
    Tuple(Vec<WitType>),
    ListType(Box<WitType>),
    TypeRef(String),
}

impl WitParser {
    /// Create new parser
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    /// Parse WIT document
    pub fn parse(&mut self, source: &str) -> Result<&WitPackage, ParseError> {
        // Simplified parsing - would use pest or similar
        let package = WitPackage {
            name: "parsed".to_string(),
            version: "0.1.0".to_string(),
            world: World::default(),
            types: Vec::new(),
        };
        self.packages.push(package);
        Ok(self.packages.last().unwrap())
    }
}

/// Parse error
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_val_type_encoding() {
        let builder = ComponentBuilder::new("test", "1.0.0");
        let mut binary = Vec::new();
        builder.encode_val_type(&ComponentValType::U32, &mut binary);
        assert_eq!(binary, vec![0x7B]);
    }

    #[test]
    fn test_component_builder() {
        let mut builder = ComponentBuilder::new("my-component", "0.1.0");
        builder.add_func_type(
            vec![ComponentValType::U32, ComponentValType::U32],
            vec![ComponentValType::U32],
        );
        let component = builder.build();
        assert!(!component.binary.is_empty());
    }

    #[test]
    fn test_wit_type() {
        let ty = WitType::Option(Box::new(WitType::U32));
        assert!(matches!(ty, WitType::Option(_)));
    }
}
