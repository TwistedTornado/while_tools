use crate::ast::{Ast, Value};
use crate::interpreter::state::State;

mod context;
pub mod interpret_error;
pub mod state;

use crate::ast::Value::*;
use crate::interpreter::context::Context;

use crate::interpreter::interpret_error::InterpretError;

/// A tree-walk interpreter. The interpreter doesn't
/// modify the AST.
pub struct Interpreter {
    context: Context,
    ast: Ast,
}

impl Interpreter {
    pub fn new(ast: Ast) -> Self {
        Self {
            context: Context::new(),
            ast,
        }
    }

    pub fn interpret(&mut self) -> Result<State, InterpretError> {
        self.interpret_ast(&self.ast.clone())?;

        Ok(self.context.state.clone())
    }

    fn interpret_ast(&mut self, ast: &Ast) -> Result<Value, InterpretError> {
        match ast {
            Ast::Ass { ident, value } => {
                match value {
                    ref x if !x.is_statement() => match self.interpret_ast(value) {
                        Ok(I32(x)) => self.context.set_variable(ident.clone(), x),
                        _ => return Err(InterpretError(format!("Bad RHS of Assign: {:?}", value))),
                    },

                    ref x if x.is_statement() => self
                        .context
                        .add_definition(ident.to_string(), *value.clone()),

                    _ => panic!("Bad RHS of expression"),
                };
                Ok(Unit)
            }

            Ast::DefinitionRun { ident } => {
                let referenced_ast = self.context.get_definition(&ident).unwrap().clone();
                self.interpret_ast(&referenced_ast)
            }

            Ast::Skip => Ok(Unit),

            Ast::Comp { first, second } => {
                self.interpret_ast(first)?;
                self.interpret_ast(second)?;
                Ok(Unit)
            }

            Ast::If {
                cond,
                true_path,
                false_path,
            } => match self.interpret_ast(cond)? {
                Bool(true) => self.interpret_ast(true_path),
                Bool(false) => self.interpret_ast(false_path),
                I32(_) => Err(InterpretError(
                    "Arithmetic conditional not allowed".to_string(),
                )),
                Unit => Err(InterpretError(
                    "Statement conditional not allowed".to_string(),
                )),
            },

            Ast::While { cond, body } => {
                loop {
                    match self.interpret_ast(cond) {
                        Ok(Bool(true)) => self.interpret_ast(body),
                        Ok(Bool(false)) => break,
                        Ok(_) => Err(InterpretError("Bad conditional".to_string())),
                        err @ Err(_) => err,
                    }?;
                }

                Ok(Unit)
            }

            Ast::True => Ok(Bool(true)),
            Ast::False => Ok(Bool(false)),

            Ast::Not { expr } => {
                let inner = self.interpret_ast(expr);

                match inner {
                    Ok(Bool(b)) => Ok(Bool(!b)),
                    Ok(I32(_)) => Err(InterpretError("Cannot negate arithmetic".to_string())),
                    _ => Err(InterpretError(
                        "Bool did not evaluate correctly".to_string(),
                    )),
                }
            }
            Ast::Eq { left, right } => {
                let left_result = self.interpret_ast(left);
                let right_result = self.interpret_ast(right);

                match (left_result, right_result) {
                    (Ok(I32(l)), Ok(I32(r))) => Ok(Bool(l == r)),

                    (Ok(Bool(l)), Ok(Bool(r))) => Ok(Bool(l == r)),

                    (Ok(I32(_)), Ok(Bool(_))) => {
                        Err(InterpretError("Cannot evaluate Arith = Bool".to_string()))
                    }

                    (Ok(Bool(_)), Ok(I32(_))) => {
                        Err(InterpretError("Cannot evaluate Bool = Arith".to_string()))
                    }

                    _ => Err(InterpretError("Unexpected error".to_string())),
                }
            }

            Ast::LessEq { left, right } => {
                let Ok(I32(left_inner)) = self.interpret_ast(left) else { return Err(InterpretError("LHS is not arithmetic".to_string())) };
                let Ok(I32(right_inner)) = self.interpret_ast(right) else { return Err(InterpretError("RHS is not arithmetic".to_string())) };

                Ok(Bool(left_inner <= right_inner))
            }
            Ast::And { left, right } => {
                let Ok(Bool(left_inner)) = self.interpret_ast(left) else { return Err(InterpretError("LHS is not boolean".to_string())) };
                let Ok(Bool(right_inner)) = self.interpret_ast(right) else { return Err(InterpretError("LHS is not boolean".to_string())) };

                Ok(Bool(left_inner && right_inner))
            }
            Ast::Add { left, right } => {
                let I32(left_inner) = self.interpret_ast(left)? else { return Err(InterpretError("Got boolean, not arithmetic.".to_string())) };
                let I32(right_inner) = self.interpret_ast(right)? else { return Err(InterpretError("Got boolean, not arithmetic.".to_string())) };

                Ok(I32(left_inner + right_inner))
            }
            Ast::Sub { left, right } => {
                let Ok(I32(left_inner)) = self.interpret_ast(left) else { return Err(InterpretError("LHS is not arithmetic".to_string())) };
                let Ok(I32(right_inner)) = self.interpret_ast(right) else { return Err(InterpretError("RHS is not arithmetic".to_string())) };

                Ok(I32(left_inner - right_inner))
            }
            Ast::Mul { left, right } => {
                let Ok(I32(left_inner)) = self.interpret_ast(left) else { return Err(InterpretError("LHS is not arithmetic".to_string())) };
                let Ok(I32(right_inner)) = self.interpret_ast(right) else { return Err(InterpretError("LHS is not arithmetic".to_string())) };

                Ok(I32(left_inner * right_inner))
            }
            Ast::Literal(x) => Ok(I32(*x)),
            Ast::Ident(i) => Ok(I32(self.context.get_variable(&i))),
        }
    }
}
