use crate::{common::operator::Operator, frontend::parser::ast::*};

pub fn optimize(mut program: Program) -> Program {
    let mut functions = Vec::new();
    for function in program.functions {
        functions.push(Function {
            name: function.name,
            ret_typ: function.ret_typ,
            body: match opt_statement(function.body) {
                Some(stmt) => stmt,
                None => AstStatement::Block { stmts: Vec::new() },
            },
        });
    }
    program.functions = functions;
    program
}

fn opt_statement(statement: AstStatement) -> Option<AstStatement> {
    match statement {
        AstStatement::Block { stmts } => {
            let mut new_stmts = Vec::new();
            for stmt in stmts {
                if let Some(new_stmt) = opt_statement(stmt) {
                    new_stmts.push(new_stmt);
                }
            }
            Some(AstStatement::Block { stmts: new_stmts })
        }
        AstStatement::Declare { name, typ, value } => Some(AstStatement::Declare {
            name,
            typ,
            value: Box::new(opt_expression(*value)),
        }),
        AstStatement::Assign { name, value } => Some(AstStatement::Assign {
            name,
            value: Box::new(opt_expression(*value)),
        }),
        AstStatement::Return { value } => Some(AstStatement::Return {
            value: Box::new(opt_expression(*value)),
        }),
        AstStatement::If { cond, then, els } => {
            let cond = opt_expression(*cond);
            if let AstExpression::Bool { value } = cond {
                match value {
                    true => opt_statement(*then),
                    false => match els {
                        Some(els) => Some(opt_statement(*els)?),
                        None => None,
                    },
                }
            } else {
                Some(AstStatement::If {
                    cond: Box::new(cond),
                    then,
                    els,
                })
            }
        }
    }
}

fn opt_expression(expression: AstExpression) -> AstExpression {
    match expression {
        AstExpression::Integer { .. } => expression,
        AstExpression::Bool { .. } => expression,
        AstExpression::Ident { .. } => expression,
        AstExpression::BinaryOp { op, lhs, rhs } => {
            let lhs = opt_expression(*lhs);
            let rhs = opt_expression(*rhs);
            match (lhs, rhs) {
                (
                    AstExpression::Integer { value: left_value },
                    AstExpression::Integer { value: right_value },
                ) => {
                    return opt_binop_int(op, left_value, right_value);
                }
                (lhs, rhs) => AstExpression::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            }
        }
    }
}

fn opt_binop_int(op: Operator, left_value: i32, right_value: i32) -> AstExpression {
    match op {
        Operator::Add => AstExpression::Integer {
            value: left_value + right_value,
        },
        Operator::Sub => AstExpression::Integer {
            value: left_value - right_value,
        },
        Operator::Mul => AstExpression::Integer {
            value: left_value * right_value,
        },
        Operator::Div => AstExpression::Integer {
            value: left_value / right_value,
        },
        Operator::And => AstExpression::Integer {
            value: left_value & right_value,
        },
        Operator::Or => AstExpression::Integer {
            value: left_value | right_value,
        },
        Operator::Xor => AstExpression::Integer {
            value: left_value ^ right_value,
        },

        Operator::Equal => AstExpression::Bool {
            value: left_value == right_value,
        },
        Operator::NotEqual => AstExpression::Bool {
            value: left_value != right_value,
        },
        Operator::Lt => AstExpression::Bool {
            value: left_value < right_value,
        },
        Operator::Lte => AstExpression::Bool {
            value: left_value <= right_value,
        },
        Operator::Gt => AstExpression::Bool {
            value: left_value > right_value,
        },
        Operator::Gte => AstExpression::Bool {
            value: left_value >= right_value,
        },
    }
}
