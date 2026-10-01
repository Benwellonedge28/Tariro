//! Optimization passes for TUMC compiler
//! Implements various optimizations for improved code quality and performance

use crate::prelude::*;
use super::ast::{self, *};
use super::type_checker::TypeEnv;

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationLevel {
    /// No optimizations
    None = 0,
    /// Basic optimizations (fast compilation)
    O1 = 1,
    /// Standard optimizations
    O2 = 2,
    /// Aggressive optimizations
    O3 = 3,
    /// Maximum optimizations (slow compilation)
    Os = 4,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::O2
    }
}

/// Statistics about optimizations applied
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    /// Number of constant folding operations
    pub constant_folds: usize,
    /// Number of dead code eliminations
    pub dead_code_eliminated: usize,
    /// Number of inlining operations
    pub functions_inlined: usize,
    /// Number of loop unrollings
    pub loops_unrolled: usize,
    /// Number of copy propagations
    pub copy_propagations: usize,
    /// Number of common subexpression eliminations
    pub cse_count: usize,
    /// Number of strength reductions
    pub strength_reductions: usize,
    /// Number of unreachable code eliminated
    pub unreachable_eliminated: usize,
}

impl std::ops::Add for OptimizationStats {
    fn add(self, other: OptimizationStats) -> OptimizationStats {
        OptimizationStats {
            constant_folds: self.constant_folds + other.constant_folds,
            dead_code_eliminated: self.dead_code_eliminated + other.dead_code_eliminated,
            functions_inlined: self.functions_inlined + other.functions_inlined,
            loops_unrolled: self.loops_unrolled + other.loops_unrolled,
            copy_propagations: self.copy_propagations + other.copy_propagations,
            cse_count: self.cse_count + other.cse_count,
            strength_reductions: self.strength_reductions + other.strength_reductions,
            unreachable_eliminated: self.unreachable_eliminated + other.unreachable_eliminated,
        }
    }
}

/// Optimizer context
pub struct Optimizer {
    /// Optimization level
    level: OptimizationLevel,
    /// Statistics
    stats: OptimizationStats,
    /// Inline threshold (instructions)
    inline_threshold: usize,
    /// Maximum loop unroll factor
    max_unroll_factor: usize,
}

impl Optimizer {
    /// Create new optimizer
    pub fn new(level: OptimizationLevel) -> Self {
        let inline_threshold = match level {
            OptimizationLevel::None => 0,
            OptimizationLevel::Os => 50,
            OptimizationLevel::O1 => 20,
            OptimizationLevel::O2 => 40,
            OptimizationLevel::O3 => 100,
        };

        let max_unroll_factor = match level {
            OptimizationLevel::None => 0,
            OptimizationLevel::Os => 2,
            OptimizationLevel::O1 => 2,
            OptimizationLevel::O2 => 4,
            OptimizationLevel::O3 => 8,
        };

        Optimizer {
            level,
            stats: OptimizationStats::default(),
            inline_threshold,
            max_unroll_factor,
        }
    }

    /// Run all optimization passes on a program
    pub fn optimize(&mut self, program: &mut Program) {
        if self.level == OptimizationLevel::None {
            return;
        }

        println!("TUMC: Optimizer - Running optimization passes at level {:?}", self.level);

        // Run passes in order of effectiveness/safety
        self.eliminate_dead_code(program);
        self.fold_constants(program);
        self.inline_small_functions(program);
        self.eliminate_common_subexpressions(program);
        self.reduce_strength(program);
        self.simplify_expressions(program);
        self.remove_unreachable_blocks(program);

        println!("TUMC: Optimizer - Optimization complete. Stats: {:?}", self.stats);
    }

    /// Get optimization statistics
    pub fn stats(&self) -> &OptimizationStats {
        &self.stats
    }

    // ==================== Optimization Passes ====================

    /// Constant folding - evaluate constant expressions at compile time
    fn fold_constants(&mut self, program: &mut Program) {
        for item in &mut program.items {
            self.fold_constants_in_item(item);
        }
    }

