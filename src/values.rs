#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Value {
    String(String),
    Int(i32),
    Uint(u32)
}

impl Value {
    pub fn unwrap_int(&self) -> i32 {
        match self {
            Value::Int(n) => *n,
            _ => panic!("int unwrap failed")
        }
    }

    pub fn unwrap_uint(&self) -> u32 {
        match self {
            Value::Uint(n) => *n,
            _ => panic!("uint unwrap failed")
        }
    }

    pub fn unwrap_string(&self) -> String {
        match self {
            Value::String(n) => n.clone(),
            _ => panic!("string unwrap failed")
        }
    }
}