use crate::parser::Value;

#[derive(PartialEq, Clone, Debug)]
pub struct Table {
    keys: Vec<Value>,
    values: Vec<Value>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn register(&mut self, key: Value, value: Value) {
        self.keys.push(key);
        self.values.push(value);
    }

    pub fn index(&self, key: Value) -> Option<usize> {
        if !self.keys.contains(&key) {
            None
        } else {
            Some(self.keys.iter().position(|k| *k == key).unwrap())
        }
    }

    pub fn update(&mut self, key: Value, value: Value) {
        if self.keys.contains(&key) {
            self.register(key, value);
        } else {
            let index = self.index(key).unwrap();
            self.values[index] = value;
        }
    }
}
