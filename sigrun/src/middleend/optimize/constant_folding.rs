use crate::{
    common::operator::{BinaryOperator, UnaryOperator},
    frontend::ast::*,
};

pub fn optimize(mut module: Module) -> Module {
    let mut functions = Vec::new();
    for function in module.functions {
        let optimized_body = match function.body {
            Some(body) => Some(opt_statement(body).unwrap_or(Statement::new(
                StatementKind::Block { stmts: Vec::new() },
                function.pos.clone(),
            ))),
            x => x,
        };

        functions.push(Function {
            body: optimized_body,
            ..function
        });
    }
    module.functions = functions;
    module
}

fn opt_statement(statement: Statement) -> Option<Statement> {
    let kind = match statement.kind {
        StatementKind::Block { stmts } => {
            let new_stmts = stmts
                .into_iter()
                .map(opt_statement)
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect();
            StatementKind::Block { stmts: new_stmts }
        }
        StatementKind::Var { name, typ, value } => StatementKind::Var {
            name,
            typ,
            value: value.map(|x| Box::new(opt_expression(*x))),
        },
        StatementKind::Val { name, typ, value } => StatementKind::Val {
            name,
            typ,
            value: value.map(|x| Box::new(opt_expression(*x))),
        },
        StatementKind::Assign { dst, value } => StatementKind::Assign {
            dst,
            value: Box::new(opt_expression(*value)),
        },
        StatementKind::Return { value } => StatementKind::Return {
            value: value.map(|value| Box::new(opt_expression(*value))),
        },
        StatementKind::If { cond, then, els } => match opt_expression(*cond) {
            Expression {
                kind: ExpressionKind::Bool { value },
                ..
            } => {
                return match (value, els) {
                    (true, _) => opt_statement(*then),
                    (false, els) => els.and_then(|els| opt_statement(*els)),
                }
            }
            cond => StatementKind::If {
                cond: Box::new(cond),
                then,
                els,
            },
        },
        StatementKind::While { cond, body } => StatementKind::While {
            cond: Box::new(opt_expression(*cond)),
            body: Box::new(opt_statement(*body)?),
        },
        StatementKind::Call { name, args } => {
            let new_args = args.into_iter().map(opt_expression).collect();
            StatementKind::Call {
                name,
                args: new_args,
            }
        }
    };

    Some(Statement::new(kind, statement.pos))
}

fn opt_expression(expression: Expression) -> Expression {
    match expression.kind {
        ExpressionKind::Char { .. } => expression,
        ExpressionKind::Integer { .. } => expression,
        ExpressionKind::String { .. } => expression,
        ExpressionKind::Bool { .. } => expression,
        ExpressionKind::Ident { .. } => expression,
        ExpressionKind::UnaryOp { op, expr } => Expression {
            kind: opt_unop(op, *expr),
            ..expression
        },
        ExpressionKind::BinaryOp { op, lhs, rhs } => Expression {
            kind: opt_binop(op, *lhs, *rhs),
            ..expression
        },
        ExpressionKind::Call { name, args } => {
            let new_args = args.into_iter().map(opt_expression).collect();
            Expression::new(
                ExpressionKind::Call {
                    name,
                    args: new_args,
                },
                expression.pos,
            )
        }
        ExpressionKind::Index { lhs, index } => Expression::new(
            ExpressionKind::Index {
                lhs,
                index: Box::new(opt_expression(*index)),
            },
            expression.pos,
        ),
    }
}

fn opt_unop(op: UnaryOperator, expr: Expression) -> ExpressionKind {
    let expr = opt_expression(expr);
    match &expr.kind {
        ExpressionKind::Integer { value: expr } => opt_unop_int(op, *expr),
        ExpressionKind::Bool { value: expr } => opt_unop_bool(op, *expr),
        _ => ExpressionKind::UnaryOp {
            op,
            expr: Box::new(expr),
        },
    }
}

fn opt_unop_int(op: UnaryOperator, value: i32) -> ExpressionKind {
    match op {
        UnaryOperator::Not => ExpressionKind::Integer { value: !value },
        _ => panic!(),
    }
}

fn opt_unop_bool(op: UnaryOperator, value: bool) -> ExpressionKind {
    match op {
        UnaryOperator::Not => ExpressionKind::Bool { value: !value },
        _ => panic!(),
    }
}

fn opt_binop(op: BinaryOperator, lhs: Expression, rhs: Expression) -> ExpressionKind {
    let lhs = opt_expression(lhs);
    let rhs = opt_expression(rhs);

    match (&lhs.kind, &rhs.kind) {
        (ExpressionKind::Integer { value: lhs }, ExpressionKind::Integer { value: rhs }) => {
            opt_binop_int(op, *lhs, *rhs)
        }
        _ => ExpressionKind::BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
    }
}

fn opt_binop_int(op: BinaryOperator, lhs: i32, rhs: i32) -> ExpressionKind {
    use BinaryOperator::*;

    macro_rules! binop_int {
        ($op: tt) => (ExpressionKind::Integer { value: lhs $op rhs });
    }

    macro_rules! binop_bool {
        ($op: tt) => (ExpressionKind::Bool { value: lhs $op rhs });
    }

    match op {
        Add => binop_int!(+),
        Sub => binop_int!(-),
        Mul => binop_int!(*),
        Div => binop_int!(/),
        Mod => binop_int!(%),
        And => binop_int!(&),
        Or => binop_int!(|),
        Xor => binop_int!(^),

        Equal => binop_bool!(==),
        NotEqual => binop_bool!(!=),
        Lt => binop_bool!(<),
        Lte => binop_bool!(<=),
        Gt => binop_bool!(>),
        Gte => binop_bool!(>=),
    }
}