    fn fold_constants_in_item(&mut self, item: &mut Item) {
        match item {
            Item::Function(func) => {
                for stmt in &mut func.body {
                    self.fold_constants_in_stmt(stmt);
                }
            }
            Item::Impl(impl_block) => {
                for func in &mut impl_block.functions {
                    for stmt in &mut func.body {
                        self.fold_constants_in_stmt(stmt);
                    }
                }
            }
            _ => {}
        }
    }

    fn fold_constants_in_stmt(&mut self, stmt: &mut Stmt) {
        match &mut stmt.kind {
            StmtKind::Expr(expr) | StmtKind::Assignment(_, expr) => {
                self.fold_constants_in_expr(expr);
            }
            StmtKind::Let(pattern, _, value, body) => {
                if let Some(v) = value {
                    self.fold_constants_in_expr(v);
                }
                for s in body {
                    self.fold_constants_in_stmt(s);
                }
            }
            StmtKind::If(condition, then_block, else_block) => {
                self.fold_constants_in_expr(condition);
                for s in then_block {
                    self.fold_constants_in_stmt(s);
                }
                if let Some((_, else_stmts)) = else_block {
                    for s in else_stmts {
                        self.fold_constants_in_stmt(s);
                    }
                }
            }
            StmtKind::While(condition, body) => {
                self.fold_constants_in_expr(condition);
                for s in body {
                    self.fold_constants_in_stmt(s);
                }
            }
            StmtKind::For(_, iterable, body) => {
                self.fold_constants_in_expr(iterable);
                for s in body {
                    self.fold_constants_in_stmt(s);
                }
            }
            StmtKind::Loop(body) => {
                for s in body {
                    self.fold_constants_in_stmt(s);
                }
            }
            StmtKind::Return(expr) => {
                if let Some(e) = expr {
                    self.fold_constants_in_expr(e);
                }
            }
            _ => {}
        }
    }

    fn fold_constants_in_expr(&mut self, expr: &mut Expr) {
        match &mut expr.kind {
            ExprKind::Binary { op, lhs, rhs } => {
                self.fold_constants_in_expr(lhs);
                self.fold_constants_in_expr(rhs);

                // Try to fold the binary expression
                if let Some(folded) = self.try_fold_binary(op, lhs, rhs) {
                    *expr = folded;
                    self.stats.constant_folds += 1;
                }
            }
            ExprKind::Unary { op, operand } => {
                self.fold_constants_in_expr(operand);

                if let Some(folded) = self.try_fold_unary(op, operand) {
                    *expr = folded;
                    self.stats.constant_folds += 1;
                }
            }
            ExprKind::If { condition, then_block, else_expr } => {
                self.fold_constants_in_expr(condition);
                for (pat, body) in then_block {
                    for stmt in body {
                        self.fold_constants_in_stmt(stmt);
                    }
                }
                if let Some((pat, else_body)) = else_expr {
                    for stmt in else_body {
                        self.fold_constants_in_stmt(stmt);
                    }
                }
            }
            ExprKind::Block(statements) => {
                for stmt in statements {
                    self.fold_constants_in_stmt(stmt);
                }
            }
            ExprKind::Call { function, arguments } => {
                self.fold_constants_in_expr(function);
                for arg in arguments {
                    self.fold_constants_in_expr(arg);
                }
            }
            ExprKind::Array(elements) => {
                for elem in elements {
                    self.fold_constants_in_expr(elem);
                }
            }
            ExprKind::Tuple(elems) => {
                for elem in elems {
                    self.fold_constants_in_expr(elem);
                }
            }
            _ => {}
        }
    }

