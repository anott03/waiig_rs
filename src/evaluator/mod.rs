mod tests;

use std::sync::{Mutex, Arc};
use crate::object::*;
use crate::ast::*;

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

fn eval_program(p: Program, env: Arc<Environment<'static>>) -> Object<'static> {
    let mut result: Object = Object::Null;
    for i in 0..p.statements.len() {
        result = eval_statement(p.statements[i].clone(), env.clone());
        if let Object::ReturnValue(r) = result{
            return *r;
        } else if let Object::Error(_) = result {
            return result;
        }
    }
    return result;
}

fn eval_block_statement(bs: BlockStatement, env: Arc<Environment<'static>>) -> Object<'static> {
    let mut result: Object = Object::Null;
    for i in 0..bs.statements.len() {
        result = eval_statement(bs.statements[i].clone(), env.clone());
        if let Object::ReturnValue(_) = result {
            break;
        }
        if let Object::Error(_) = result {
            break;
        }
    }
    return result;
}

fn eval_statement(s: Statement, env: Arc<Environment<'static>>) -> Object<'static> {
    return match s {
        Statement::ExpressionStatement(es) => eval_expression(es.expression, env.clone()),
        Statement::ReturnStatement(rs) => {
            let val = eval_expression(rs.return_val, env);
            if let Object::Error(_) = val {
                val
            } else {
                Object::ReturnValue(Box::new(val))
            }
        },
        Statement::LetStatement(ls) => {
            if let Some(value) = ls.value {
                let val = eval_expression(value, env.clone());
                if let Object::Error(_) = val {
                    val
                } else {
                    env.set(ls.name.value.clone(), val.clone());
                    val
                }
            } else {
                new_error!("variable does not have initial value: {}", ls.name.value)
            }
        }
    };
}

fn eval_bang(right: Object) -> Object {
    return match right {
        Object::Boolean(b) => if b == true { Object::Boolean(false) } else { Object::Boolean(true) },
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
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

fn eval_infix_int_expression(op: String, left: i32, right: i32) -> Object<'static> {
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

fn eval_infix_bool_expression(op: String, left: bool, right: bool) -> Object<'static> {
    return match op.as_str() {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    };
}

fn eval_infix_expression(op: String, left: Object<'static>, right: Object<'static>) -> Object<'static> {
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

fn eval_expressions(exps: Vec<Box<Expression>>, env: Arc<Environment<'static>>) -> Vec<Object<'static>> {
    let mut objs: Vec<Object> = Vec::new();
    for i in 0..exps.len() {
        let exp = *exps[i].clone();
        let evaluated = eval_expression(exp, env.clone());
        if let Object::Error(_) = evaluated {
            return vec![evaluated];
        }
        objs.push(evaluated);
    };
    return objs;
}

fn eval_expression(e: Expression, env: Arc<Environment<'static>>) -> Object<'static> {
    return match e {
        Expression::IntegerLiteral(i) => Object::Integer(i.value),
        Expression::Boolean(b) => Object::Boolean(b.value),
        Expression::PrefixExpression(pe) => {
            let right = eval_expression(*pe.right, env);
            if let Object::Error(_) = right {
                right
            } else {
                eval_prefix_expression(pe.operator, right)
            }
        },
        Expression::InfixExpression(ie) => {
            let left = eval_expression(*ie.left, env.clone());
            let right = eval_expression(*ie.right, env.clone());
            if let Object::Error(_) = left {
                left
            } else if let Object::Error(_) = right {
                right
            } else {
                eval_infix_expression(ie.operator, left, right)
            }
        },
        Expression::IfExpression(ie) => {
            let condition = eval_expression(*ie.condition, env.clone());
            if let Object::Error(_) = condition {
                condition
            } else if is_truthy(condition) {
                eval(Node::BlockStatement(ie.consequence), env.clone())
            } else {
                if let Some(alt) = ie.alternative {
                    eval(Node::BlockStatement(alt), env.clone())
                } else {
                    Object::Null
                }
            }
        }
        Expression::Identifier(i) => {
            // if let Some(val) = env.get(&i.value) {
            //     val.clone()
            // } else {
            //     new_error!("unknown identifier: {}", i.value);
            // }
            new_error!("unknown identifier: {}", i.value)
        },
        Expression::FunctionLiteral(fl) => {
            let parameters = fl.parameters;
            let body = fl.body;
            
            Object::Function(Function { parameters, body, env: Some(env) })
        }
        Expression::CallExpression(ce) => {
            let function = eval_expression(*ce.function, env.clone());
            if let Object::Error(_) = function {
                function
            } else {
                let args = eval_expressions(ce.arguments, env.clone());
                if let Object::Error(_) = args[0] {
                    args[0].clone()
                } else {
                    Object::Null
                }
            }
        },
        Expression::Empty => Object::Null,
    };
}

pub fn eval(node: Node, env: Arc<Environment<'static>>) -> Object<'static> {
    return match node {
        Node::Program(p) => eval_program(p, env.clone()),
        Node::Statement(s) => eval_statement(s, env.clone()),
        Node::Expression(e) => eval_expression(e, env.clone()),
        Node::BlockStatement(bs) => eval_block_statement(bs, env.clone()),
    };
}
