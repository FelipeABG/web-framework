use std::{any::Any, collections::HashMap};

pub struct Session {
    data: HashMap<String, Box<dyn Any>>,
}

impl Session {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add<T: Any>(&mut self, key: String, value: T) {
        self.data.insert(key, Box::new(value));
    }

    fn get<T: Any>(&self, key: &str) -> Option<&T> {
        self.data.get(key)?.downcast_ref()
    }
}

pub struct Sessions {
    sessions: HashMap<usize, Session>,
    counter: usize,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_session(&mut self) -> usize {
        self.sessions.insert(self.counter, Session::new());
        let prev = self.counter.clone();
        self.counter += 1;
        prev
    }

    pub fn get_session(&mut self, key: usize) -> &mut Session {
        self.sessions.get_mut(&key).unwrap()
    }
}
