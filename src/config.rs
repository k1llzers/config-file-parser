use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub sections: HashMap<String, Section>,
}

#[derive(Debug, PartialEq)]
pub struct Section {
    pub name: String,
    pub pairs: HashMap<String, String>,
}