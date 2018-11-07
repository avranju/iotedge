// Copyright (c) Microsoft. All rights reserved.

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Initializer {
    name: Option<String>,
}

impl Initializer {
    pub fn new() -> Initializer {
        Initializer { name: None }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}
