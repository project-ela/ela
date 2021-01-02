use crate::{
    common::operator::{BinaryOperator, UnaryOperator},
    frontend::parser::ast::*,
};

pub fn optimize(mut program: Program) -> Program {
    let mut functions = Vec::new();
    for function in program.functions {
        if function.body.is_none() {
            continue;
        }
        let optimized_body = opt_statement(function.body.unwrap()).unwrap_or(Statement::new(
            StatementKind::Block { stmts: Vec::new() },
            function.pos.clone(),
        ));
        functions.push(Function {
            name: function.name,
            params: function.params,
            ret_typ: function.ret_typ,
            body: Some(optimized_body),
            pos: function.pos,
        });
    }
    program.functions = functions;
    program
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
                pos: _,
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
        ExpressionKind::BinaryOp { op, lhs, rhs } => Expression {
            kind: opt_binop(op, *lhs, *rhs),
            pos: expression.pos,
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