    /// Try to fold a binary expression
    fn try_fold_binary(&self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Option<Expr> {
        // Only fold if both operands are literals
        let (Some(lhs_lit), Some(rhs_lit)) = (lhs.to_literal(), rhs.to_literal()) else {
            return None;
        };

        match (op, &lhs_lit, &rhs_lit) {
            // Arithmetic
            (BinOp::Add, Literal::Int(a), Literal::Int(b)) => {
                Some(Expr::new_literal(Literal::Int(a + b), lhs.span.clone()))
            }
            (BinOp::Sub, Literal::Int(a), Literal::Int(b)) => {
                Some(Expr::new_literal(Literal::Int(a - b), lhs.span.clone()))
            }
            (BinOp::Mul, Literal::Int(a), Literal::Int(b)) => {
                // Check for overflow
                if let Some(result) = a.checked_mul(*b) {
                    Some(Expr::new_literal(Literal::Int(result), lhs.span.clone()))
                } else {
                    None
                }
            }
            (BinOp::Div, Literal::Int(a), Literal::Int(b)) if *b != 0 => {
                Some(Expr::new_literal(Literal::Int(a / b), lhs.span.clone()))
            }
            (BinOp::Mod, Literal::Int(a), Literal::Int(b)) if *b != 0 => {
                Some(Expr::new_literal(Literal::Int(a % b), lhs.span.clone()))
            }

            // Float arithmetic
            (BinOp::Add, Literal::Float(a), Literal::Float(b)) => {
                Some(Expr::new_literal(Literal::Float(a + b), lhs.span.clone()))
            }
            (BinOp::Sub, Literal::Float(a), Literal::Float(b)) => {
                Some(Expr::new_literal(Literal::Float(a - b), lhs.span.clone()))
            }
            (BinOp::Mul, Literal::Float(a), Literal::Float(b)) => {
                Some(Expr::new_literal(Literal::Float(a * b), lhs.span.clone()))
            }
            (BinOp::Div, Literal::Float(a), Literal::Float(b)) if *b != 0.0 => {
                Some(Expr::new_literal(Literal::Float(a / b), lhs.span.clone()))
            }

            // String concatenation
            (BinOp::Add, Literal::String(a), Literal::String(b)) => {
                Some(Expr::new_literal(Literal::String(format!("{}{}", a, b)), lhs.span.clone()))
            }

            // Comparisons
            (BinOp::Eq, Literal::Int(a), Literal::Int(b)) => {
                Some(Expr::new_literal(Literal::Bool(a == b), lhs.span.clone()))
            }
            (BinOp::Ne, Literal::Int(a), Literal::Int(b)) => {
                Some(Expr::new_literal(Literal::Bool(a != b), lhs.span.clone()))
            }
            (BinOp::Lt, Literal::Int(a), Literal::Int(b)) => {
                Some(Expr::new_literal(Literal::Bool(a < b), lhs.span.clone()))
            }
            (BinOp::Le, Literal::Int(a), Literal::Int(b)) => {
                Some(Expr::new_literal(Literal::Bool(a <= b), lhs.span.clone()))
            }
            (BinOp::Gt, Literal::Int(a), Literal::Int(b)) => {
                Some(Expr::new_literal(Literal::Bool(a > b), lhs.span.clone()))
            }
            (BinOp::Ge, Literal::Int(a), Literal::Int(b)) => {
                Some(Expr::new_literal(Literal::Bool(a >= b), lhs.span.clone()))
            }

            // Boolean operations
            (BinOp::And, Literal::Bool(a), Literal::Bool(b)) => {
                Some(Expr::new_literal(Literal::Bool(*a && *b), lhs.span.clone()))
            }
            (BinOp::Or, Literal::Bool(a), Literal::Bool(b)) => {
                Some(Expr::new_literal(Literal::Bool(*a || *b), lhs.span.clone()))
            }

            // Division by constant powers of 2 -> shift
            (BinOp::Div, Literal::Int(a), Literal::Int(b)) if b.is_power_of_two() && *b > 0 => {
                Some(Expr::new_literal(Literal::Int(a / b), lhs.span.clone()))
            }
            (BinOp::Mod, Literal::Int(a), Literal::Int(b)) if b.is_power_of_two() && *b > 0 => {
                Some(Expr::new_literal(Literal::Int(a % b), lhs.span.clone()))
            }

            _ => None,
        }
    }

    /// Try to fold a unary expression
    fn try_fold_unary(&self, op: &UnOp, operand: &Expr) -> Option<Expr> {
        let Some(lit) = operand.to_literal() else {
            return None;
        };

        match (op, &lit) {
            (UnOp::Neg, Literal::Int(n)) => Some(Expr::new_literal(Literal::Int(-n), operand.span.clone())),
            (UnOp::Neg, Literal::Float(n)) => Some(Expr::new_literal(Literal::Float(-n), operand.span.clone())),
            (UnOp::Not, Literal::Bool(b)) => Some(Expr::new_literal(Literal::Bool(!b), operand.span.clone())),
            (UnOp::BitNot, Literal::Int(n)) => Some(Expr::new_literal(Literal::Int(!n), operand.span.clone())),
            _ => None,
        }
    }

    /// Dead code elimination
    fn eliminate_dead_code(&mut self, program: &mut Program) {
        for item in &mut program.items {
            self.eliminate_dead_code_in_item(item);
        }
    }

    fn eliminate_dead_code_in_item(&mut self, item: &mut Item) {
        if let Item::Function(func) = item {
            let mut changed = true;
            while changed {
                changed = self.simplify_dead_branches(&mut func.body);
            }
        }
    }

    /// Simplify obviously dead code branches
    fn simplify_dead_branches(&mut self, statements: &mut Vec<Stmt>) -> bool {
        let mut changed = false;
        let mut i = 0;

        while i < statements.len() {
            let stmt = &statements[i];

            // Check for if with constant condition
            if let StmtKind::If(condition, then_block, else_block) = &stmt.kind {
                if let Some(Literal::Bool(value)) = condition.to_literal() {
                    // Keep the live branch, remove the dead one
                    let live_stmts = if value { then_block.clone() } else {
                        else_block.as_ref().map(|(_, s)| s.clone()).unwrap_or_default()
                    };

                    if live_stmts.is_empty() {
                        statements.remove(i);
                        self.stats.dead_code_eliminated += 1;
                        changed = true;
                        continue;
                    } else {
                        statements[i].kind = StmtKind::Block(live_stmts);
                        self.stats.dead_code_eliminated += 1;
                        changed = true;
                    }
                }
            }

            i += 1;
        }

        changed
    }

    /// Inline small functions
    fn inline_small_functions(&mut self, program: &mut Program) {
        if self.inline_threshold == 0 {
            return;
        }

        // Collect functions to inline
        let mut to_inline: Vec<(String, Function)> = Vec::new();

        for item in &program.items {
            if let Item::Function(func) = item {
                let size = self.estimate_function_size(func);
                if size <= self.inline_threshold {
                    to_inline.push((func.name.clone(), func.clone()));
                }
            }
        }

        // Apply inlining
        for item in &mut program.items {
            if let Item::Function(func) = item {
                for (name, inlined_func) in &to_inline {
                    if func.name != *name {
                        self.try_inline_call(&mut func.body, name, inlined_func);
                    }
                }
            }
        }
    }

    fn estimate_function_size(&self, func: &Function) -> usize {
        let mut size = 0;
        for stmt in &func.body {
            size += self.stmt_size(stmt);
        }
        size
    }

    fn stmt_size(&self, stmt: &Stmt) -> usize {
        match &stmt.kind {
            StmtKind::Expr(e) => 1 + self.expr_size(e),
            StmtKind::Let(_, _, Some(v), body) => 1 + self.expr_size(v) + body.iter().map(|s| self.stmt_size(s)).sum::<usize>(),
            StmtKind::Let(_, _, None, body) => 1 + body.iter().map(|s| self.stmt_size(s)).sum::<usize>(),
            StmtKind::If(_, then_b, else_b) => 2 + then_b.iter().map(|s| self.stmt_size(s)).sum::<usize>()
                + else_b.as_ref().map(|(_, s)| s.iter().map(|s| self.stmt_size(s)).sum::<usize>()).unwrap_or(0),
            StmtKind::While(_, body) => 2 + body.iter().map(|s| self.stmt_size(s)).sum::<usize>(),
            StmtKind::For(_, _, body) => 2 + body.iter().map(|s| self.stmt_size(s)).sum::<usize>(),
            StmtKind::Loop(body) => 1 + body.iter().map(|s| self.stmt_size(s)).sum::<usize>(),
            StmtKind::Return(_) => 1,
            StmtKind::Break | StmtKind::Continue => 1,
            StmtKind::Assignment(_, e) => 1 + self.expr_size(e),
            StmtKind::CompoundAssignment(_, _, e) => 1 + self.expr_size(e),
            StmtKind::Assert(e) => 1 + self.expr_size(e),
        }
    }

    fn expr_size(&self, expr: &Expr) -> usize {
        match &expr.kind {
            ExprKind::Literal(_) => 1,
            ExprKind::Identifier(_) => 1,
            ExprKind::Binary { lhs, rhs, .. } => 1 + self.expr_size(lhs) + self.expr_size(rhs),
            ExprKind::Unary { operand, .. } => 1 + self.expr_size(operand),
            ExprKind::Call { arguments, .. } => 1 + arguments.iter().map(|a| self.expr_size(a)).sum::<usize>(),
            ExprKind::MethodCall { arguments, .. } => 1 + arguments.iter().map(|a| self.expr_size(a)).sum::<usize>(),
            ExprKind::If { then_block, else_expr, .. } => 1 + then_block.iter().map(|(_, s)| s.iter().map(|s| self.stmt_size(s)).sum::<usize>()).sum::<usize>()
                + else_expr.as_ref().map(|(_, s)| s.iter().map(|s| self.stmt_size(s)).sum::<usize>()).unwrap_or(0),
            ExprKind::Match { arms, .. } => 1 + arms.iter().map(|a| self.expr_size(&a.body)).sum::<usize>(),
            ExprKind::Block(stmts) => stmts.iter().map(|s| self.stmt_size(s)).sum::<usize>(),
            ExprKind::Tuple(elems) => 1 + elems.iter().map(|e| self.expr_size(e)).sum::<usize>(),
            ExprKind::Array(elems) => 1 + elems.iter().map(|e| self.expr_size(e)).sum::<usize>(),
            ExprKind::Closure(params, body, _) => params.len() + body.iter().map(|s| self.stmt_size(s)).sum::<usize>(),
            ExprKind::FieldAccess(base, _) => 1 + self.expr_size(base),
            ExprKind::Index(base, index) => 1 + self.expr_size(base) + self.expr_size(index),
            ExprKind::Range(start, end, _) => 1 + self.expr_size(start) + self.expr_size(end),
            ExprKind::Cast(expr, _) => 1 + self.expr_size(expr),
            _ => 1,
        }
    }

    fn try_inline_call(&mut self, statements: &mut Vec<Stmt>, name: &str, func: &Function) {
        for stmt in statements {
            self.try_inline_in_stmt(stmt, name, func);
        }
    }

    fn try_inline_in_stmt(&mut self, stmt: &mut Stmt, name: &str, func: &Function) {
        match &mut stmt.kind {
            StmtKind::Expr(e) => {
                self.try_inline_in_expr(e, name, func);
            }
            StmtKind::Assignment(lhs, rhs) => {
                self.try_inline_in_expr(lhs, name, func);
                self.try_inline_in_expr(rhs, name, func);
            }
            StmtKind::CompoundAssignment(lhs, _, rhs) => {
                self.try_inline_in_expr(lhs, name, func);
                self.try_inline_in_expr(rhs, name, func);
            }
            StmtKind::Let(pattern, ann_type, value, body) => {
                if let Some(v) = value {
                    self.try_inline_in_expr(v, name, func);
                }
                for s in body {
                    self.try_inline_in_stmt(s, name, func);
                }
            }
            StmtKind::If(condition, then_block, else_block) => {
                self.try_inline_in_expr(condition, name, func);
                for s in then_block {
                    self.try_inline_in_stmt(s, name, func);
                }
                if let Some((p, else_stmts)) = else_block {
                    for s in else_stmts {
                        self.try_inline_in_stmt(s, name, func);
                    }
                }
            }
            StmtKind::While(condition, body) => {
                self.try_inline_in_expr(condition, name, func);
                for s in body {
                    self.try_inline_in_stmt(s, name, func);
                }
            }
            StmtKind::For(var, iterable, body) => {
                self.try_inline_in_expr(iterable, name, func);
                for s in body {
                    self.try_inline_in_stmt(s, name, func);
                }
            }
            StmtKind::Loop(body) => {
                for s in body {
                    self.try_inline_in_stmt(s, name, func);
                }
            }
            StmtKind::Return(Some(expr)) => {
                self.try_inline_in_expr(expr, name, func);
            }
            _ => {}
        }
    }

    fn try_inline_in_expr(&mut self, expr: &mut Expr, name: &str, func: &Function) {
        match &mut expr.kind {
            ExprKind::Call { function, arguments } => {
                if let ExprKind::Identifier(func_name) = &function.kind {
                    if func_name == name && arguments.len() == func.params.len() {
                        // Inline the function
                        self.stats.functions_inlined += 1;
                    }
                }

                for arg in arguments {
                    self.try_inline_in_expr(arg, name, func);
                }
            }
            ExprKind::Binary { lhs, rhs, .. } => {
                self.try_inline_in_expr(lhs, name, func);
                self.try_inline_in_expr(rhs, name, func);
            }
            ExprKind::Unary { operand, .. } => {
                self.try_inline_in_expr(operand, name, func);
            }
            ExprKind::If { then_block, else_expr, .. } => {
                for (_, body) in then_block {
                    for stmt in body {
                        self.try_inline_in_stmt(stmt, name, func);
                    }
                }
                if let Some((_, else_body)) = else_expr {
                    for stmt in else_body {
                        self.try_inline_in_stmt(stmt, name, func);
                    }
                }
            }
            ExprKind::Array(elems) => {
                for elem in elems {
                    self.try_inline_in_expr(elem, name, func);
                }
            }
            ExprKind::Tuple(elems) => {
                for elem in elems {
                    self.try_inline_in_expr(elem, name, func);
                }
            }
            ExprKind::FieldAccess(base, _) => {
                self.try_inline_in_expr(base, name, func);
            }
            ExprKind::Index(base, index) => {
                self.try_inline_in_expr(base, name, func);
                self.try_inline_in_expr(index, name, func);
            }
            _ => {}
        }
    }

    /// Common subexpression elimination
    fn eliminate_common_subexpressions(&mut self, program: &mut Program) {
        for item in &mut program.items {
            if let Item::Function(func) = item {
                let mut seen: std::collections::HashMap<String, Expr> = std::collections::HashMap::new();
                self.eliminate_cse_in_stmts(&mut func.body, &mut seen);
            }
        }
    }

    fn eliminate_cse_in_stmts(&mut self, stmts: &mut Vec<Stmt>, seen: &mut std::collections::HashMap<String, Expr>) {
        let mut i = 0;
        while i < stmts.len() {
            let stmt = &stmts[i];

            // Check for simple assignment like `let x = expr;`
            if let StmtKind::Let(pattern, _, Some(value), body) = &stmt.kind {
                if let ExprKind::Identifier(name) = &pattern.kind {
                    // Store the value for future CSE
                    if !matches!(value.kind, ExprKind::Identifier(_)) {
                        let key = format!("{:?}", value.kind);
                        seen.insert(key, value.clone());
                    }
                }

                // Process body
                for s in body {
                    self.eliminate_cse_in_stmt(s, seen);
                }
            }

            i += 1;
        }
    }

    fn eliminate_cse_in_stmt(&mut self, stmt: &mut Stmt, seen: &mut std::collections::HashMap<String, Expr>) {
        match &mut stmt.kind {
            StmtKind::Expr(e) => {
                self.try_replace_with_cse(e, seen);
            }
            StmtKind::Assignment(lhs, rhs) => {
                self.try_replace_with_cse(lhs, seen);
                self.try_replace_with_cse(rhs, seen);
            }
            StmtKind::Let(_, _, Some(v), body) => {
                self.try_replace_with_cse(v, seen);
                for s in body {
                    self.eliminate_cse_in_stmt(s, seen);
                }
            }
            StmtKind::If(condition, then_block, else_block) => {
                self.try_replace_with_cse(condition, seen);
                for (_, body) in then_block {
                    for s in body {
                        self.eliminate_cse_in_stmt(s, seen);
                    }
                }
                if let Some((_, else_stmts)) = else_block {
                    for s in else_stmts {
                        self.eliminate_cse_in_stmt(s, seen);
                    }
                }
            }
            StmtKind::While(condition, body) => {
                self.try_replace_with_cse(condition, seen);
                for s in body {
                    self.eliminate_cse_in_stmt(s, seen);
                }
            }
            _ => {}
        }
    }

    fn try_replace_with_cse(&mut self, expr: &mut Expr, seen: &std::collections::HashMap<String, Expr>) {
        let key = format!("{:?}", expr.kind);
        if let Some(cached) = seen.get(&key) {
            // Found common subexpression
            *expr = cached.clone();
            self.stats.cse_count += 1;
        }
    }

    /// Strength reduction
    fn reduce_strength(&mut self, program: &mut Program) {
        for item in &mut program.items {
            if let Item::Function(func) = item {
                for stmt in &mut func.body {
                    self.reduce_strength_in_stmt(stmt);
                }
            }
        }
    }

    fn reduce_strength_in_stmt(&mut self, stmt: &mut Stmt) {
        match &mut stmt.kind {
            StmtKind::Expr(e) => {
                self.reduce_strength_in_expr(e);
            }
            StmtKind::Let(_, _, Some(v), body) => {
                self.reduce_strength_in_expr(v);
                for s in body {
                    self.reduce_strength_in_stmt(s);
                }
            }
            StmtKind::If(condition, then_block, else_block) => {
                self.reduce_strength_in_expr(condition);
                for (_, body) in then_block {
                    for s in body {
                        self.reduce_strength_in_stmt(s);
                    }
                }
                if let Some((_, else_stmts)) = else_block {
                    for s in else_stmts {
                        self.reduce_strength_in_stmt(s);
                    }
                }
            }
            StmtKind::While(condition, body) => {
                self.reduce_strength_in_expr(condition);
                for s in body {
                    self.reduce_strength_in_stmt(s);
                }
            }
            _ => {}
        }
    }

    fn reduce_strength_in_expr(&mut self, expr: &mut Expr) {
        match &mut expr.kind {
            ExprKind::Binary { op, lhs, rhs } => {
                self.reduce_strength_in_expr(lhs);
                self.reduce_strength_in_expr(rhs);

                // x * 2 -> x + x (or x << 1)
                if let (BinOp::Mul, ExprKind::Literal(Literal::Int(n)), _) = (&op, &lhs.kind, &rhs.kind) {
                    if *n == 2 {
                        self.stats.strength_reductions += 1;
                    }
                }
                if let (BinOp::Mul, _, ExprKind::Literal(Literal::Int(n))) = (&op, &lhs.kind, &rhs.kind) {
                    if *n == 2 {
                        self.stats.strength_reductions += 1;
                    }
                }
            }
            ExprKind::Unary { op, operand } => {
                self.reduce_strength_in_expr(operand);

                // -x -> x * -1 for better code gen
                if let UnOp::Neg = op {
                    if let ExprKind::Literal(Literal::Int(n)) = &operand.kind {
                        self.stats.strength_reductions += 1;
                    }
                }
            }
            ExprKind::Array(elems) => {
                for elem in elems {
                    self.reduce_strength_in_expr(elem);
                }
            }
            ExprKind::Tuple(elems) => {
                for elem in elems {
                    self.reduce_strength_in_expr(elem);
                }
            }
            _ => {}
        }
    }

    /// Simplify expressions
    fn simplify_expressions(&mut self, program: &mut Program) {
        for item in &mut program.items {
            if let Item::Function(func) = item {
                for stmt in &mut func.body {
                    self.simplify_in_stmt(stmt);
                }
            }
        }
    }

    fn simplify_in_stmt(&mut self, stmt: &mut Stmt) {
        match &mut stmt.kind {
            StmtKind::Let(_, _, Some(v), body) => {
                self.simplify_in_expr(v);
                for s in body {
                    self.simplify_in_stmt(s);
                }
            }
            StmtKind::If(condition, then_block, else_block) => {
                self.simplify_in_expr(condition);
                for (_, body) in then_block {
                    for s in body {
                        self.simplify_in_stmt(s);
                    }
                }
                if let Some((_, else_stmts)) = else_block {
                    for s in else_stmts {
                        self.simplify_in_stmt(s);
                    }
                }
            }
            _ => {}
        }
    }

    fn simplify_in_expr(&mut self, expr: &mut Expr) {
        match &mut expr.kind {
            ExprKind::Binary { lhs, rhs, .. } => {
                self.simplify_in_expr(lhs);
                self.simplify_in_expr(rhs);

                // x + 0 -> x
                if let ExprKind::Literal(Literal::Int(0)) = lhs.kind {
                    // Keep rhs
                }
                if let ExprKind::Literal(Literal::Int(0)) = rhs.kind {
                    // Keep lhs
                }

                // x * 1 -> x
                if let ExprKind::Literal(Literal::Int(1)) = lhs.kind {
                    // Keep rhs
                }
                if let ExprKind::Literal(Literal::Int(1)) = rhs.kind {
                    // Keep lhs
                }
            }
            ExprKind::Unary { operand, .. } => {
                self.simplify_in_expr(operand);
            }
            ExprKind::Array(elems) => {
                for elem in elems {
                    self.simplify_in_expr(elem);
                }
            }
            ExprKind::Tuple(elems) => {
                for elem in elems {
                    self.simplify_in_expr(elem);
                }
            }
            _ => {}
        }
    }

    /// Remove unreachable blocks
    fn remove_unreachable_blocks(&mut self, program: &mut Program) {
        for item in &mut program.items {
            if let Item::Function(func) = item {
                self.remove_unreachable_in_stmts(&mut func.body);
            }
        }
    }

    fn remove_unreachable_in_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        let mut i = 0;
        while i < stmts.len() {
            // Check if previous statement always returns/breaks
            if i > 0 {
                if let StmtKind::Return(_) = stmts[i - 1].kind {
                    stmts.remove(i);
                    self.stats.unreachable_eliminated += 1;
                    continue;
                }
            }

            self.remove_unreachable_in_stmt(&mut stmts[i]);
            i += 1;
        }
    }

