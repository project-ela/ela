use crate::{
    common::operator::{BinaryOperator, UnaryOperator},
    frontend::parser::ast::*,
};

pub fn optimize(mut program: Program) -> Program {
    let mut functions = Vec::new();
    for function in program.functions {
        functions.push(Function {
            name: function.name,
            ret_typ: function.ret_typ,
            body: match opt_statement(function.body) {
                Some(stmt) => stmt,
                None => Statement::new(
                    StatementKind::Block { stmts: Vec::new() },
                    function.pos.clone(),
                ),
            },
            pos: function.pos,
        });
    }
    program.functions = functions;
    program
}

fn opt_statement(statement: Statement) -> Option<Statement> {
    match statement.kind {
        StatementKind::Block { stmts } => {
            let mut new_stmts = Vec::new();
            for stmt in stmts {
                if let Some(new_stmt) = opt_statement(stmt) {
                    new_stmts.push(new_stmt);
                }
            }
            Some(Statement::new(
                StatementKind::Block { stmts: new_stmts },
                statement.pos,
            ))
        }
        StatementKind::Var { name, typ, value } => Some(Statement::new(
            StatementKind::Var {
                name,
                typ,
                value: Box::new(opt_expression(*value)),
            },
            statement.pos,
        )),
        StatementKind::Val { name, typ, value } => Some(Statement::new(
            StatementKind::Val {
                name,
                typ,
                value: Box::new(opt_expression(*value)),
            },
            statement.pos,
        )),
        StatementKind::Assign { name, value } => Some(Statement::new(
            StatementKind::Assign {
                name,
                value: Box::new(opt_expression(*value)),
            },
            statement.pos,
        )),
        StatementKind::Return { value } => Some(Statement::new(
            StatementKind::Return {
                value: match value {
                    Some(value) => Some(Box::new(opt_expression(*value))),
                    None => None,
                },
            },
            statement.pos,
        )),
        StatementKind::If { cond, then, els } => match opt_expression(*cond).kind {
            ExpressionKind::Bool { value } => {
                return match (value, els) {
                    (true, _) => opt_statement(*then),
                    (false, els) => els.and_then(|els| opt_statement(*els)),
                }
            }
            _ => unreachable!(),
        },
        StatementKind::While { cond, body } => Some(Statement::new(
            StatementKind::While {
                cond: Box::new(opt_expression(*cond)),
                body: Box::new(opt_statement(*body)?),
            },
            statement.pos,
        )),
        StatementKind::Call { .. } => Some(statement),
    }
}

fn opt_expression(expression: Expression) -> Expression {
    match expression.kind {
        ExpressionKind::Integer { .. } => expression,
        ExpressionKind::Bool { .. } => expression,
        ExpressionKind::Ident { .. } => expression,
        ExpressionKind::UnaryOp { op, ref expr } => match expr.kind {
            ExpressionKind::Integer { value } => {
                Expression::new(opt_unop_int(op, value), expression.pos)
            }
            ExpressionKind::Bool { value } => {
                Expression::new(opt_unop_bool(op, value), expression.pos)
            }
            _ => expression,
        },
        ExpressionKind::BinaryOp { op, lhs, rhs } => {
            let lhs = opt_expression(*lhs);
            let rhs = opt_expression(*rhs);
            match (&lhs.kind, &rhs.kind) {
                (
                    ExpressionKind::Integer { value: left_value },
                    ExpressionKind::Integer { value: right_value },
                ) => {
                    return Expression::new(
                        opt_binop_int(op, *left_value, *right_value),
                        expression.pos,
                    );
                }
                _ => Expression::new(
                    ExpressionKind::BinaryOp {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                    expression.pos,
                ),
            }
        }
        ExpressionKind::Call { .. } => expression,
    }
}

fn opt_unop_int(op: UnaryOperator, value: i32) -> ExpressionKind {
    match op {
        UnaryOperator::Not => ExpressionKind::Integer { value: !value },
    }
}

fn opt_unop_bool(op: UnaryOperator, value: bool) -> ExpressionKind {
    match op {
        UnaryOperator::Not => ExpressionKind::Bool { value: !value },
    }
}

fn opt_binop_int(op: BinaryOperator, left_value: i32, right_value: i32) -> ExpressionKind {
    match op {
        BinaryOperator::Add => ExpressionKind::Integer {
            value: left_value + right_value,
        },
        BinaryOperator::Sub => ExpressionKind::Integer {
            value: left_value - right_value,
        },
        BinaryOperator::Mul => ExpressionKind::Integer {
            value: left_value * right_value,
        },
        BinaryOperator::Div => ExpressionKind::Integer {
            value: left_value / right_value,
        },
        BinaryOperator::And => ExpressionKind::Integer {
            value: left_value & right_value,
        },
        BinaryOperator::Or => ExpressionKind::Integer {
            value: left_value | right_value,
        },
        BinaryOperator::Xor => ExpressionKind::Integer {
            value: left_value ^ right_value,
        },

        BinaryOperator::Equal => ExpressionKind::Bool {
            value: left_value == right_value,
        },
        BinaryOperator::NotEqual => ExpressionKind::Bool {
            value: left_value != right_value,
        },
        BinaryOperator::Lt => ExpressionKind::Bool {
            value: left_value < right_value,
        },
        BinaryOperator::Lte => ExpressionKind::Bool {
            value: left_value <= right_value,
        },
        BinaryOperator::Gt => ExpressionKind::Bool {
            value: left_value > right_value,
        },
        BinaryOperator::Gte => ExpressionKind::Bool {
            value: left_value >= right_value,
        },
    }
}
