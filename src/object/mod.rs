use std::sync::{Mutex, Arc};

use crate::ast::{self, Inspect};

#[derive(Debug)]
pub struct Function<'a> {
    pub parameters: Vec<ast::Identifier>,
    pub body: ast::BlockStatement,
    pub env: Option<Arc<Mutex<Environment<'a>>>>,
}

impl PartialEq for Function<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.parameters.len() == other.parameters.len() && self.body.statements.len() == other.body.statements.len();
    }
}

impl PartialOrd for Function<'_> {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Clone for Function<'_> {
    fn clone(&self) -> Self {
        Self {
            parameters: self.parameters.clone(),
            body: self.body.clone(),
            env: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object<'a> {
    Integer(i32),
    Boolean(bool),
    String(String),
    ReturnValue(Box<Object<'a>>),
    Null,
    Error(String),
    Function(Function<'a>),
}

impl Object<'_> {
    pub fn inspect(&self) -> String {
        return match self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::String(s) => s.to_string(),
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
        Object::String(_) => String::from("STRING"),
        Object::ReturnValue(_) => String::from("RETURN_VALUE"),
        Object::Error(_) => String::from("ERROR"),
        Object::Null => String::from("NULL"),
        Object::Function(_) => String::from("FUNCTION"),
    };
}

#[derive(Debug)]
pub struct Environment<'a> {
    store: Box<std::collections::HashMap<String, Object<'a>>>,
    parent: Option<Arc<Mutex<Environment<'a>>>>,
}

impl <'a>Environment<'a> {
    pub fn new() -> Self {
        return Self {
            store: Box::new(std::collections::HashMap::new()),
            parent: None,
        };
    }

    pub fn get<'b>(&'b self, name: &String) -> Option<Object<'a>> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone());
        } else if let Some(parent) = &self.parent {
            let p = parent.try_lock().expect("error locking parent");
            return p.get(name);
        }
        return None;
    }

    pub fn set<'b>(&'b mut self, name: String, val: Object<'a>) {
        self.store.insert(name, val);
    }
}

pub fn new_enclosed_env(parent: Arc<Mutex<Environment>>) -> Environment {
    return Environment {
        store: Box::new(std::collections::HashMap::new()),
        parent: Some(parent),
    }
}
