use std::{any::Any, collections::HashMap};

pub struct Context {
    globals: HashMap<String, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }

    pub fn add<T: Any>(&mut self, key: String, value: T) {
        todo!()
    }

    pub fn get<T: Any>(&self, key: &str) -> Option<&T> {
        todo!()
    }
}