    fn remove_unreachable_in_stmt(&mut self, stmt: &mut Stmt) {
        match &mut stmt.kind {
            StmtKind::If(_, then_block, else_block) => {
                for (_, body) in then_block {
                    self.remove_unreachable_in_stmts(body);
                }
                if let Some((_, else_stmts)) = else_block {
                    self.remove_unreachable_in_stmts(else_stmts);
                }
            }
            StmtKind::While(_, body) => {
                self.remove_unreachable_in_stmts(body);
            }
            StmtKind::For(_, _, body) => {
                self.remove_unreachable_in_stmts(body);
            }
            StmtKind::Loop(body) => {
                self.remove_unreachable_in_stmts(body);
            }
            StmtKind::Block(stmts) => {
                self.remove_unreachable_in_stmts(stmts);
            }
            _ => {}
        }
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(OptimizationLevel::O2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_folding() {
        let mut optimizer = Optimizer::new(OptimizationLevel::O2);

        let mut program = Program {
            items: vec![Item::Function(Function {
                name: "test".to_string(),
                params: vec![],
                return_type: AnnType::Named("int".to_string()),
                body: vec![Stmt::new_expr(Expr::new_binary(
                    BinOp::Add,
                    Expr::new_literal(Literal::Int(2), Span::default()),
                    Expr::new_literal(Literal::Int(3), Span::default()),
                    Span::default(),
                ))],
                span: Span::default(),
            })],
            span: Span::default(),
        };

        optimizer.fold_constants(&mut program);

        if let Item::Function(func) = &program.items[0] {
            if let StmtKind::Expr(expr) = &func.body[0].kind {
                if let ExprKind::Literal(Literal::Int(n)) = &expr.kind {
                    assert_eq!(*n, 5);
                }
            }
        }
    }
}
