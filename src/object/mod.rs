#[derive(Debug, Clone)]
pub struct Integer {
    value: i32,
}

impl Integer {
    pub fn inspect(&self) -> String {
        return self.value.to_string();
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    value: bool,
}

impl Boolean {
    pub fn inspect(&self) -> String {
        return self.value.to_string();
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null,
}

impl Object {
    fn inspect(&self) -> String {
        return match self {
            Object::Integer(i) => i.inspect(),
            Object::Boolean(b) => b.inspect(),
            Object::Null => "null".to_string(),
        };
    }
}
