use crate::parser::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Null,
    Integer(i128),
    Float(f64),
    String(String),
    Boolean(bool),
    NativeFunction {
        args: Vec<RuntimeValue>,
    },
    Array(Vec<RuntimeValue>),
    Iterable(Vec<Node>),
    Function {
        args: Vec<RuntimeValue>,
        body: Vec<Node>,
    },
}

fn declare(
    hashmap: &mut HashMap<String, RuntimeValue>,
    name: String,
    value: RuntimeValue,
) -> Result<(), String> {
    if hashmap.contains_key(&name) {
        Err(format!("Variable '{}' already declared", name))
    } else {
        hashmap.insert(name, value);
        Ok(())
    }
}

fn assign(
    hashmap: &mut HashMap<String, RuntimeValue>,
    name: String,
    value: RuntimeValue,
) -> Result<(), String> {
    if hashmap.contains_key(&name) {
        hashmap.insert(name, value);
        Ok(())
    } else {
        Err(format!("Variable '{}' does not exist.", name))
    }
}

fn lookup(hashmap: &mut HashMap<String, RuntimeValue>, name: String) -> Option<RuntimeValue> {
    hashmap.get(&name).cloned()
}

pub fn evaluate(
    node: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, String> {
    match node {
        Node::Scope { body: statements } => {
            let mut result = RuntimeValue::Null;
            for statement in statements {
                result = evaluate(statement, env)?;
            }

            Ok(result)
        }
        Node::IntegerLiteral(i) => Ok(RuntimeValue::Integer(i)),
        Node::StringLiteral(s) => Ok(RuntimeValue::String(s)),
        Node::FloatLiteral(f) => Ok(RuntimeValue::Float(f)),
        Node::Identifier(name) => evaluate_identifier(name, env),
        Node::BinaryExpression {
            left,
            operand,
            right,
        } => evaluate_binary_expression(*left, operand, *right, env),
        Node::AssignmentExpression { name, value } => {
            evaluate_assignment_expression(*name, *value, env)
        }
        Node::VariableDeclaration { name, value } => {
            evaluate_variable_declaration(*name, *value, env)
        }
    }
}

fn evaluate_identifier(
    name: String,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, String> {
    let result = lookup(env, name.clone());
    match result {
        Some(value) => Ok(value.clone()),
        None => Err(format!("Variable '{}' does not exist", name)),
    }
}

fn evaluate_variable_declaration(
    name: Node,
    value: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, String> {
    if let Node::Identifier(name) = name {
        let value = evaluate(value, env)?;
        let res = declare(env, name, value.clone());
        match res {
            Err(e) => Err(e),
            Ok(_) => Ok(value),
        }
    } else {
        Err(format!("Expected a string value"))
    }
}

fn evaluate_assignment_expression(
    name: Node,
    value: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, String> {
    if let Node::Identifier(name) = name {
        let value = evaluate(value, env)?;
        let res = assign(env, name, value.clone());
        match res {
            Err(e) => Err(e),
            Ok(_) => Ok(value),
        }
    } else {
        Err(format!("Expected a string value, found '{:?}'", name))
    }
}

fn evaluate_binary_expression(
    left: Node,
    operand: char,
    right: Node,
    environment: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, String> {
    let left = evaluate(left, environment)?;
    let right = evaluate(right, environment)?;

    match operand {
        '+' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l + r))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(l as f64 + r))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l + r as f64))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l + r)),
            _ => Err(format!(
                "Incompatible types: '{:?}' and '{:?}'",
                left, right
            )),
        },
        '-' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l - r))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(l as f64 - r))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l - r as f64))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l - r)),
            _ => Err(format!(
                "Incompatible types: '{:?}' and '{:?}'",
                left, right
            )),
        },
        '*' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l * r))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(l as f64 * r))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l * r as f64))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l * r)),
            _ => Err(format!(
                "Incompatible types: '{:?}' and '{:?}'",
                left, right
            )),
        },
        '/' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l as f64 / r as f64))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(l as f64 / r))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l / r as f64))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l / r)),
            _ => Err(format!(
                "Incompatible types: '{:?}' and '{:?}'",
                left, right
            )),
        },
        '%' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l % r))
            }
            _ => Err(format!(
                "Incompatible types: '{:?}' and '{:?}'",
                left, right
            )),
        },
        '^' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l.pow(r.try_into().unwrap())))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float((l as f64).powf(r)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l.powf(r as f64)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l.powf(r))),
            _ => Err(format!(
                "Incompatible types: '{:?}' and '{:?}'",
                left, right
            )),
        },
        _ => Ok(RuntimeValue::Null),
    }
}
