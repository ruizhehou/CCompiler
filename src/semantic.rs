// Semantic analyzer for C language

use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub var_type: Type,
    pub is_function: bool,
    pub params: Vec<(Type, String)>,
}

pub struct SemanticAnalyzer {
    symbol_table: Vec<HashMap<String, Symbol>>,
    current_function: Option<String>,
    errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: vec![HashMap::new()],
            current_function: None,
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<String>> {
        // First pass: register all functions and global variables
        for func in &program.functions {
            self.add_symbol(Symbol {
                name: func.name.clone(),
                var_type: func.return_type.clone(),
                is_function: true,
                params: func.params.clone(),
            });
        }

        for (var_type, name, _) in &program.global_declarations {
            self.add_symbol(Symbol {
                name: name.clone(),
                var_type: var_type.clone(),
                is_function: false,
                params: Vec::new(),
            });
        }

        // Second pass: analyze function bodies
        for func in &program.functions {
            self.current_function = Some(func.name.clone());
            self.enter_scope();

            // Add parameters to scope
            for (param_type, param_name) in &func.params {
                self.add_symbol(Symbol {
                    name: param_name.clone(),
                    var_type: param_type.clone(),
                    is_function: false,
                    params: Vec::new(),
                });
            }

            self.analyze_statement(&func.body)?;

            self.exit_scope();
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        let scope = self.symbol_table.last_mut().unwrap();
        scope.insert(symbol.name.clone(), symbol);
    }

    fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        for scope in self.symbol_table.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    fn enter_scope(&mut self) {
        self.symbol_table.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.symbol_table.pop();
    }

    fn analyze_statement(&mut self, stmt: &Stmt) -> Result<(), Vec<String>> {
        match stmt {
            Stmt::Expr(expr) => {
                self.analyze_expression(expr)?;
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.analyze_expression(expr)?;
                }
            }
            Stmt::Block(statements) => {
                self.enter_scope();
                for s in statements {
                    self.analyze_statement(s)?;
                }
                self.exit_scope();
            }
            Stmt::If { condition, then_stmt, else_stmt } => {
                self.analyze_expression(condition)?;
                self.analyze_statement(then_stmt)?;
                if let Some(else_s) = else_stmt {
                    self.analyze_statement(else_s)?;
                }
            }
            Stmt::While { condition, body } => {
                self.analyze_expression(condition)?;
                self.analyze_statement(body)?;
            }
            Stmt::DoWhile { body, condition } => {
                self.analyze_statement(body)?;
                self.analyze_expression(condition)?;
            }
            Stmt::For { init, condition, update, body } => {
                if let Some(init_stmt) = init {
                    self.analyze_statement(init_stmt)?;
                }
                if let Some(cond_expr) = condition {
                    self.analyze_expression(cond_expr)?;
                }
                if let Some(update_expr) = update {
                    self.analyze_expression(update_expr)?;
                }
                self.analyze_statement(body)?;
            }
            Stmt::Break | Stmt::Continue => {
                // Check if we're inside a loop
                // Simplified: we don't track loop nesting here
            }
            Stmt::Declaration { var_type, name, init } => {
                if let Some(init_expr) = init {
                    self.analyze_expression(init_expr)?;
                }
                self.add_symbol(Symbol {
                    name: name.clone(),
                    var_type: var_type.clone(),
                    is_function: false,
                    params: Vec::new(),
                });
            }
        }
        Ok(())
    }

    fn analyze_expression(&mut self, expr: &Expr) -> Result<(), Vec<String>> {
        match expr {
            Expr::IntConst(_) => {}
            Expr::CharConst(_) => {}
            Expr::StringConst(_) => {}
            Expr::Identifier(name) => {
                if self.lookup_symbol(name).is_none() {
                    self.errors.push(format!("Undeclared identifier: {}", name));
                }
            }
            Expr::BinaryOp { op: _, left, right } => {
                self.analyze_expression(left)?;
                self.analyze_expression(right)?;
                // Type checking could be added here
            }
            Expr::UnaryOp { op: _, operand } => {
                self.analyze_expression(operand)?;
            }
            Expr::Assignment { left, right } => {
                self.analyze_expression(left)?;
                self.analyze_expression(right)?;
                // Check if left is assignable (lvalue)
                match &**left {
                    Expr::Identifier(_) => {}
                    Expr::UnaryOp { op: UnaryOp::Deref, .. } => {}
                    _ => {
                        self.errors.push("Invalid lvalue in assignment".to_string());
                    }
                }
            }
            Expr::Call { func, args } => {
                self.analyze_expression(func)?;
                for arg in args {
                    self.analyze_expression(arg)?;
                }
                // Check function signature
                if let Expr::Identifier(func_name) = &**func {
                    if let Some(symbol) = self.lookup_symbol(func_name) {
                        if !symbol.is_function {
                            self.errors.push(format!("'{}' is not a function", func_name));
                        }
                    }
                }
            }
            Expr::Cast { target_type: _, expr: inner } => {
                self.analyze_expression(inner)?;
            }
        }
        Ok(())
    }
}

pub fn analyze(program: &Program) -> Result<(), Vec<String>> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(program)
}
