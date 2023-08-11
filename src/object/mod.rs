use crate::ast::{self, Inspect};

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<ast::Identifier>,
    pub body: ast::BlockStatement,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        return self.parameters.len() == other.parameters.len() && self.body.statements.len() == other.body.statements.len();
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            parameters: self.parameters.clone(),
            body: self.body.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
    Error(String),
    Function(Function),
}

impl Object {
    pub fn inspect(&self) -> String {
        return match self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::ReturnValue(r) => r.inspect(),
            Object::Error(e) => format!("Error: {}", e),
            Object::Null => "null".to_string(),
            Object::Function(f) => {
                let mut params = String::new();
                f.parameters.iter().for_each(|p| {
                    params += p.to_string().as_str();
                    params += ",";
                });

                format!("fn({}) {{{}}}", params, f.body.to_string())
            },
        };
    }
}

pub fn get_type(obj: &Object) -> String {
    return match obj {
        Object::Integer(_) => String::from("INTEGER"),
        Object::Boolean(_) => String::from("BOOLEAN"),
        Object::ReturnValue(_) => String::from("RETURN_VALUE"),
        Object::Error(_) => String::from("ERROR"),
        Object::Null => String::from("NULL"),
        Object::Function(_) => String::from("FUNCTION"),
    };
}

#[derive(Debug)]
pub struct Environment<'a> {
    store: Box<std::collections::HashMap<String, Object>>,
    parent: Option<&'a mut Environment<'a>>,
}

impl <'a>Environment<'a> {
    pub fn new() -> Self {
        return Self {
            store: Box::new(std::collections::HashMap::new()),
            parent: None,
        };
    }

    pub fn get(&self, name: &String) -> Option<&Object> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj);
        } else if let Some(parent) = &self.parent {
            return parent.get(name);
        }
        return None;
    }

    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }

    pub fn new_enclosed(&'a mut self) -> Self {
        return Self {
            store: Box::new(std::collections::HashMap::new()),
            parent: Some(self),
        }
    }
}
