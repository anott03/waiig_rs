mod tests;

use crate::object::*;
use crate::ast::*;

const TRUE: Object = Object::Boolean(Boolean {
    value: true
});
const FALSE: Object = Object::Boolean(Boolean {
    value: false
});

fn eval_program(p: Program) -> Object {
    let mut result: Object = Object::Null;
    p.statements.iter().for_each(|s| {
        result = eval_statement(s.clone());
    });
    return result;
}

fn eval_statement(s: Statement) -> Object {
    return match s {
        Statement::ExpressionStatement(es) => eval_expression(es.expression),
        _ => Object::Null,
    };
}

fn eval_bang(right: Object) -> Object {
    return match right {
        Object::Boolean(b) => if b.value == true { FALSE } else { TRUE },
        Object::Null => TRUE,
        _ => FALSE,
    };
}

fn eval_minus(right: Object) -> Object {
    return match right {
        Object::Integer(i) => Object::Integer(Integer{ value: -1*i.value}),
        _ => Object::Null,
    };
}

fn eval_prefix_expression(op: String, right: Object) -> Object {
    return match op.as_str() {
        "!" => eval_bang(right),
        "-" => eval_minus(right),
        _ => right,
    };
}

fn eval_expression(e: Expression) -> Object {
    return match e {
        Expression::IntegerLiteral(i) => Object::Integer(Integer {
            value: i.value,
        }),
        Expression::Boolean(b) => Object::Boolean(Boolean {
            value: b.value,
        }),
        Expression::PrefixExpression(pe) => {
            let right = eval_expression(*pe.right);
            eval_prefix_expression(pe.operator, right)
        }
        _ => Object::Null,
    };
}

pub fn eval(node: Node) -> Object {
    return match node {
        Node::Program(p) => eval_program(p),
        Node::Statement(s) => eval_statement(s),
        Node::Expression(e) => eval_expression(e),
    };
}
