mod tests;

use crate::object::*;
use crate::ast::*;

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);

macro_rules! new_error {
    ($($arg:tt)*) => {{
        let str = format!($($arg)*);
        return Object::Error(str.to_string());
    }}
}

fn is_truthy(condition: Object) -> bool {
    return match condition {
        Object::Null => false,
        Object::Boolean(b) => if b { true } else { false },
        Object::Integer(i) => if i == 0 { false } else { true },
        Object::ReturnValue(r) => is_truthy(*r),
        _ => true,
    };
}

fn eval_program(p: Program) -> Object {
    let mut result: Object = Object::Null;
    for i in 0..p.statements.len() {
        result = eval_statement(p.statements[i].clone());
        if let Object::ReturnValue(r) = result{
            return *r;
        } else if let Object::Error(_) = result {
            return result;
        }
    }
    return result;
}

fn eval_block_statement(bs: BlockStatement) -> Object {
    let mut result: Object = Object::Null;
    for i in 0..bs.statements.len() {
        result = eval_statement(bs.statements[i].clone());
        if let Object::ReturnValue(_) = result {
            break;
        }
        if let Object::Error(_) = result {
            break;
        }
    }
    return result;
}

fn eval_statement(s: Statement) -> Object {
    return match s {
        Statement::ExpressionStatement(es) => eval_expression(es.expression),
        Statement::ReturnStatement(rs) => {
            let val = eval_expression(rs.return_val);
            if let Object::Error(_) = val {
                val
            } else {
                Object::ReturnValue(Box::new(val))
            }
        },
        _ => Object::Null,
    };
}

fn eval_bang(right: Object) -> Object {
    return match right {
        Object::Boolean(b) => if b == true { FALSE } else { TRUE },
        Object::Null => TRUE,
        _ => FALSE,
    };
}

fn eval_minus(right: Object) -> Object {
    return match right {
        Object::Integer(i) => Object::Integer(-1*i),
        _ => new_error!("unknown operator: -{}", get_type(&right).as_str()),
    };
}

fn eval_prefix_expression(op: String, right: Object) -> Object {
    return match op.as_str() {
        "!" => eval_bang(right),
        "-" => eval_minus(right),
        _ => new_error!("unknown operator: {}{}", op.as_str(), get_type(&right).as_str()),
    };
}

fn eval_infix_int_expression(op: String, left: i32, right: i32) -> Object {
    return match op.as_str() {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "**" => Object::Integer(i32::pow(left, right.try_into().unwrap())),
        "<" => Object::Boolean(left < right),
        ">" => Object::Boolean(left > right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => new_error!("unknown operator: {} {} {}", left, op, right),
    };
}

fn eval_infix_bool_expression(op: String, left: bool, right: bool) -> Object {
    return match op.as_str() {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    };
}

fn eval_infix_expression(op: String, left: Object, right: Object) -> Object {
    return match op.as_str() {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        "<" => Object::Boolean(left < right),
        ">" => Object::Boolean(left > right),
        _ => match left {
            Object::Integer(l) => {
                if let Object::Integer(r) = right {
                    eval_infix_int_expression(op, l, r)
                } else {
                    new_error!("type mismatch: {} {} {}", get_type(&left), op, get_type(&right))
                }
            },
            _ => new_error!("unknown operator: {} {} {}", get_type(&left), op, get_type(&right))
        }
    };
}

fn eval_expression(e: Expression) -> Object {
    return match e {
        Expression::IntegerLiteral(i) => Object::Integer(i.value),
        Expression::Boolean(b) => Object::Boolean(b.value),
        Expression::PrefixExpression(pe) => {
            let right = eval_expression(*pe.right);
            if let Object::Error(_) = right {
                right
            } else {
                eval_prefix_expression(pe.operator, right)
            }
        },
        Expression::InfixExpression(ie) => {
            let left = eval_expression(*ie.left);
            let right = eval_expression(*ie.right);
            if let Object::Error(_) = left {
                left
            } else if let Object::Error(_) = right {
                right
            } else {
                eval_infix_expression(ie.operator, left, right)
            }
        },
        Expression::IfExpression(ie) => {
            let condition = eval_expression(*ie.condition);
            if let Object::Error(_) = condition {
                condition
            } else if is_truthy(condition) {
                eval(Node::BlockStatement(ie.consequence))
            } else {
                if let Some(alt) = ie.alternative {
                    eval(Node::BlockStatement(alt))
                } else {
                    Object::Null
                }
            }
        }
        _ => Object::Null,
    };
}

pub fn eval(node: Node) -> Object {
    return match node {
        Node::Program(p) => eval_program(p),
        Node::Statement(s) => eval_statement(s),
        Node::Expression(e) => eval_expression(e),
        Node::BlockStatement(bs) => eval_block_statement(bs),
    };
}
