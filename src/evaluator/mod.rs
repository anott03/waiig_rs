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

fn eval_infix_int_expression(op: String, left: i32, right: i32) -> Object {
    return match op.as_str() {
        "+" => Object::Integer(Integer{ value: left + right }),
        "-" => Object::Integer(Integer{ value: left - right }),
        "*" => Object::Integer(Integer{ value: left * right }),
        "/" => Object::Integer(Integer{ value: left / right }),
        "<" => Object::Boolean(Boolean{ value: left < right }),
        ">" => Object::Boolean(Boolean{ value: left > right }),
        "==" => Object::Boolean(Boolean{ value: left == right }),
        "!=" => Object::Boolean(Boolean{ value: left != right }),
        _ => Object::Null,
    };
}

fn eval_infix_bool_expression(op: String, left: bool, right: bool) -> Object {
    return match op.as_str() {
        "==" => Object::Boolean(Boolean { value: left == right }),
        "!=" => Object::Boolean(Boolean { value: left != right }),
        _ => Object::Null,
    };
}

fn eval_infix_expression(op: String, left: Object, right: Object) -> Object {
    return match op.as_str() {
        "==" => Object::Boolean(Boolean{ value: left == right }),
        "!=" => Object::Boolean(Boolean{ value: left != right }),
        "<" => Object::Boolean(Boolean{ value: left < right }),
        ">" => Object::Boolean(Boolean{ value: left > right }),
        _ => match left {
            Object::Integer(l) => {
                if let Object::Integer(r) = right {
                    eval_infix_int_expression(op, l.value, r.value)
                } else {
                    Object::Null
                }
            },
            _ => Object::Null,
        }
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
        },
        Expression::InfixExpression(ie) => {
            let left = eval_expression(*ie.left);
            let right = eval_expression(*ie.right);
            eval_infix_expression(ie.operator, left, right)
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
