//! TUMC Documentation Generator
//!
//! Automatic documentation generation for TUMC programs including
//! HTML, Markdown, and PDF output formats.

use crate::tumc_compiler::*;

/// Documentation generator
pub struct DocGenerator {
    /// Output format
    format: DocFormat,
    /// Include private items
    include_private: bool,
    /// Include source code
    include_source: bool,
    /// Highlight syntax
    highlight_syntax: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocFormat {
    Html,
    Markdown,
    Latex,
    Json,
}

/// Documentation item
#[derive(Debug, Clone)]
pub struct DocItem {
    /// Item kind
    pub kind: DocKind,
    /// Name
    pub name: String,
    /// Documentation comment
    pub doc: String,
    /// Source location
    pub location: Option<SourceLocation>,
    /// Children (for modules, structs, etc.)
    pub children: Vec<DocItem>,
    /// Attributes
    pub attributes: Vec<DocAttribute>,
}

/// Documentation kind
#[derive(Debug, Clone)]
pub enum DocKind {
    Module,
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Constant,
    Type,
    Macro,
}

/// Documentation attribute
#[derive(Debug, Clone)]
pub struct DocAttribute {
    pub name: String,
    pub value: Option<String>,
}

/// Documentation renderer
pub trait DocRenderer {
    /// Render documentation item
    fn render_item(&self, item: &DocItem) -> String;

    /// Render documentation for entire program
    fn render_program(&self, items: &[DocItem]) -> String;
}

/// HTML documentation renderer
pub struct HtmlRenderer {
    /// CSS theme
    theme: HtmlTheme,
}

#[derive(Debug, Clone)]
pub struct HtmlTheme {
    pub primary_color: String,
    pub background_color: String,
    pub font_family: String,
    pub code_font_family: String,
}

impl Default for HtmlTheme {
    fn default() -> Self {
        Self {
            primary_color: "#4a90e2".to_string(),
            background_color: "#ffffff".to_string(),
            font_family: "system-ui, sans-serif".to_string(),
            code_font_family: "monospace".to_string(),
        }
    }
}

impl HtmlRenderer {
    /// Create new HTML renderer
    pub fn new() -> Self {
        Self {
            theme: HtmlTheme::default(),
        }
    }

