use anyhow::anyhow;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

/// The main INI grammar parser based on Pest grammar definition.
#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIGrammar;

/// Errors that may occur during parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Error triggered when parsing fails due to syntax or format issues.
    #[error("Parsing failed")]
    ParseFailed(#[from] pest::error::Error<Rule>),
}

/// Representation of the parsed configuration file.
#[derive(Debug, PartialEq)]
pub struct Config {
    /// Contains all sections in the INI file, mapped by section name.
    pub sections: HashMap<String, Section>,
}

impl Config {
    /// Retrieves a value by its key across all sections.
    /// If multiple sections contain the same key, returns the first occurrence.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to search for in all sections.
    ///
    /// # Returns
    ///
    /// * `Option<&Value>` - Returns the value if found, or `None` if the key does not exist.
    pub fn get_value(&self, key: &str) -> Option<&Value> {
        for section in self.sections.values() {
            if let Some(value) = section.pairs.get(key) {
                return Some(value);
            }
        }
        None
    }

    /// Retrieves a value by section and key.
    ///
    /// # Arguments
    ///
    /// * `section` - The section name where the key resides.
    /// * `key` - The key to search for within the specified section.
    ///
    /// # Returns
    ///
    /// * `Option<&Value>` - Returns the value if found, or `None` if the key does not exist.
    pub fn get_value_in_section(&self, section: &str, key: &str) -> Option<&Value> {
        self.sections.get(section)?.pairs.get(key)
    }
}

/// Represents a section in the INI file, containing key-value pairs.
#[derive(Debug, PartialEq)]
pub struct Section {
    /// The name of the section.
    pub name: String,
    /// A map of key-value pairs in this section.
    pub pairs: HashMap<String, Value>,
}

/// Represents a values of the section, containing string and array value.
#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Array(Vec<String>),
}

/// Parses the given INI content into a `Config` structure.
///
/// # Arguments
///
/// * `input` - A string slice containing the INI file content.
///
/// # Returns
///
/// * `Result<Config, anyhow::Error>` - Returns `Config` on success, or an error if parsing fails.
pub fn parse_ini(input: &str) -> anyhow::Result<Config> {
    let parsed_file = INIGrammar::parse(Rule::file, input)?
        .next()
        .ok_or_else(|| anyhow!("Can't read ini"))?;

    let mut config = Config {
        sections: HashMap::new(),
    };

    for record in parsed_file.into_inner() {
        match record.as_rule() {
            Rule::section => {
                let mut section_name = String::new();
                let mut pairs = HashMap::new();

                for section_part in record.into_inner() {
                    match section_part.as_rule() {
                        Rule::name => {
                            section_name = section_part.as_str().to_string();
                        }
                        Rule::pair => {
                            let mut pair_parts = section_part.into_inner();
                            let key = pair_parts.next().unwrap().as_str().to_string();
                            let value_part = pair_parts.next().unwrap();

                            let value = match value_part.as_rule() {
                                Rule::value => Value::String(value_part.as_str().to_string()),
                                Rule::array_value => {
                                    let array_items = value_part
                                        .into_inner()
                                        .map(|item| item.as_str().to_string())
                                        .collect::<Vec<_>>();
                                    Value::Array(array_items)
                                }
                                _ => unreachable!(),
                            };
                            pairs.insert(key, value);
                        }
                        Rule::comment => {}
                        _ => unreachable!(),
                    }
                }

                config.sections.insert(
                    section_name.clone(),
                    Section {
                        name: section_name,
                        pairs,
                    },
                );
            }
            Rule::comment => {}
            Rule::EOI => {}
            x => println!("unreached: {:#?}", x),
        }
    }

    Ok(config)
}
