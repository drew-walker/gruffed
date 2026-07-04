use std::path::Path;

use oxc::allocator::Allocator;
use oxc::ast::ast;
use oxc::parser::Parser;
use oxc::span::SourceType as OxcSourceType;

/// An import specifier extracted from a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportSpecifier {
    /// The import specifier string, e.g. "./foo", "react", "@/utils".
    pub specifier: String,
    /// Line number (1-based) where the import appears.
    pub line: u32,
}

/// Extract all import specifiers from a file's source code.
///
/// Handles:
/// - Static imports: `import x from "./foo"`
/// - Dynamic imports: `import("./bar")` (string-literal args only)
/// - Re-exports: `export { x } from "./baz"`
/// - `export * from "./baz"`
/// - `require()` calls (for CJS)
pub fn extract_imports(source_text: &str, file_path: &Path) -> Vec<ImportSpecifier> {
    // Determine SourceType from file extension
    let source_type = match file_path.extension().and_then(|e| e.to_str()) {
        Some("ts") => OxcSourceType::ts(),
        Some("tsx") => OxcSourceType::tsx(),
        Some("jsx") => OxcSourceType::jsx(),
        Some("cjs") => OxcSourceType::cjs(),
        _ => OxcSourceType::mjs(),
    };

    let allocator = Allocator::default();
    let parser_return = Parser::new(&allocator, source_text, source_type).parse();
    if parser_return.panicked {
        return vec![];
    }
    let program = parser_return.program;

    let mut imports = Vec::new();

    // Walk the AST and extract import specifiers
    for statement in &program.body {
        match statement {
            // import x from "./foo"
            ast::Statement::ImportDeclaration(decl) => {
                let spec = decl.source.value.as_str();
                let line = line_number(source_text, decl.source.span.start);
                imports.push(ImportSpecifier {
                    specifier: spec.to_string(),
                    line,
                });
            }
            // export { x } from "./baz"
            ast::Statement::ExportNamedDeclaration(decl) => {
                if let Some(source) = &decl.source {
                    let spec = source.value.as_str();
                    let line = line_number(source_text, source.span.start);
                    imports.push(ImportSpecifier {
                        specifier: spec.to_string(),
                        line,
                    });
                }
            }
            // export * from "./baz"
            ast::Statement::ExportAllDeclaration(decl) => {
                let spec = decl.source.value.as_str();
                let line = line_number(source_text, decl.source.span.start);
                imports.push(ImportSpecifier {
                    specifier: spec.to_string(),
                    line,
                });
            }
            // Dynamic import("./bar") and require("./bar")
            ast::Statement::ExpressionStatement(expr) => {
                extract_dynamic_imports(&expr.expression, source_text, &mut imports);
            }
            // `const mod = await import("./bar")` and similar
            ast::Statement::VariableDeclaration(decl) => {
                for declarator in &decl.declarations {
                    if let Some(init) = &declarator.init {
                        extract_dynamic_imports(init, source_text, &mut imports);
                    }
                }
            }
            _ => {}
        }
    }

    imports
}

fn line_number(source: &str, byte_offset: u32) -> u32 {
    let end = byte_offset.min(source.len() as u32) as usize;
    source[..end].lines().count() as u32
}

fn extract_dynamic_imports(
    expr: &ast::Expression,
    source_text: &str,
    imports: &mut Vec<ImportSpecifier>,
) {
    match expr {
        // import("./bar") — a top-level expression variant, not a call
        ast::Expression::ImportExpression(import_expr) => {
            if let ast::Expression::StringLiteral(lit) = &import_expr.source {
                imports.push(ImportSpecifier {
                    specifier: lit.value.as_str().to_string(),
                    line: line_number(source_text, lit.span.start),
                });
            }
        }
        // await import("./bar")
        ast::Expression::AwaitExpression(await_expr) => {
            extract_dynamic_imports(&await_expr.argument, source_text, imports);
        }
        // require("./bar")
        ast::Expression::CallExpression(call) => {
            handle_call_expression(call, source_text, imports);
        }
        // Recurse into common wrapper expressions
        ast::Expression::ParenthesizedExpression(paren) => {
            extract_dynamic_imports(&paren.expression, source_text, imports);
        }
        ast::Expression::ConditionalExpression(cond) => {
            extract_dynamic_imports(&cond.consequent, source_text, imports);
            extract_dynamic_imports(&cond.alternate, source_text, imports);
        }
        ast::Expression::AssignmentExpression(assign) => {
            extract_dynamic_imports(&assign.right, source_text, imports);
        }
        ast::Expression::SequenceExpression(seq) => {
            for inner in &seq.expressions {
                extract_dynamic_imports(inner, source_text, imports);
            }
        }
        _ => {}
    }
}

