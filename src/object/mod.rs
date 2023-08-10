#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
    Error(String),
}

impl Object {
    pub fn inspect(&self) -> String {
        return match self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::ReturnValue(r) => r.inspect(),
            Object::Error(e) => format!("Error: {}", e),
            Object::Null => "null".to_string(),
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
    };
}

#[derive(Debug)]
pub struct Environment {
    store: Box<std::collections::HashMap<String, Object>>
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            store: Box::new(std::collections::HashMap::new()),
        };
    }

    pub fn get(&self, name: &String) -> Option<&Object> {
        return self.store.get(name);
    }

    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }
}