    /// Set theme
    pub fn with_theme(mut self, theme: HtmlTheme) -> Self {
        self.theme = theme;
        self
    }
}

impl DocRenderer for HtmlRenderer {
    fn render_item(&self, item: &DocItem) -> String {
        let mut html = String::new();

        html.push_str(&format!(
            r#"<div class="doc-item doc-kind-{:?}">"#,
            format!("{:?}", item.kind).to_lowercase()
        ));

        // Header
        html.push_str(&format!(
            r#"<h{level} id="{}"><a href="#{}">{}</a></h{level}>"#,
            2,
            item.name.to_lowercase().replace(' ', "-"),
            item.name,
            2
        ));

        // Documentation
        if !item.doc.is_empty() {
            html.push_str(&format!(
                r#"<div class="doc-comment"><p>{}</p></div>"#,
                item.doc.replace('\n', "</p><p>")
            ));
        }

        // Attributes
        if !item.attributes.is_empty() {
            html.push_str(r#"<div class="doc-attributes"><ul>"#);
            for attr in &item.attributes {
                let value = attr.value.as_deref().unwrap_or("");
                html.push_str(&format!(
                    r#"<li><code>@{}</code>{}</li>"#,
                    attr.name,
                    if value.is_empty() { "".to_string() } else { format!(" = {}", value) }
                ));
            }
            html.push_str("</ul></div>");
        }

        // Source link
        if let Some(loc) = &item.location {
            html.push_str(&format!(
                r#"<div class="doc-location">{}:{}</div>"#,
                loc.line, loc.column
            ));
        }

        // Children
        if !item.children.is_empty() {
            html.push_str(r#"<div class="doc-children">"#);
            for child in &item.children {
                html.push_str(&self.render_item(child));
            }
            html.push_str("</div>");
        }

        html.push_str("</div>");

        html
    }

    fn render_program(&self, items: &[DocItem]) -> String {
        let mut html = String::new();

        // Header
        html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>TUMC Program Documentation</title>
    <style>
        body {
            font-family: system-ui, sans-serif;
            max-width: 900px;
            margin: 0 auto;
            padding: 20px;
            background: #f5f5f5;
        }
        .doc-item {
            background: white;
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 16px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h2 { color: #333; margin-top: 0; }
        .doc-comment { color: #666; line-height: 1.6; }
        .doc-attributes { background: #f9f9f9; padding: 10px; border-radius: 4px; }
        .doc-location { color: #999; font-size: 0.9em; }
        .doc-children { margin-left: 20px; }
        pre { background: #f4f4f4; padding: 10px; border-radius: 4px; overflow-x: auto; }
        code { background: #f4f4f4; padding: 2px 4px; border-radius: 2px; }
    </style>
</head>
<body>
    <h1>TUMC Program Documentation</h1>
"#);

        for item in items {
            html.push_str(&self.render_item(item));
        }

        html.push_str("</body></html>");

        html
    }
}

/// Markdown documentation renderer
pub struct MarkdownRenderer {
    /// Include table of contents
    include_toc: bool,
}

impl MarkdownRenderer {
    /// Create new Markdown renderer
    pub fn new() -> Self {
        Self {
            include_toc: true,
        }
    }

    /// Set include TOC
    pub fn with_toc(mut self, include: bool) -> Self {
        self.include_toc = include;
        self
    }
}

impl DocRenderer for MarkdownRenderer {
    fn render_item(&self, item: &DocItem) -> String {
        let mut md = String::new();

        // Header
        let kind_marker = match item.kind {
            DocKind::Module => "##",
            DocKind::Function => "##",
            DocKind::Struct => "##",
            DocKind::Enum => "##",
            DocKind::Trait => "##",
            DocKind::Impl => "##",
            DocKind::Constant => "##",
            DocKind::Type => "##",
            DocKind::Macro => "##",
        };

        md.push_str(&format!("{} {}\n\n", kind_marker, item.name));

        // Documentation
        if !item.doc.is_empty() {
            md.push_str(&format!("{}\n\n", item.doc));
        }

        // Attributes
        if !item.attributes.is_empty() {
            md.push_str("### Attributes\n\n");
            md.push_str("| Name | Value |\n");
            md.push_str("|------|-------|\n");
            for attr in &item.attributes {
                let value = attr.value.as_deref().unwrap_or("-");
                md.push_str(&format!("| `@{}` | `{}` |\n", attr.name, value));
            }
            md.push_str("\n");
        }

        // Location
        if let Some(loc) = &item.location {
            md.push_str(&format!(
                "*Defined at line {}, column {}*\n\n",
                loc.line, loc.column
            ));
        }

        // Children
        if !item.children.is_empty() {
            md.push_str("---\n\n");
            for child in &item.children {
                md.push_str(&self.render_item(child));
            }
        }

        md
    }

    fn render_program(&self, items: &[DocItem]) -> String {
        let mut md = String::new();

        // Title
        md.push_str("# TUMC Program Documentation\n\n");

        // Table of contents
        if self.include_toc && !items.is_empty() {
            md.push_str("## Table of Contents\n\n");
            for item in items {
                md.push_str(&format!(
                    "- [{}](#{})\n",
                    item.name,
                    item.name.to_lowercase().replace(' ', "-")
                ));
            }
            md.push_str("\n---\n\n");
        }

        // Items
        for item in items {
            md.push_str(&self.render_item(item));
        }

        md
    }
}

/// LaTeX documentation renderer
pub struct LatexRenderer {
    /// Document class options
    document_class: String,
}

impl LatexRenderer {
    /// Create new LaTeX renderer
    pub fn new() -> Self {
        Self {
            document_class: "article".to_string(),
        }
    }
}

impl DocRenderer for LatexRenderer {
    fn render_item(&self, item: &DocItem) -> String {
        let mut tex = String::new();

        // Section
        let section_level = match item.kind {
            DocKind::Module | DocKind::Struct | DocKind::Enum | DocKind::Trait => "section",
            DocKind::Function | DocKind::Constant | DocKind::Type | DocKind::Macro => "subsection",
            DocKind::Impl => "subsubsection",
        };

        tex.push_str(&format!(
            "\\{}{{ {} }}\n\n",
            section_level,
            escape_latex(&item.name)
        ));

        // Documentation
        if !item.doc.is_empty() {
            tex.push_str(&format!("{}\n\n", escape_latex(&item.doc)));
        }

        // Children
        for child in &item.children {
            tex.push_str(&self.render_item(child));
        }

        tex
    }

    fn render_program(&self, items: &[DocItem]) -> String {
        let mut tex = String::new();

        tex.push_str(&format!(
            "\\documentclass[12pt]{{{}}}\n\n",
            self.document_class
        ));

        tex.push_str(r#"
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{hyperref}
\usepackage{minted}

\title{TUMC Program Documentation}
\author{Generated by TUMC}
\date{\today}

\begin{document}

\maketitle

"#);

        for item in items {
            tex.push_str(&self.render_item(item));
        }

        tex.push_str("\n\\end{document}\n");

        tex
    }
}

/// JSON documentation renderer
pub struct JsonRenderer;

impl JsonRenderer {
    /// Create new JSON renderer
    pub fn new() -> Self {
        Self
    }
}

impl DocRenderer for JsonRenderer {
    fn render_item(&self, item: &DocItem) -> String {
        serde_json::to_string_pretty(item).unwrap_or_else(|_| "{}".to_string())
    }

    fn render_program(&self, items: &[DocItem]) -> String {
        let doc = serde_json::json!({
            "program": "TUMC Program",
            "items": items
        });
        serde_json::to_string_pretty(&doc).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Escape LaTeX special characters
fn escape_latex(s: &str) -> String {
    s.replace('\\', "\\textbackslash{}")
        .replace('&', "\\&")
        .replace('%', "\\%")
        .replace('$', "\\$")
        .replace('#', "\\#")
        .replace('_', "\\_")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('~', "\\textasciitilde{}")
        .replace('^', "\\textasciicircum{}")
}

/// AST to documentation converter
pub struct AstToDocConverter;

impl AstToDocConverter {
    /// Convert AST to documentation items
    pub fn convert(ast: &ast::Program) -> Vec<DocItem> {
        let mut items = Vec::new();

        for item in &ast.items {
            match item {
                ast::Item::Function(f) => {
                    items.push(DocItem {
                        kind: DocKind::Function,
                        name: f.name.clone(),
                        doc: f.doc.clone().unwrap_or_default(),
                        location: Some(f.location),
                        children: Vec::new(),
                        attributes: Vec::new(),
                    });
                }
                ast::Item::Struct(s) => {
                    let children = s
                        .fields
                        .iter()
                        .map(|f| DocItem {
                            kind: DocKind::Type,
                            name: f.name.clone(),
                            doc: f.doc.clone().unwrap_or_default(),
                            location: Some(f.location),
                            children: Vec::new(),
                            attributes: Vec::new(),
                        })
                        .collect();

                    items.push(DocItem {
                        kind: DocKind::Struct,
                        name: s.name.clone(),
                        doc: s.doc.clone().unwrap_or_default(),
                        location: Some(s.location),
                        children,
                        attributes: Vec::new(),
                    });
                }
                ast::Item::Enum(e) => {
                    let children = e
                        .variants
                        .iter()
                        .map(|v| DocItem {
                            kind: DocKind::Type,
                            name: v.name.clone(),
                            doc: v.doc.clone().unwrap_or_default(),
                            location: Some(v.location),
                            children: Vec::new(),
                            attributes: Vec::new(),
                        })
                        .collect();

                    items.push(DocItem {
                        kind: DocKind::Enum,
                        name: e.name.clone(),
                        doc: e.doc.clone().unwrap_or_default(),
                        location: Some(e.location),
                        children,
                        attributes: Vec::new(),
                    });
                }
                _ => {}
            }
        }

        items
    }
}

/// Documentation generator builder
pub struct DocGeneratorBuilder {
    format: DocFormat,
    include_private: bool,
    include_source: bool,
    highlight_syntax: bool,
}

impl Default for DocGeneratorBuilder {
    fn default() -> Self {
        Self {
            format: DocFormat::Html,
            include_private: false,
            include_source: true,
            highlight_syntax: true,
        }
    }
}

impl DocGeneratorBuilder {
    /// Set format
    pub fn format(mut self, format: DocFormat) -> Self {
        self.format = format;
        self
    }

    /// Set include private
    pub fn include_private(mut self, include: bool) -> Self {
        self.include_private = include;
        self
    }

    /// Set include source
    pub fn include_source(mut self, include: bool) -> Self {
        self.include_source = include;
        self
    }

    /// Set highlight syntax
    pub fn highlight_syntax(mut self, highlight: bool) -> Self {
        self.highlight_syntax = highlight;
        self
    }

    /// Build generator
    pub fn build(&self) -> DocGenerator {
        DocGenerator {
            format: self.format,
            include_private: self.include_private,
            include_source: self.include_source,
            highlight_syntax: self.highlight_syntax,
        }
    }
}

impl DocGenerator {
    /// Generate documentation
    pub fn generate(&self, ast: &ast::Program) -> String {
        let items = AstToDocConverter::convert(ast);
        let filtered: Vec<DocItem> = if self.include_private {
            items
        } else {
            items
                .into_iter()
                .filter(|i| !i.name.starts_with('_'))
                .collect()
        };

        match self.format {
            DocFormat::Html => HtmlRenderer::new().render_program(&filtered),
            DocFormat::Markdown => MarkdownRenderer::new().render_program(&filtered),
            DocFormat::Latex => LatexRenderer::new().render_program(&filtered),
            DocFormat::Json => JsonRenderer::new().render_program(&filtered),
        }
    }
}

/// API documentation generator
pub struct ApiDocGenerator;

impl ApiDocGenerator {
    /// Generate API documentation
    pub fn generate_api_docs(compiler: &Compiler) -> String {
        let mut docs = String::new();

        docs.push_str("# TUMC Compiler API Documentation\n\n");

        // Compiler
        docs.push_str("## Compiler\n\n");
        docs.push_str("```rust\n");
        docs.push_str("pub struct Compiler {\n");
        docs.push_str("    source_language: SourceLanguage,\n");
        docs.push_str("    target_platform: TargetPlatform,\n");
        docs.push_str("    optimization_level: OptimizationLevel,\n");
        docs.push_str("}\n");
        docs.push_str("```\n\n");

        // Methods
        docs.push_str("### Methods\n\n");
        docs.push_str("- `new(source, target)` - Create new compiler\n");
        docs.push_str("- `compile(source)` - Compile source code\n");
        docs.push_str("- `set_optimization(level)` - Set optimization level\n\n");

        // Source languages
        docs.push_str("## Source Languages\n\n");
        docs.push_str("| Language | Description |\n");
        docs.push_str("|----------|-------------|\n");
        docs.push_str("| Rust | Rust language subset |\n");
        docs.push_str("| Python | Python language subset |\n");
        docs.push_str("| JavaScript | JavaScript subset |\n");
        docs.push_str("| TumcIR | TUMC Intermediate Representation |\n\n");

        // Target platforms
        docs.push_str("## Target Platforms\n\n");
        docs.push_str("| Platform | Output |\n");
        docs.push_str("|----------|--------|\n");
        docs.push_str("| Wasm | WebAssembly binary |\n");
        docs.push_str("| Native | Native executable |\n");
        docs.push_str("| JavaScript | JavaScript/ASM.js |\n");
        docs.push_str("| TariroBytecode | Tariro VM bytecode |\n\n");

        docs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_renderer() {
        let items = vec![DocItem {
            kind: DocKind::Function,
            name: "test".to_string(),
            doc: "A test function".to_string(),
            location: None,
            children: Vec::new(),
            attributes: Vec::new(),
        }];

        let renderer = HtmlRenderer::new();
        let html = renderer.render_program(&items);
        assert!(html.contains("test"));
        assert!(html.contains("A test function"));
    }

    #[test]
    fn test_markdown_renderer() {
        let items = vec![DocItem {
            kind: DocKind::Struct,
            name: "Point".to_string(),
            doc: "A 2D point".to_string(),
            location: None,
            children: Vec::new(),
            attributes: Vec::new(),
        }];

        let renderer = MarkdownRenderer::new();
        let md = renderer.render_program(&items);
        assert!(md.contains("# Point"));
        assert!(md.contains("A 2D point"));
    }

    #[test]
    fn test_escape_latex() {
        assert_eq!(escape_latex("hello & world"), "hello \\& world");
        assert_eq!(escape_latex("100%"), "100\\%");
    }
}