fn handle_call_expression(
    call: &ast::CallExpression,
    source_text: &str,
    imports: &mut Vec<ImportSpecifier>,
) {
    // require("./bar")
    if let ast::Expression::Identifier(ident) = &call.callee {
        if ident.name == "require" {
            if let Some(ast::Argument::StringLiteral(lit)) = call.arguments.first() {
                imports.push(ImportSpecifier {
                    specifier: lit.value.as_str().to_string(),
                    line: line_number(source_text, lit.span.start),
                });
            }
        }
    }
    // Recurse into arguments and callee for nested cases
    for arg in &call.arguments {
        extract_dynamic_imports_from_argument(arg, source_text, imports);
    }
    extract_dynamic_imports(&call.callee, source_text, imports);
}

fn extract_dynamic_imports_from_argument(
    arg: &ast::Argument,
    source_text: &str,
    imports: &mut Vec<ImportSpecifier>,
) {
    match arg {
        ast::Argument::SpreadElement(_) => {}
        ast::Argument::ImportExpression(import_expr) => {
            if let ast::Expression::StringLiteral(lit) = &import_expr.source {
                imports.push(ImportSpecifier {
                    specifier: lit.value.as_str().to_string(),
                    line: line_number(source_text, lit.span.start),
                });
            }
        }
        ast::Argument::AwaitExpression(await_expr) => {
            extract_dynamic_imports(&await_expr.argument, source_text, imports);
        }
        ast::Argument::CallExpression(call) => {
            handle_call_expression(call, source_text, imports);
        }
        ast::Argument::ParenthesizedExpression(paren) => {
            extract_dynamic_imports(&paren.expression, source_text, imports);
        }
        ast::Argument::ConditionalExpression(cond) => {
            extract_dynamic_imports(&cond.consequent, source_text, imports);
            extract_dynamic_imports(&cond.alternate, source_text, imports);
        }
        ast::Argument::AssignmentExpression(assign) => {
            extract_dynamic_imports(&assign.right, source_text, imports);
        }
        ast::Argument::SequenceExpression(seq) => {
            for inner in &seq.expressions {
                extract_dynamic_imports(inner, source_text, imports);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn parse_imports(source: &str) -> Vec<ImportSpecifier> {
        extract_imports(source, &PathBuf::from("test.ts"))
    }

    #[test]
    fn extracts_static_import() {
        let imports = parse_imports(r#"import { foo } from "./bar";"#);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].specifier, "./bar");
    }

    #[test]
    fn extracts_default_import() {
        let imports = parse_imports(r#"import foo from "./bar";"#);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].specifier, "./bar");
    }

    #[test]
    fn extracts_re_export() {
        let imports = parse_imports(r#"export { foo } from "./bar";"#);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].specifier, "./bar");
    }

    #[test]
    fn extracts_export_all() {
        let imports = parse_imports(r#"export * from "./bar";"#);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].specifier, "./bar");
    }

    #[test]
    fn extracts_dynamic_import() {
        let imports = parse_imports(r#"const mod = await import("./bar");"#);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].specifier, "./bar");
    }

    #[test]
    fn extracts_bare_dynamic_import() {
        let imports = parse_imports(r#"import("./bar");"#);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].specifier, "./bar");
    }

    #[test]
    fn extracts_require_call() {
        let imports = parse_imports(r#"const mod = require("./bar");"#);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].specifier, "./bar");
    }

    #[test]
    fn extracts_multiple_imports() {
        let imports = parse_imports(
            r#"
            import { a } from "./a";
            import b from "./b";
            export { c } from "./c";
            "#,
        );
        assert_eq!(imports.len(), 3);
    }

    #[test]
    fn line_numbers_are_tracked() {
        let imports = parse_imports(
            r#"
            const x = 1;
            import { foo } from "./bar";
            "#,
        );
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].line, 3);
    }
}
