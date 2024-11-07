mod config;

use anyhow::anyhow;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIGrammar;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing failed")]
    ParseFailed(#[from] pest::error::Error<Rule>),
}

// TODO: refactor to return Config Structure
pub fn parse_ini(input: &str) -> anyhow::Result< () > {
    let parsed_file = INIGrammar::parse(Rule::file, input)?.next().ok_or_else(|| anyhow!( "Can't read ini" ))?;
    dbg!(parsed_file);
    Ok(())
}